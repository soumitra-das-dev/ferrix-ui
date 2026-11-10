//! Append-only write-ahead log (WAL) journal with CRC32 integrity verification.

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const JOURNAL_MAGIC: [u8; 4] = [0x46, 0x58, 0x57, 0x4C]; // "FXWL"
pub const OP_SET: u8 = 1;
pub const OP_DELETE: u8 = 2;

#[derive(Error, Debug)]
pub enum JournalError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Corrupt journal header")]
    CorruptHeader,
    #[error("CRC32 checksum mismatch")]
    ChecksumMismatch,
    #[error("Invalid UTF-8 key")]
    InvalidUtf8,
}

/// Standard IEEE 802.3 CRC32 implementation.
pub fn calculate_crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xED88_8320 & mask);
        }
    }
    !crc
}

pub struct Journal {
    path: PathBuf,
    file: File,
}

impl Journal {
    /// Opens or creates an append-only journal file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<(Self, HashMap<String, Vec<u8>>), JournalError> {
        let path_buf = path.as_ref().to_path_buf();
        if let Some(parent) = path_buf.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path_buf)?;

        let mut entries = HashMap::new();
        let file_len = file.metadata()?.len();

        if file_len > 0 {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            let mut offset = 0;

            while offset + 15 <= buf.len() {
                // Check magic
                if buf[offset..offset + 4] != JOURNAL_MAGIC {
                    break;
                }
                let expected_crc =
                    u32::from_le_bytes(buf[offset + 4..offset + 8].try_into().unwrap());
                let op = buf[offset + 8];
                let key_len =
                    u16::from_le_bytes(buf[offset + 9..offset + 11].try_into().unwrap()) as usize;

                let header_len = 11;
                let val_len_offset = offset + header_len + key_len;

                if val_len_offset + 4 > buf.len() {
                    break; // Partial record at EOF
                }

                let val_len =
                    u32::from_le_bytes(buf[val_len_offset..val_len_offset + 4].try_into().unwrap())
                        as usize;
                let record_len = header_len + key_len + 4 + val_len;

                if offset + record_len > buf.len() {
                    break; // Partial record at EOF
                }

                // Verify CRC32 over payload [OP..end]
                let payload = &buf[offset + 8..offset + record_len];
                let actual_crc = calculate_crc32(payload);

                if actual_crc != expected_crc {
                    break; // Corrupted record at EOF
                }

                let key_bytes = &buf[offset + 11..offset + 11 + key_len];
                let key = match std::str::from_utf8(key_bytes) {
                    Ok(k) => k.to_string(),
                    Err(_) => break,
                };

                let val_start = val_len_offset + 4;
                let val_bytes = buf[val_start..val_start + val_len].to_vec();

                if op == OP_SET {
                    entries.insert(key, val_bytes);
                } else if op == OP_DELETE {
                    entries.remove(&key);
                }

                offset += record_len;
            }

            // Seek file pointer to last valid record for appending
            file.seek(SeekFrom::Start(offset as u64))?;
        }

        Ok((
            Self {
                path: path_buf,
                file,
            },
            entries,
        ))
    }

    /// Appends a SET entry to the journal with CRC32.
    pub fn append_set(&mut self, key: &str, val: &[u8]) -> Result<(), JournalError> {
        self.append_record(OP_SET, key, val)
    }

    /// Appends a DELETE entry to the journal with CRC32.
    pub fn append_delete(&mut self, key: &str) -> Result<(), JournalError> {
        self.append_record(OP_DELETE, key, &[])
    }

    fn append_record(&mut self, op: u8, key: &str, val: &[u8]) -> Result<(), JournalError> {
        let key_bytes = key.as_bytes();
        let mut payload = Vec::with_capacity(1 + 2 + key_bytes.len() + 4 + val.len());

        payload.push(op);
        payload.extend_from_slice(&(key_bytes.len() as u16).to_le_bytes());
        payload.extend_from_slice(key_bytes);
        payload.extend_from_slice(&(val.len() as u32).to_le_bytes());
        payload.extend_from_slice(val);

        let crc = calculate_crc32(&payload);

        let mut record = Vec::with_capacity(8 + payload.len());
        record.extend_from_slice(&JOURNAL_MAGIC);
        record.extend_from_slice(&crc.to_le_bytes());
        record.extend_from_slice(&payload);

        self.file.write_all(&record)?;
        self.file.flush()?;
        Ok(())
    }

    /// Atomically compacts the journal with current live state.
    pub fn compact(&mut self, live_entries: &HashMap<String, Vec<u8>>) -> Result<(), JournalError> {
        let compact_path = self.path.with_extension("compact");
        {
            let mut compact_journal = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&compact_path)?;

            for (k, v) in live_entries {
                let key_bytes = k.as_bytes();
                let mut payload = Vec::with_capacity(1 + 2 + key_bytes.len() + 4 + v.len());
                payload.push(OP_SET);
                payload.extend_from_slice(&(key_bytes.len() as u16).to_le_bytes());
                payload.extend_from_slice(key_bytes);
                payload.extend_from_slice(&(v.len() as u32).to_le_bytes());
                payload.extend_from_slice(v);

                let crc = calculate_crc32(&payload);

                compact_journal.write_all(&JOURNAL_MAGIC)?;
                compact_journal.write_all(&crc.to_le_bytes())?;
                compact_journal.write_all(&payload)?;
            }
            compact_journal.flush()?;
        }

        std::fs::rename(&compact_path, &self.path)?;
        self.file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.path)?;

        Ok(())
    }
}
