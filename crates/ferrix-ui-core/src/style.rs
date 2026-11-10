//! Style system for Ferrix components matching React Native StyleSheet & CSS Flexbox standards.

use ferrix_layout::{DimensionValue, LayoutStyle, *};
use ferrix_renderer_protocol::Property;

/// Color utilities supporting hex, rgb, rgba, and named colors.
pub struct Color;

impl Color {
    pub const TRANSPARENT: u32 = 0x00000000;
    pub const BLACK: u32 = 0xFF000000;
    pub const WHITE: u32 = 0xFFFFFFFF;
    pub const RED: u32 = 0xFFFF0000;
    pub const GREEN: u32 = 0xFF00FF00;
    pub const BLUE: u32 = 0xFF0000FF;
    pub const GRAY: u32 = 0xFF808080;
    pub const LIGHT_GRAY: u32 = 0xFFE5E7EB;
    pub const DARK_GRAY: u32 = 0xFF1F2937;
    pub const PRIMARY: u32 = 0xFF2563EB; // Blue 600

    /// Creates an ARGB color from RGBA components.
    #[inline]
    pub const fn rgba(r: u8, g: u8, b: u8, a: f32) -> u32 {
        let alpha = ((a * 255.0).clamp(0.0, 255.0)) as u32;
        (alpha << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }

    /// Creates an opaque ARGB color from RGB components.
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
        Self::rgba(r, g, b, 1.0)
    }

    /// Parses a CSS hex string into ARGB `u32` (e.g. "#RGB", "#RGBA", "#RRGGBB", "#RRGGBBAA").
    pub fn hex(s: &str) -> u32 {
        Self::parse(s).unwrap_or(Self::BLACK)
    }

    /// Parses a color string (hex or named).
    pub fn parse(s: &str) -> Option<u32> {
        let s = s.trim();
        if let Some(hex) = s.strip_prefix('#') {
            match hex.len() {
                3 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                    Some(Self::rgb(r * 17, g * 17, b * 17))
                }
                4 => {
                    let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                    let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                    let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                    let a = u8::from_str_radix(&hex[3..4], 16).ok()?;
                    Some(Self::rgba(r * 17, g * 17, b * 17, (a * 17) as f32 / 255.0))
                }
                6 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    Some(Self::rgb(r, g, b))
                }
                8 => {
                    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                    let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                    Some(Self::rgba(r, g, b, a as f32 / 255.0))
                }
                _ => None,
            }
        } else {
            match s.to_lowercase().as_str() {
                "transparent" => Some(Self::TRANSPARENT),
                "black" => Some(Self::BLACK),
                "white" => Some(Self::WHITE),
                "red" => Some(Self::RED),
                "green" => Some(Self::GREEN),
                "blue" => Some(Self::BLUE),
                "gray" | "grey" => Some(Self::GRAY),
                _ => None,
            }
        }
    }
}

