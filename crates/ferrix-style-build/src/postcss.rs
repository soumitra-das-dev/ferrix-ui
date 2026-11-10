//! PostCSS subprocess runner and SourceMap line mapping.
//!
//! When `postcss.config.*` is present in a project, this module coordinates running PostCSS
//! through a Node.js subprocess (`tools/postcss-runner/runner.js`) at build-time.
//! In production APKs, zero Node or PostCSS code ships.

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::diagnostics::StyleDiagnostic;

/// Result returned from PostCSS execution.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PostCssOutput {
    pub success: bool,
    #[serde(default)]
    pub css: String,
    #[serde(default)]
    pub map: Option<String>,
    #[serde(default)]
    pub warnings: Vec<PostCssWarning>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub line: Option<u32>,
    #[serde(default)]
    pub column: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostCssWarning {
    pub text: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

/// Original source location resolved from a sourcemap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalLocation {
    pub source: String,
    pub line: usize,
    pub col: usize,
}

/// Parsed v3 Sourcemap supporting line and column lookups.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    pub sources: Vec<String>,
    /// Map of `generated_line` (1-based) to sorted segments `(gen_col, src_idx, orig_line, orig_col)`.
    lines: HashMap<usize, Vec<(usize, usize, usize, usize)>>,
}

impl SourceMap {
    /// Parses a JSON v3 sourcemap string.
    pub fn parse(json_str: &str) -> Option<Self> {
        #[derive(Deserialize)]
        struct RawSourceMap {
            #[serde(default)]
            sources: Vec<String>,
            #[serde(default)]
            mappings: String,
        }

        let raw: RawSourceMap = serde_json::from_str(json_str).ok()?;
        let mut map = SourceMap {
            sources: raw.sources,
            lines: HashMap::new(),
        };

        let mut src_idx = 0i32;
        let mut orig_line = 0i32;
        let mut orig_col = 0i32;

        let lines: Vec<&str> = raw.mappings.split(';').collect();
        for (line_idx, line_str) in lines.into_iter().enumerate() {
            let gen_line = line_idx + 1;
            if line_str.is_empty() {
                continue;
            }

            let mut gen_col = 0i32;
            let mut line_segments = Vec::new();

            for segment_str in line_str.split(',') {
                if segment_str.is_empty() {
                    continue;
                }
                let bytes = segment_str.as_bytes();
                let mut idx = 0;

                let col_delta = decode_vlq(bytes, &mut idx)?;
                gen_col += col_delta;

                if idx < bytes.len() {
                    let src_delta = decode_vlq(bytes, &mut idx)?;
                    let line_delta = decode_vlq(bytes, &mut idx)?;
                    let col_delta2 = decode_vlq(bytes, &mut idx)?;

                    src_idx += src_delta;
                    orig_line += line_delta;
                    orig_col += col_delta2;

                    if src_idx >= 0 && orig_line >= 0 && orig_col >= 0 {
                        line_segments.push((
                            gen_col.max(0) as usize,
                            src_idx as usize,
                            (orig_line + 1) as usize,
                            (orig_col + 1) as usize,
                        ));
                    }
                }
            }

            if !line_segments.is_empty() {
                map.lines.insert(gen_line, line_segments);
            }
        }

        Some(map)
    }

    /// Maps generated line and column back to original file, line, and column.
    pub fn lookup(
        &self,
        generated_line: usize,
        generated_column: usize,
    ) -> Option<OriginalLocation> {
        let segments = self.lines.get(&generated_line)?;
        if segments.is_empty() {
            return None;
        }

        // Find segment with largest gen_col <= generated_column
        let mut best = &segments[0];
        for seg in segments {
            if seg.0 <= generated_column {
                best = seg;
            } else {
                break;
            }
        }

        let source = self.sources.get(best.1)?.clone();
        Some(OriginalLocation {
            source,
            line: best.2,
            col: best.3,
        })
    }
}

