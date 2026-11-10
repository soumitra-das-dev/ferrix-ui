//! Arbitrary value extraction for Tailwind bracket syntax `[value]`.

use ferrix_layout::DimensionValue;
use ferrix_style::Color;

use crate::values::parse_dimension;

/// Extracts the inner content of a bracketed arbitrary value `[content]`.
pub fn extract_arbitrary_value(s: &str) -> Option<&str> {
    let start = s.find('[')?;
    let end = s.rfind(']')?;
    if start < end {
        Some(&s[start + 1..end])
    } else {
        None
    }
}

/// Parses an arbitrary dimension (e.g. `120px`, `50%`, `16dp`, `2rem`).
pub fn parse_arbitrary_dimension(s: &str) -> Option<DimensionValue> {
    let inner = extract_arbitrary_value(s)?;
    parse_dimension(inner)
}

/// Parses an arbitrary color (e.g. `[#3b82f6]`, `[rgb(255,0,0)]`).
pub fn parse_arbitrary_color(s: &str) -> Option<u32> {
    let inner = extract_arbitrary_value(s)?;
    Color::parse(inner)
}

/// Parses an arbitrary numeric float (e.g. `[14px]` -> 14.0, `[0.75]` -> 0.75).
pub fn parse_arbitrary_f32(s: &str) -> Option<f32> {
    let inner = extract_arbitrary_value(s)?;
    let clean = inner
        .strip_suffix("px")
        .or_else(|| inner.strip_suffix("dp"))
        .or_else(|| inner.strip_suffix("pt"))
        .unwrap_or(inner);
    clean.trim().parse::<f32>().ok()
}