/// Comprehensive styling struct for Ferrix components.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    // Layout & Flexbox
    pub flex_direction: Option<FlexDirection>,
    pub flex_grow: Option<f32>,
    pub flex_shrink: Option<f32>,
    pub flex_basis: Option<DimensionValue>,
    pub align_items: Option<AlignItems>,
    pub align_self: Option<AlignSelf>,
    pub align_content: Option<AlignContent>,
    pub justify_content: Option<JustifyContent>,
    pub flex_wrap: Option<FlexWrap>,
    pub gap: Option<(f32, f32)>,
    pub position: Option<Position>,
    pub width: Option<DimensionValue>,
    pub height: Option<DimensionValue>,
    pub min_width: Option<DimensionValue>,
    pub max_width: Option<DimensionValue>,
    pub min_height: Option<DimensionValue>,
    pub max_height: Option<DimensionValue>,
    pub aspect_ratio: Option<f32>,
    pub left: Option<DimensionValue>,
    pub right: Option<DimensionValue>,
    pub top: Option<DimensionValue>,
    pub bottom: Option<DimensionValue>,
    pub start: Option<DimensionValue>,
    pub end: Option<DimensionValue>,
    pub display: Option<Display>,

    // Spacing
    pub margin_left: Option<DimensionValue>,
    pub margin_right: Option<DimensionValue>,
    pub margin_top: Option<DimensionValue>,
    pub margin_bottom: Option<DimensionValue>,
    pub margin_start: Option<DimensionValue>,
    pub margin_end: Option<DimensionValue>,
    pub padding_left: Option<DimensionValue>,
    pub padding_right: Option<DimensionValue>,
    pub padding_top: Option<DimensionValue>,
    pub padding_bottom: Option<DimensionValue>,
    pub padding_start: Option<DimensionValue>,
    pub padding_end: Option<DimensionValue>,

    // Visual & Appearance
    pub background_color: Option<u32>,
    pub opacity: Option<f32>,
    pub border_width: Option<(f32, f32, f32, f32)>,
    pub border_color: Option<u32>,
    pub border_radius: Option<(f32, f32, f32, f32)>,
    pub elevation: Option<f32>,
    pub transform: Option<(f32, f32, f32, f32, f32)>,

    // Text & Typography
    pub color: Option<u32>,
    pub font_size: Option<f32>,
    pub font_weight: Option<u16>,
    pub font_style: Option<u8>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
    pub text_align: Option<u8>,
    pub text_decoration: Option<u8>,
    pub number_of_lines: Option<u32>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn flex_direction(mut self, dir: FlexDirection) -> Self {
        self.flex_direction = Some(dir);
        self
    }

    pub fn flex(mut self, val: f32) -> Self {
        self.flex_grow = Some(val);
        self.flex_shrink = Some(1.0);
        self
    }

    pub fn flex_grow(mut self, val: f32) -> Self {
        self.flex_grow = Some(val);
        self
    }

    pub fn flex_shrink(mut self, val: f32) -> Self {
        self.flex_shrink = Some(val);
        self
    }

    pub fn align_items(mut self, align: AlignItems) -> Self {
        self.align_items = Some(align);
        self
    }

    pub fn justify_content(mut self, justify: JustifyContent) -> Self {
        self.justify_content = Some(justify);
        self
    }

    pub fn gap(mut self, g: f32) -> Self {
        self.gap = Some((g, g));
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(DimensionValue::Points(w));
        self
    }

    pub fn width_pct(mut self, pct: f32) -> Self {
        self.width = Some(DimensionValue::Percent(pct));
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(DimensionValue::Points(h));
        self
    }

    pub fn height_pct(mut self, pct: f32) -> Self {
        self.height = Some(DimensionValue::Percent(pct));
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding_left = Some(DimensionValue::Points(p));
        self.padding_right = Some(DimensionValue::Points(p));
        self.padding_top = Some(DimensionValue::Points(p));
        self.padding_bottom = Some(DimensionValue::Points(p));
        self
    }

    pub fn padding_horizontal(mut self, p: f32) -> Self {
        self.padding_left = Some(DimensionValue::Points(p));
        self.padding_right = Some(DimensionValue::Points(p));
        self
    }

    pub fn padding_vertical(mut self, p: f32) -> Self {
        self.padding_top = Some(DimensionValue::Points(p));
        self.padding_bottom = Some(DimensionValue::Points(p));
        self
    }

    pub fn margin(mut self, m: f32) -> Self {
        self.margin_left = Some(DimensionValue::Points(m));
        self.margin_right = Some(DimensionValue::Points(m));
        self.margin_top = Some(DimensionValue::Points(m));
        self.margin_bottom = Some(DimensionValue::Points(m));
        self
    }

    pub fn margin_horizontal(mut self, m: f32) -> Self {
        self.margin_left = Some(DimensionValue::Points(m));
        self.margin_right = Some(DimensionValue::Points(m));
        self
    }

    pub fn margin_vertical(mut self, m: f32) -> Self {
        self.margin_top = Some(DimensionValue::Points(m));
        self.margin_bottom = Some(DimensionValue::Points(m));
        self
    }

    pub fn background_color(mut self, color: u32) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font_size(mut self, sp: f32) -> Self {
        self.font_size = Some(sp);
        self
    }

    pub fn font_weight(mut self, weight: u16) -> Self {
        self.font_weight = Some(weight);
        self
    }

    pub fn border_radius(mut self, r: f32) -> Self {
        self.border_radius = Some((r, r, r, r));
        self
    }

    pub fn border_width(mut self, w: f32) -> Self {
        self.border_width = Some((w, w, w, w));
        self
    }

    pub fn border_color(mut self, color: u32) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn opacity(mut self, op: f32) -> Self {
        self.opacity = Some(op);
        self
    }

    pub fn elevation(mut self, elev: f32) -> Self {
        self.elevation = Some(elev);
        self
    }

    /// Merges another style over this style (other takes precedence for defined fields).
    pub fn merge(&self, other: &Style) -> Style {
        Style {
            flex_direction: other.flex_direction.or(self.flex_direction),
            flex_grow: other.flex_grow.or(self.flex_grow),
            flex_shrink: other.flex_shrink.or(self.flex_shrink),
            flex_basis: other.flex_basis.or(self.flex_basis),
            align_items: other.align_items.or(self.align_items),
            align_self: other.align_self.or(self.align_self),
            align_content: other.align_content.or(self.align_content),
            justify_content: other.justify_content.or(self.justify_content),
            flex_wrap: other.flex_wrap.or(self.flex_wrap),
            gap: other.gap.or(self.gap),
            position: other.position.or(self.position),
            width: other.width.or(self.width),
            height: other.height.or(self.height),
            min_width: other.min_width.or(self.min_width),
            max_width: other.max_width.or(self.max_width),
            min_height: other.min_height.or(self.min_height),
            max_height: other.max_height.or(self.max_height),
            aspect_ratio: other.aspect_ratio.or(self.aspect_ratio),
            left: other.left.or(self.left),
            right: other.right.or(self.right),
            top: other.top.or(self.top),
            bottom: other.bottom.or(self.bottom),
            start: other.start.or(self.start),
            end: other.end.or(self.end),
            display: other.display.or(self.display),
            margin_left: other.margin_left.or(self.margin_left),
            margin_right: other.margin_right.or(self.margin_right),
            margin_top: other.margin_top.or(self.margin_top),
            margin_bottom: other.margin_bottom.or(self.margin_bottom),
            margin_start: other.margin_start.or(self.margin_start),
            margin_end: other.margin_end.or(self.margin_end),
            padding_left: other.padding_left.or(self.padding_left),
            padding_right: other.padding_right.or(self.padding_right),
            padding_top: other.padding_top.or(self.padding_top),
            padding_bottom: other.padding_bottom.or(self.padding_bottom),
            padding_start: other.padding_start.or(self.padding_start),
            padding_end: other.padding_end.or(self.padding_end),
            background_color: other.background_color.or(self.background_color),
            opacity: other.opacity.or(self.opacity),
            border_width: other.border_width.or(self.border_width),
            border_color: other.border_color.or(self.border_color),
            border_radius: other.border_radius.or(self.border_radius),
            elevation: other.elevation.or(self.elevation),
            transform: other.transform.or(self.transform),
            color: other.color.or(self.color),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.or(self.font_weight),
            font_style: other.font_style.or(self.font_style),
            line_height: other.line_height.or(self.line_height),
            letter_spacing: other.letter_spacing.or(self.letter_spacing),
            text_align: other.text_align.or(self.text_align),
            text_decoration: other.text_decoration.or(self.text_decoration),
            number_of_lines: other.number_of_lines.or(self.number_of_lines),
        }
    }

    /// Converts non-layout visual and text styling into protocol `Property` values.
    pub fn to_properties(&self) -> Vec<Property> {
        let mut props = Vec::new();
        if let Some(c) = self.background_color {
            props.push(Property::BackgroundColor(c));
        }
        if let Some(c) = self.color {
            props.push(Property::TextColor(c));
        }
        if let Some(s) = self.font_size {
            props.push(Property::FontSize(s));
        }
        if let Some(w) = self.font_weight {
            props.push(Property::FontWeight(w));
        }
        if let Some(s) = self.font_style {
            props.push(Property::FontStyle(s));
        }
        if let Some(lh) = self.line_height {
            props.push(Property::LineHeight(lh));
        }
        if let Some(ls) = self.letter_spacing {
            props.push(Property::LetterSpacing(ls));
        }
        if let Some(ta) = self.text_align {
            props.push(Property::TextAlign(ta));
        }
        if let Some(td) = self.text_decoration {
            props.push(Property::TextDecoration(td));
        }
        if let Some(n) = self.number_of_lines {
            props.push(Property::NumberOfLines(n));
        }
        if let Some(op) = self.opacity {
            props.push(Property::Opacity(op));
        }
        if let Some(elev) = self.elevation {
            props.push(Property::Elevation(elev));
        }
        if let Some((l, t, r, b)) = self.border_width {
            props.push(Property::BorderWidth {
                left: l,
                top: t,
                right: r,
                bottom: b,
            });
        }
        if let Some(c) = self.border_color {
            props.push(Property::BorderColor(c));
        }
        if let Some((tl, tr, br, bl)) = self.border_radius {
            props.push(Property::BorderRadius {
                top_left: tl,
                top_right: tr,
                bottom_right: br,
                bottom_left: bl,
            });
        }
        if let Some((tx, ty, sx, sy, rot)) = self.transform {
            props.push(Property::Transform {
                translate_x: tx,
                translate_y: ty,
                scale_x: sx,
                scale_y: sy,
                rotate_deg: rot,
            });
        }
        props
    }

    /// Converts layout-specific styles into `LayoutStyle` for Taffy layout computation.
    pub fn to_layout_style(&self) -> LayoutStyle {
        let mut ls = LayoutStyle::default();
        if let Some(dir) = self.flex_direction {
            ls.flex_direction = dir;
        }
        if let Some(g) = self.flex_grow {
            ls.flex_grow = g;
        }
        if let Some(s) = self.flex_shrink {
            ls.flex_shrink = s;
        }
        if let Some(b) = self.flex_basis {
            ls.flex_basis = b;
        }
        if let Some(ai) = self.align_items {
            ls.align_items = Some(ai);
        }
        if let Some(aself) = self.align_self {
            ls.align_self = Some(aself);
        }
        if let Some(ac) = self.align_content {
            ls.align_content = Some(ac);
        }
        if let Some(jc) = self.justify_content {
            ls.justify_content = Some(jc);
        }
        if let Some(fw) = self.flex_wrap {
            ls.flex_wrap = fw;
        }
        if let Some(gap) = self.gap {
            ls.gap = gap;
        }
        if let Some(pos) = self.position {
            ls.position = pos;
        }
        if let Some(w) = self.width {
            ls.width = w;
        }
        if let Some(h) = self.height {
            ls.height = h;
        }
        if let Some(mw) = self.min_width {
            ls.min_width = mw;
        }
        if let Some(mw) = self.max_width {
            ls.max_width = mw;
        }
        if let Some(mh) = self.min_height {
            ls.min_height = mh;
        }
        if let Some(mh) = self.max_height {
            ls.max_height = mh;
        }
        if let Some(ar) = self.aspect_ratio {
            ls.aspect_ratio = Some(ar);
        }
        if let Some(l) = self.margin_left {
            ls.margin_left = l;
        }
        if let Some(r) = self.margin_right {
            ls.margin_right = r;
        }
        if let Some(t) = self.margin_top {
            ls.margin_top = t;
        }
        if let Some(b) = self.margin_bottom {
            ls.margin_bottom = b;
        }
        if let Some(s) = self.margin_start {
            ls.margin_start = s;
        }
        if let Some(e) = self.margin_end {
            ls.margin_end = e;
        }
        if let Some(l) = self.padding_left {
            ls.padding_left = l;
        }
        if let Some(r) = self.padding_right {
            ls.padding_right = r;
        }
        if let Some(t) = self.padding_top {
            ls.padding_top = t;
        }
        if let Some(b) = self.padding_bottom {
            ls.padding_bottom = b;
        }
        if let Some(s) = self.padding_start {
            ls.padding_start = s;
        }
        if let Some(e) = self.padding_end {
            ls.padding_end = e;
        }
        if let Some(d) = self.display {
            ls.display = d;
        }
        ls
    }
}

