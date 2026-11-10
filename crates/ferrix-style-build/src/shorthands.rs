//! CSS shorthand expansion for margins, paddings, borders, border-radius, gaps, and flex.

use ferrix_layout::DimensionValue;
use ferrix_style::StyleIr;

use crate::values::{parse_color, parse_dimension};

/// Expands `margin: ...` into individual directional margins.
pub fn expand_margin(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    let dims: Vec<DimensionValue> = parts.iter().filter_map(|p| parse_dimension(p)).collect();
    match dims.len() {
        1 => {
            style.margin_top = Some(dims[0]);
            style.margin_right = Some(dims[0]);
            style.margin_bottom = Some(dims[0]);
            style.margin_left = Some(dims[0]);
        }
        2 => {
            style.margin_top = Some(dims[0]);
            style.margin_bottom = Some(dims[0]);
            style.margin_right = Some(dims[1]);
            style.margin_left = Some(dims[1]);
        }
        3 => {
            style.margin_top = Some(dims[0]);
            style.margin_right = Some(dims[1]);
            style.margin_left = Some(dims[1]);
            style.margin_bottom = Some(dims[2]);
        }
        4 => {
            style.margin_top = Some(dims[0]);
            style.margin_right = Some(dims[1]);
            style.margin_bottom = Some(dims[2]);
            style.margin_left = Some(dims[3]);
        }
        _ => {}
    }
}

/// Expands `padding: ...` into individual directional paddings.
pub fn expand_padding(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    let dims: Vec<DimensionValue> = parts.iter().filter_map(|p| parse_dimension(p)).collect();
    match dims.len() {
        1 => {
            style.padding_top = Some(dims[0]);
            style.padding_right = Some(dims[0]);
            style.padding_bottom = Some(dims[0]);
            style.padding_left = Some(dims[0]);
        }
        2 => {
            style.padding_top = Some(dims[0]);
            style.padding_bottom = Some(dims[0]);
            style.padding_right = Some(dims[1]);
            style.padding_left = Some(dims[1]);
        }
        3 => {
            style.padding_top = Some(dims[0]);
            style.padding_right = Some(dims[1]);
            style.padding_left = Some(dims[1]);
            style.padding_bottom = Some(dims[2]);
        }
        4 => {
            style.padding_top = Some(dims[0]);
            style.padding_right = Some(dims[1]);
            style.padding_bottom = Some(dims[2]);
            style.padding_left = Some(dims[3]);
        }
        _ => {}
    }
}

/// Expands `border-radius: ...` into corner radii.
pub fn expand_border_radius(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    let floats: Vec<f32> = parts
        .iter()
        .filter_map(|p| match parse_dimension(p) {
            Some(DimensionValue::Points(pt)) => Some(pt),
            _ => None,
        })
        .collect();

    match floats.len() {
        1 => {
            style.border_radius = Some((floats[0], floats[0], floats[0], floats[0]));
        }
        2 => {
            style.border_radius = Some((floats[0], floats[1], floats[0], floats[1]));
        }
        3 => {
            style.border_radius = Some((floats[0], floats[1], floats[2], floats[1]));
        }
        4 => {
            style.border_radius = Some((floats[0], floats[1], floats[2], floats[3]));
        }
        _ => {}
    }
}

/// Expands `gap: ...` into (row_gap, col_gap).
pub fn expand_gap(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    let floats: Vec<f32> = parts
        .iter()
        .filter_map(|p| match parse_dimension(p) {
            Some(DimensionValue::Points(pt)) => Some(pt),
            _ => None,
        })
        .collect();

    match floats.len() {
        1 => {
            style.gap = Some((floats[0], floats[0]));
        }
        2 => {
            style.gap = Some((floats[0], floats[1]));
        }
        _ => {}
    }
}

/// Expands `border: 1px solid #ccc` or `border: 2px red`.
pub fn expand_border(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    for part in parts {
        if let Some(DimensionValue::Points(w)) = parse_dimension(part) {
            style.border_width = Some((w, w, w, w));
        } else if let Some(color) = parse_color(part) {
            style.border_color = Some(color);
        }
        // "solid", "dashed" etc. are ignored in native 1-pixel border mapping
    }
}

/// Expands `flex: ...` (e.g. `1` or `1 1 auto`).
pub fn expand_flex(val: &str, style: &mut StyleIr) {
    let parts: Vec<&str> = val.split_whitespace().collect();
    if parts.len() == 1 {
        if let Ok(num) = parts[0].parse::<f32>() {
            style.flex_grow = Some(num);
            style.flex_shrink = Some(1.0);
            style.flex_basis = Some(DimensionValue::Percent(0.0));
        }
    } else if parts.len() >= 2 {
        if let Ok(grow) = parts[0].parse::<f32>() {
            style.flex_grow = Some(grow);
        }
        if let Ok(shrink) = parts[1].parse::<f32>() {
            style.flex_shrink = Some(shrink);
        }
        if parts.len() == 3 {
            style.flex_basis = parse_dimension(parts[2]);
        }
    }
}
