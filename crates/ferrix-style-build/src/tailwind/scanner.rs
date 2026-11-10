//! Candidate scanner extracting Tailwind class candidates from `.fsx` source files.

use std::collections::HashSet;

/// Scans source text for class candidates.
pub fn scan_candidates(source: &str, safelist: &[&str]) -> HashSet<String> {
    let mut candidates = HashSet::new();

    // 1. Add safelist
    for &s in safelist {
        for word in s.split_whitespace() {
            if !word.is_empty() {
                candidates.insert(word.to_string());
            }
        }
    }

    // 2. Scan class / className string attributes and cx(...) calls
    // Match quoted strings within the document
    let mut in_quote = None;
    let mut quote_start = 0;

    for (byte_idx, ch) in source.char_indices() {
        if ch == '"' || ch == '\'' || ch == '`' {
            if let Some(q) = in_quote {
                if q == ch {
                    // Extract quoted string
                    let content = &source[quote_start + 1..byte_idx];
                    for token in content.split_whitespace() {
                        let trimmed = token.trim_matches(|c: char| {
                            c == '"' || c == '\'' || c == '`' || c == ',' || c == ';'
                        });
                        if is_valid_candidate(trimmed) {
                            candidates.insert(trimmed.to_string());
                        }
                    }
                    in_quote = None;
                }
            } else {
                in_quote = Some(ch);
                quote_start = byte_idx;
            }
        }
    }

    candidates
}

/// Checks if a token could be a Tailwind class.
fn is_valid_candidate(token: &str) -> bool {
    if token.is_empty() || token.len() < 2 {
        return false;
    }

    // Must not contain curly braces, code punctuation
    if token.contains('{') || token.contains('}') || token.contains(';') || token.contains('(') {
        return false;
    }

    // Common utility prefix or variant check
    let check_token = if let Some((_variant, rest)) = token.split_once(':') {
        rest
    } else {
        token
    };

    // Valid identifier characters + hyphen + brackets
    check_token.chars().all(|c| {
        c.is_alphanumeric()
            || c == '-'
            || c == '_'
            || c == '/'
            || c == '['
            || c == ']'
            || c == '#'
            || c == '%'
            || c == '.'
    })
}
