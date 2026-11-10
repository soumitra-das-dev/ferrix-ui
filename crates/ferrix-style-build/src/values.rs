//! CSS value and unit parsing.

use ferrix_layout::DimensionValue;
use ferrix_style::Color;

/// Parses a dimension string into a `DimensionValue`.
pub fn parse_dimension(s: &str) -> Option<DimensionValue> {
    let s = s.trim();
    if s.eq_ignore_ascii_case("auto") {
        return Some(DimensionValue::Auto);
    }
    if let Some(num_str) = s.strip_suffix('%') {
        let val = num_str.trim().parse::<f32>().ok()?;
        return Some(DimensionValue::Percent(val));
    }
    if let Some(num_str) = s.strip_suffix("px").or_else(|| s.strip_suffix("dp")) {
        let val = num_str.trim().parse::<f32>().ok()?;
        return Some(DimensionValue::Points(val));
    }
    if let Some(num_str) = s.strip_suffix("rem") {
        let val = num_str.trim().parse::<f32>().ok()?;
        return Some(DimensionValue::Points(val * 16.0));
    }
    if let Some(num_str) = s.strip_suffix("em") {
        let val = num_str.trim().parse::<f32>().ok()?;
        return Some(DimensionValue::Points(val * 16.0));
    }
    // Unitless numeric value defaults to Points
    if let Ok(val) = s.parse::<f32>() {
        return Some(DimensionValue::Points(val));
    }
    None
}

/// Parses font size into scaled points (sp).
pub fn parse_font_size(s: &str) -> Option<f32> {
    let s = s.trim();
    if let Some(num_str) = s
        .strip_suffix("sp")
        .or_else(|| s.strip_suffix("px"))
        .or_else(|| s.strip_suffix("dp"))
    {
        num_str.trim().parse::<f32>().ok()
    } else if let Some(num_str) = s.strip_suffix("rem").or_else(|| s.strip_suffix("em")) {
        num_str.trim().parse::<f32>().ok().map(|v| v * 16.0)
    } else {
        s.parse::<f32>().ok()
    }
}

/// Parses a color string into an ARGB `u32`.
pub fn parse_color(s: &str) -> Option<u32> {
    Color::parse(s)
}