/// Helper decoding a Base64 character into 6-bit integer.
fn decode_base64_char(c: u8) -> Option<i32> {
    match c {
        b'A'..=b'Z' => Some((c - b'A') as i32),
        b'a'..=b'z' => Some((c - b'a' + 26) as i32),
        b'0'..=b'9' => Some((c - b'0' + 52) as i32),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}

/// Helper decoding a variable length quantity (VLQ) segment.
fn decode_vlq(bytes: &[u8], idx: &mut usize) -> Option<i32> {
    let mut result = 0i32;
    let mut shift = 0;
    while *idx < bytes.len() {
        let b = decode_base64_char(bytes[*idx])?;
        *idx += 1;
        let has_continuation = (b & 32) != 0;
        let digit = b & 31;
        result |= digit << shift;
        shift += 5;
        if !has_continuation {
            let is_negative = (result & 1) != 0;
            let val = result >> 1;
            return Some(if is_negative { -val } else { val });
        }
    }
    None
}

/// Runner for the PostCSS build stage.
#[derive(Clone, Debug, Default)]
pub struct PostCssRunner;

impl PostCssRunner {
    /// Detects if a PostCSS configuration file exists in the given directory or any parent.
    pub fn detect_config(start_dir: &Path) -> Option<PathBuf> {
        let config_names = [
            "postcss.config.js",
            "postcss.config.cjs",
            "postcss.config.mjs",
            "postcss.config.json",
        ];

        let mut cur = start_dir.to_path_buf();
        loop {
            for name in &config_names {
                let candidate = cur.join(name);
                if candidate.exists() {
                    return Some(candidate);
                }
            }
            if !cur.pop() {
                break;
            }
        }
        None
    }

    /// Locates the `runner.js` script.
    pub fn find_runner_script() -> Option<PathBuf> {
        if let Ok(p) = std::env::var("FERRIX_POSTCSS_RUNNER") {
            let path = PathBuf::from(p);
            if path.exists() {
                return Some(path);
            }
        }

        if let Ok(mut cur) = std::env::current_dir() {
            loop {
                let candidate = cur.join("tools/postcss-runner/runner.js");
                if candidate.exists() {
                    return Some(candidate);
                }
                if !cur.pop() {
                    break;
                }
            }
        }

        None
    }

    /// Executes PostCSS on the given CSS content using the specified config file.
    pub fn run(
        config_path: &Path,
        css: &str,
        source_file: &Path,
    ) -> Result<(String, Option<SourceMap>, Vec<StyleDiagnostic>), Vec<StyleDiagnostic>> {
        let runner_script = Self::find_runner_script().ok_or_else(|| {
            vec![StyleDiagnostic::error(
                "PostCSS runner script (tools/postcss-runner/runner.js) not found",
                source_file.to_string_lossy().to_string(),
                1,
                1,
            )]
        })?;

        let mut child = Command::new("node")
            .arg(&runner_script)
            .arg("--config")
            .arg(config_path)
            .arg("--from")
            .arg(source_file)
            .arg("--input")
            .arg("-")
            .arg("--json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                vec![StyleDiagnostic::error(
                    format!("Failed to spawn node for PostCSS execution: {}", e),
                    source_file.to_string_lossy().to_string(),
                    1,
                    1,
                )]
            })?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(css.as_bytes());
        }

        let output = child.wait_with_output().map_err(|e| {
            vec![StyleDiagnostic::error(
                format!("Failed to wait for PostCSS runner: {}", e),
                source_file.to_string_lossy().to_string(),
                1,
                1,
            )]
        })?;

        let stdout_str = String::from_utf8_lossy(&output.stdout);
        let parsed_output: PostCssOutput = serde_json::from_str(&stdout_str).map_err(|e| {
            let stderr_str = String::from_utf8_lossy(&output.stderr);
            vec![StyleDiagnostic::error(
                format!(
                    "PostCSS runner output parse error: {}. Stderr: {}",
                    e, stderr_str
                ),
                source_file.to_string_lossy().to_string(),
                1,
                1,
            )]
        })?;

        if !parsed_output.success {
            let err_msg = parsed_output
                .error
                .unwrap_or_else(|| "Unknown PostCSS error".to_string());
            let line = parsed_output.line.unwrap_or(1) as usize;
            let col = parsed_output.column.unwrap_or(1) as usize;
            return Err(vec![StyleDiagnostic::error(
                format!("PostCSS error: {}", err_msg),
                source_file.to_string_lossy().to_string(),
                line,
                col,
            )]);
        }

        let sourcemap = parsed_output.map.as_deref().and_then(SourceMap::parse);

        let mut diagnostics = Vec::new();
        for warning in parsed_output.warnings {
            let mut line = warning.line.unwrap_or(1) as usize;
            let mut col = warning.column.unwrap_or(1) as usize;
            let mut file = source_file.to_string_lossy().to_string();

            if let Some(ref sm) = sourcemap {
                if let Some(orig) = sm.lookup(line, col) {
                    file = orig.source;
                    line = orig.line;
                    col = orig.col;
                }
            }

            diagnostics.push(StyleDiagnostic::warning(warning.text, file, line, col));
        }

        Ok((parsed_output.css, sourcemap, diagnostics))
    }
}