/// Helper mirroring React Native `StyleSheet.create(...)`.
pub struct StyleSheet;

impl StyleSheet {
    #[inline]
    pub fn create<T>(styles: T) -> T {
        styles
    }

    /// Merges an array or iterator of optional styles into a single computed style.
    pub fn merge<'a>(styles: impl IntoIterator<Item = Option<&'a Style>>) -> Style {
        let mut result = Style::new();
        for style in styles.into_iter().flatten() {
            result = result.merge(style);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_parsing() {
        assert_eq!(Color::hex("#FF0000"), 0xFFFF0000);
        assert_eq!(Color::hex("#00FF00"), 0xFF00FF00);
        assert_eq!(Color::hex("#0000FF"), 0xFF0000FF);
        assert_eq!(Color::hex("#FFF"), 0xFFFFFFFF);
        assert_eq!(Color::parse("transparent"), Some(0x00000000));
        assert_eq!(Color::parse("white"), Some(0xFFFFFFFF));
    }

    #[test]
    fn test_style_merging() {
        let base = Style::new()
            .flex_direction(FlexDirection::Column)
            .padding(16.0)
            .background_color(Color::WHITE);

        let active = Style::new().background_color(Color::PRIMARY).elevation(4.0);

        let merged = StyleSheet::merge([Some(&base), Some(&active)]);

        assert_eq!(merged.flex_direction, Some(FlexDirection::Column));
        assert_eq!(merged.padding_left, Some(DimensionValue::Points(16.0)));
        assert_eq!(merged.background_color, Some(Color::PRIMARY)); // Overridden by active
        assert_eq!(merged.elevation, Some(4.0));
    }
}
