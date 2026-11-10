//! Deep linking and URL scheme normalization for Android intents.

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DeepLinkError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),
    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
    #[error("Missing host or path")]
    MissingPath,
}

/// Normalizes a deep link URL into an internal router pathname with optional query string.
///
/// Supports:
/// - Custom schemes: `myapp://path/to/screen?query=val` -> `/path/to/screen?query=val`
/// - Universal links: `https://example.com/path/to/screen` -> `/path/to/screen`
pub fn parse_deep_link(raw_url: &str, expected_scheme: &str) -> Result<String, DeepLinkError> {
    let trimmed = raw_url.trim();
    if trimmed.is_empty() {
        return Err(DeepLinkError::InvalidUrl("Empty URL".to_string()));
    }

    let (scheme, rest) = trimmed
        .split_once("://")
        .ok_or_else(|| DeepLinkError::InvalidUrl(trimmed.to_string()))?;

    if scheme.eq_ignore_ascii_case("https") || scheme.eq_ignore_ascii_case("http") {
        // Universal / App Link: strip host
        let (_host, path_and_query) = rest.split_once('/').unwrap_or(("", ""));
        let path = if path_and_query.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", path_and_query)
        };
        Ok(path)
    } else if scheme.eq_ignore_ascii_case(expected_scheme) {
        // Custom URI scheme
        let path = if rest.starts_with('/') {
            rest.to_string()
        } else {
            format!("/{}", rest)
        };
        Ok(path)
    } else {
        Err(DeepLinkError::UnsupportedScheme(scheme.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_scheme() {
        let parsed = parse_deep_link("ferrix://users/123?tab=info", "ferrix").unwrap();
        assert_eq!(parsed, "/users/123?tab=info");
    }

    #[test]
    fn test_universal_link() {
        let parsed = parse_deep_link("https://app.ferrix.dev/feed/trending", "ferrix").unwrap();
        assert_eq!(parsed, "/feed/trending");
    }

    #[test]
    fn test_unsupported_scheme() {
        let err = parse_deep_link("other://home", "ferrix").unwrap_err();
        assert_eq!(err, DeepLinkError::UnsupportedScheme("other".to_string()));
    }
}
