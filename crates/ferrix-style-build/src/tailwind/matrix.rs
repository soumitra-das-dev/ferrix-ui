//! Tailwind utility class mapping matrix.
//! Translates Tailwind tokens into native `StyleIr` declarations.
//! Catches unsupported web properties and flags them with actionable diagnostics.

use ferrix_layout::{
    AlignContent, AlignItems, AlignSelf, DimensionValue, Display, FlexDirection, FlexWrap,
    JustifyContent, Position,
};
use ferrix_style::StyleIr;

use super::arbitrary::{parse_arbitrary_color, parse_arbitrary_dimension, parse_arbitrary_f32};
use super::palette::resolve_tailwind_color;
use crate::diagnostics::StyleDiagnostic;

/// Standard Tailwind spacing scale lookup in density-independent points (dp).
pub fn spacing_scale(key: &str) -> Option<f32> {
    match key {
        "0" => Some(0.0),
        "0.5" => Some(2.0),
        "1" => Some(4.0),
        "1.5" => Some(6.0),
        "2" => Some(8.0),
        "2.5" => Some(10.0),
        "3" => Some(12.0),
        "3.5" => Some(14.0),
        "4" => Some(16.0),
        "5" => Some(20.0),
        "6" => Some(24.0),
        "7" => Some(28.0),
        "8" => Some(32.0),
        "9" => Some(36.0),
        "10" => Some(40.0),
        "11" => Some(44.0),
        "12" => Some(48.0),
        "14" => Some(56.0),
        "16" => Some(64.0),
        "20" => Some(80.0),
        "24" => Some(96.0),
        "28" => Some(112.0),
        "32" => Some(128.0),
        "36" => Some(144.0),
        "40" => Some(160.0),
        "44" => Some(176.0),
        "48" => Some(192.0),
        "52" => Some(208.0),
        "56" => Some(224.0),
        "60" => Some(240.0),
        "64" => Some(256.0),
        "72" => Some(288.0),
        "80" => Some(320.0),
        "96" => Some(384.0),
        _ => None,
    }
}

/// Parses a dimension from Tailwind scale or arbitrary value.
pub fn parse_scale_or_arbitrary(val: &str) -> Option<DimensionValue> {
    if val.starts_with('[') && val.ends_with(']') {
        return parse_arbitrary_dimension(val);
    }

    if let Some(dp) = spacing_scale(val) {
        return Some(DimensionValue::Points(dp));
    }

    match val {
        "full" => Some(DimensionValue::Percent(100.0)),
        "screen" => Some(DimensionValue::Percent(100.0)),
        "auto" => Some(DimensionValue::Auto),
        "1/2" => Some(DimensionValue::Percent(50.0)),
        "1/3" => Some(DimensionValue::Percent(33.333)),
        "2/3" => Some(DimensionValue::Percent(66.666)),
        "1/4" => Some(DimensionValue::Percent(25.0)),
        "2/4" => Some(DimensionValue::Percent(50.0)),
        "3/4" => Some(DimensionValue::Percent(75.0)),
        "1/5" => Some(DimensionValue::Percent(20.0)),
        "2/5" => Some(DimensionValue::Percent(40.0)),
        "3/5" => Some(DimensionValue::Percent(60.0)),
        "4/5" => Some(DimensionValue::Percent(80.0)),
        _ => None,
    }
}

/// Maps a single Tailwind utility class to its corresponding `StyleIr`.
pub fn map_utility(class_name: &str) -> Result<Option<StyleIr>, StyleDiagnostic> {
    // Check unsupported web-only classes
    if class_name.starts_with("float-")
        || class_name.starts_with("clear-")
        || class_name.starts_with("cursor-")
        || class_name.starts_with("grid-")
        || class_name.starts_with("col-")
        || class_name.starts_with("row-")
        || class_name.starts_with("table-")
        || class_name.starts_with("select-")
    {
        return Err(StyleDiagnostic::error(
            format!(
                "Tailwind utility '{}' is not supported in native mobile layouts",
                class_name
            ),
            "tailwind",
            1,
            1,
        ));
    }

    // Handle variants: active:, dark:, sm:, md:, lg:, focus:, disabled:, checked:
    if let Some((variant, rest)) = class_name.split_once(':') {
        let inner_res = map_utility(rest)?;
        if let Some(inner_style) = inner_res {
            let mut style = StyleIr::new();
            match variant {
                "active" => style.active = Some(Box::new(inner_style)),
                "dark" => style.dark = Some(Box::new(inner_style)),
                "sm" => style.sm = Some(Box::new(inner_style)),
                "md" => style.md = Some(Box::new(inner_style)),
                "lg" => style.lg = Some(Box::new(inner_style)),
                "focus" => style.focus = Some(Box::new(inner_style)),
                "disabled" => style.disabled = Some(Box::new(inner_style)),
                "checked" => style.checked = Some(Box::new(inner_style)),
                _ => return Ok(None),
            }
            return Ok(Some(style));
        }
        return Ok(None);
    }

    let mut style = StyleIr::new();

    // 1. Layout & Flexbox
    match class_name {
        "flex" => {
            style.display = Some(Display::Flex);
            return Ok(Some(style));
        }
        "hidden" => {
            style.display = Some(Display::None);
            return Ok(Some(style));
        }
        "flex-row" => {
            style.flex_direction = Some(FlexDirection::Row);
            return Ok(Some(style));
        }
        "flex-col" => {
            style.flex_direction = Some(FlexDirection::Column);
            return Ok(Some(style));
        }
        "flex-row-reverse" => {
            style.flex_direction = Some(FlexDirection::RowReverse);
            return Ok(Some(style));
        }
        "flex-col-reverse" => {
            style.flex_direction = Some(FlexDirection::ColumnReverse);
            return Ok(Some(style));
        }
        "flex-wrap" => {
            style.flex_wrap = Some(FlexWrap::Wrap);
            return Ok(Some(style));
        }
        "flex-nowrap" => {
            style.flex_wrap = Some(FlexWrap::NoWrap);
            return Ok(Some(style));
        }
        "flex-wrap-reverse" => {
            style.flex_wrap = Some(FlexWrap::WrapReverse);
            return Ok(Some(style));
        }
        "flex-1" => {
            style.flex_grow = Some(1.0);
            style.flex_shrink = Some(1.0);
            style.flex_basis = Some(DimensionValue::Points(0.0));
            return Ok(Some(style));
        }
        "flex-auto" => {
            style.flex_grow = Some(1.0);
            style.flex_shrink = Some(1.0);
            style.flex_basis = Some(DimensionValue::Auto);
            return Ok(Some(style));
        }
        "flex-initial" => {
            style.flex_grow = Some(0.0);
            style.flex_shrink = Some(1.0);
            style.flex_basis = Some(DimensionValue::Auto);
            return Ok(Some(style));
        }
        "flex-none" => {
            style.flex_grow = Some(0.0);
            style.flex_shrink = Some(0.0);
            style.flex_basis = Some(DimensionValue::Auto);
            return Ok(Some(style));
        }
        "grow" | "flex-grow" => {
            style.flex_grow = Some(1.0);
            return Ok(Some(style));
        }
        "grow-0" | "flex-grow-0" => {
            style.flex_grow = Some(0.0);
            return Ok(Some(style));
        }
        "shrink" | "flex-shrink" => {
            style.flex_shrink = Some(1.0);
            return Ok(Some(style));
        }
        "shrink-0" | "flex-shrink-0" => {
            style.flex_shrink = Some(0.0);
            return Ok(Some(style));
        }
        "items-start" => {
            style.align_items = Some(AlignItems::FlexStart);
            return Ok(Some(style));
        }
        "items-center" => {
            style.align_items = Some(AlignItems::Center);
            return Ok(Some(style));
        }
        "items-end" => {
            style.align_items = Some(AlignItems::FlexEnd);
            return Ok(Some(style));
        }
        "items-stretch" => {
            style.align_items = Some(AlignItems::Stretch);
            return Ok(Some(style));
        }
        "items-baseline" => {
            style.align_items = Some(AlignItems::Baseline);
            return Ok(Some(style));
        }
        "justify-start" => {
            style.justify_content = Some(JustifyContent::FlexStart);
            return Ok(Some(style));
        }
        "justify-center" => {
            style.justify_content = Some(JustifyContent::Center);
            return Ok(Some(style));
        }
        "justify-end" => {
            style.justify_content = Some(JustifyContent::FlexEnd);
            return Ok(Some(style));
        }
        "justify-between" => {
            style.justify_content = Some(JustifyContent::SpaceBetween);
            return Ok(Some(style));
        }
        "justify-around" => {
            style.justify_content = Some(JustifyContent::SpaceAround);
            return Ok(Some(style));
        }
        "justify-evenly" => {
            style.justify_content = Some(JustifyContent::SpaceEvenly);
            return Ok(Some(style));
        }
        "content-start" => {
            style.align_content = Some(AlignContent::FlexStart);
            return Ok(Some(style));
        }
        "content-center" => {
            style.align_content = Some(AlignContent::Center);
            return Ok(Some(style));
        }
        "content-end" => {
            style.align_content = Some(AlignContent::FlexEnd);
            return Ok(Some(style));
        }
        "content-between" => {
            style.align_content = Some(AlignContent::SpaceBetween);
            return Ok(Some(style));
        }
        "content-around" => {
            style.align_content = Some(AlignContent::SpaceAround);
            return Ok(Some(style));
        }
        "content-stretch" => {
            style.align_content = Some(AlignContent::Stretch);
            return Ok(Some(style));
        }
        "self-auto" => {
            style.align_self = None;
            return Ok(Some(style));
        }
        "self-start" => {
            style.align_self = Some(AlignSelf::FlexStart);
            return Ok(Some(style));
        }
        "self-center" => {
            style.align_self = Some(AlignSelf::Center);
            return Ok(Some(style));
        }
        "self-end" => {
            style.align_self = Some(AlignSelf::FlexEnd);
            return Ok(Some(style));
        }
        "self-stretch" => {
            style.align_self = Some(AlignSelf::Stretch);
            return Ok(Some(style));
        }
        "absolute" => {
            style.position = Some(Position::Absolute);
            return Ok(Some(style));
        }
        "relative" => {
            style.position = Some(Position::Relative);
            return Ok(Some(style));
        }
        _ => {}
    }

    // 2. Insets (inset, top, bottom, left, right)
    if let Some(rest) = class_name.strip_prefix("inset-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.left = Some(dim);
            style.top = Some(dim);
            style.right = Some(dim);
            style.bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("top-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.top = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("bottom-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("left-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.left = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("right-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.right = Some(dim);
            return Ok(Some(style));
        }
    }

    // 3. Spacing (padding, margin, gap)
    if let Some(rest) = class_name.strip_prefix("p-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_left = Some(dim);
            style.padding_top = Some(dim);
            style.padding_right = Some(dim);
            style.padding_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("px-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_left = Some(dim);
            style.padding_right = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("py-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_top = Some(dim);
            style.padding_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("pt-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_top = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("pb-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("pl-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_left = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("pr-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.padding_right = Some(dim);
            return Ok(Some(style));
        }
    }

    if let Some(rest) = class_name.strip_prefix("m-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_left = Some(dim);
            style.margin_top = Some(dim);
            style.margin_right = Some(dim);
            style.margin_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("mx-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_left = Some(dim);
            style.margin_right = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("my-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_top = Some(dim);
            style.margin_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("mt-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_top = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("mb-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_bottom = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("ml-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_left = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("mr-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.margin_right = Some(dim);
            return Ok(Some(style));
        }
    }

    if let Some(rest) = class_name.strip_prefix("gap-") {
        if let Some(dp) = spacing_scale(rest) {
            style.gap = Some((dp, dp));
            return Ok(Some(style));
        } else if rest.starts_with('[') && rest.ends_with(']') {
            if let Some(dp) = parse_arbitrary_f32(rest) {
                style.gap = Some((dp, dp));
                return Ok(Some(style));
            }
        }
    }

    // 4. Sizing (w-, h-, min-w, max-w, min-h, max-h)
    if let Some(rest) = class_name.strip_prefix("w-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.width = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("h-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.height = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("min-w-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.min_width = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("max-w-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.max_width = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("min-h-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.min_height = Some(dim);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("max-h-") {
        if let Some(dim) = parse_scale_or_arbitrary(rest) {
            style.max_height = Some(dim);
            return Ok(Some(style));
        }
    }

    // 5. Typography
    match class_name {
        "text-xs" => {
            style.font_size = Some(12.0);
            return Ok(Some(style));
        }
        "text-sm" => {
            style.font_size = Some(14.0);
            return Ok(Some(style));
        }
        "text-base" => {
            style.font_size = Some(16.0);
            return Ok(Some(style));
        }
        "text-lg" => {
            style.font_size = Some(18.0);
            return Ok(Some(style));
        }
        "text-xl" => {
            style.font_size = Some(20.0);
            return Ok(Some(style));
        }
        "text-2xl" => {
            style.font_size = Some(24.0);
            return Ok(Some(style));
        }
        "text-3xl" => {
            style.font_size = Some(30.0);
            return Ok(Some(style));
        }
        "text-4xl" => {
            style.font_size = Some(36.0);
            return Ok(Some(style));
        }
        "text-5xl" => {
            style.font_size = Some(48.0);
            return Ok(Some(style));
        }
        "font-thin" => {
            style.font_weight = Some(100);
            return Ok(Some(style));
        }
        "font-extralight" => {
            style.font_weight = Some(200);
            return Ok(Some(style));
        }
        "font-light" => {
            style.font_weight = Some(300);
            return Ok(Some(style));
        }
        "font-normal" => {
            style.font_weight = Some(400);
            return Ok(Some(style));
        }
        "font-medium" => {
            style.font_weight = Some(500);
            return Ok(Some(style));
        }
        "font-semibold" => {
            style.font_weight = Some(600);
            return Ok(Some(style));
        }
        "font-bold" => {
            style.font_weight = Some(700);
            return Ok(Some(style));
        }
        "font-extrabold" => {
            style.font_weight = Some(800);
            return Ok(Some(style));
        }
        "font-black" => {
            style.font_weight = Some(900);
            return Ok(Some(style));
        }
        "italic" => {
            style.font_style = Some(1);
            return Ok(Some(style));
        }
        "not-italic" => {
            style.font_style = Some(0);
            return Ok(Some(style));
        }
        "text-left" => {
            style.text_align = Some(0);
            return Ok(Some(style));
        }
        "text-center" => {
            style.text_align = Some(1);
            return Ok(Some(style));
        }
        "text-right" => {
            style.text_align = Some(2);
            return Ok(Some(style));
        }
        "underline" => {
            style.text_decoration = Some(1);
            return Ok(Some(style));
        }
        "line-through" => {
            style.text_decoration = Some(2);
            return Ok(Some(style));
        }
        "no-underline" => {
            style.text_decoration = Some(0);
            return Ok(Some(style));
        }
        "leading-none" => {
            style.line_height = Some(16.0);
            return Ok(Some(style));
        }
        "leading-tight" => {
            style.line_height = Some(20.0);
            return Ok(Some(style));
        }
        "leading-snug" => {
            style.line_height = Some(22.0);
            return Ok(Some(style));
        }
        "leading-normal" => {
            style.line_height = Some(24.0);
            return Ok(Some(style));
        }
        "leading-relaxed" => {
            style.line_height = Some(26.0);
            return Ok(Some(style));
        }
        "leading-loose" => {
            style.line_height = Some(32.0);
            return Ok(Some(style));
        }
        "tracking-tighter" => {
            style.letter_spacing = Some(-0.8);
            return Ok(Some(style));
        }
        "tracking-tight" => {
            style.letter_spacing = Some(-0.4);
            return Ok(Some(style));
        }
        "tracking-normal" => {
            style.letter_spacing = Some(0.0);
            return Ok(Some(style));
        }
        "tracking-wide" => {
            style.letter_spacing = Some(0.4);
            return Ok(Some(style));
        }
        "tracking-wider" => {
            style.letter_spacing = Some(0.8);
            return Ok(Some(style));
        }
        "tracking-widest" => {
            style.letter_spacing = Some(1.6);
            return Ok(Some(style));
        }
        _ => {}
    }

    // 6. Colors (bg-, text-, border-)
    if let Some(rest) = class_name.strip_prefix("bg-") {
        if rest.starts_with('[') && rest.ends_with(']') {
            if let Some(color) = parse_arbitrary_color(rest) {
                style.background_color = Some(color);
                return Ok(Some(style));
            }
        } else if let Some(color) = parse_color_token(rest) {
            style.background_color = Some(color);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("text-") {
        if rest.starts_with('[') && rest.ends_with(']') {
            if let Some(color) = parse_arbitrary_color(rest) {
                style.color = Some(color);
                return Ok(Some(style));
            }
        } else if let Some(color) = parse_color_token(rest) {
            style.color = Some(color);
            return Ok(Some(style));
        }
    }
    if let Some(rest) = class_name.strip_prefix("border-") {
        if rest.starts_with('[') && rest.ends_with(']') {
            if let Some(color) = parse_arbitrary_color(rest) {
                style.border_color = Some(color);
                return Ok(Some(style));
            }
        } else if let Some(color) = parse_color_token(rest) {
            style.border_color = Some(color);
            return Ok(Some(style));
        }
    }

    // 7. Opacity
    if let Some(rest) = class_name.strip_prefix("opacity-") {
        if let Ok(pct) = rest.parse::<f32>() {
            style.opacity = Some((pct / 100.0).clamp(0.0, 1.0));
            return Ok(Some(style));
        }
    }

    // 8. Borders & Radii
    match class_name {
        "border" => {
            style.border_width = Some((1.0, 1.0, 1.0, 1.0));
            return Ok(Some(style));
        }
        "border-0" => {
            style.border_width = Some((0.0, 0.0, 0.0, 0.0));
            return Ok(Some(style));
        }
        "border-2" => {
            style.border_width = Some((2.0, 2.0, 2.0, 2.0));
            return Ok(Some(style));
        }
        "border-4" => {
            style.border_width = Some((4.0, 4.0, 4.0, 4.0));
            return Ok(Some(style));
        }
        "border-8" => {
            style.border_width = Some((8.0, 8.0, 8.0, 8.0));
            return Ok(Some(style));
        }
        "rounded-none" => {
            style.border_radius = Some((0.0, 0.0, 0.0, 0.0));
            return Ok(Some(style));
        }
        "rounded-sm" => {
            style.border_radius = Some((2.0, 2.0, 2.0, 2.0));
            return Ok(Some(style));
        }
        "rounded" => {
            style.border_radius = Some((4.0, 4.0, 4.0, 4.0));
            return Ok(Some(style));
        }
        "rounded-md" => {
            style.border_radius = Some((6.0, 6.0, 6.0, 6.0));
            return Ok(Some(style));
        }
        "rounded-lg" => {
            style.border_radius = Some((8.0, 8.0, 8.0, 8.0));
            return Ok(Some(style));
        }
        "rounded-xl" => {
            style.border_radius = Some((12.0, 12.0, 12.0, 12.0));
            return Ok(Some(style));
        }
        "rounded-2xl" => {
            style.border_radius = Some((16.0, 16.0, 16.0, 16.0));
            return Ok(Some(style));
        }
        "rounded-3xl" => {
            style.border_radius = Some((24.0, 24.0, 24.0, 24.0));
            return Ok(Some(style));
        }
        "rounded-full" => {
            style.border_radius = Some((9999.0, 9999.0, 9999.0, 9999.0));
            return Ok(Some(style));
        }
        _ => {}
    }

    if let Some(rest) = class_name.strip_prefix("rounded-[") {
        if let Some(end) = rest.strip_suffix(']') {
            if let Some(r) = parse_arbitrary_f32(&format!("[{}]", end)) {
                style.border_radius = Some((r, r, r, r));
                return Ok(Some(style));
            }
        }
    }

    // 9. Shadows & Elevation
    match class_name {
        "shadow-none" => {
            style.elevation = Some(0.0);
            return Ok(Some(style));
        }
        "shadow-sm" => {
            style.elevation = Some(1.0);
            return Ok(Some(style));
        }
        "shadow" => {
            style.elevation = Some(2.0);
            return Ok(Some(style));
        }
        "shadow-md" => {
            style.elevation = Some(4.0);
            return Ok(Some(style));
        }
        "shadow-lg" => {
            style.elevation = Some(8.0);
            return Ok(Some(style));
        }
        "shadow-xl" => {
            style.elevation = Some(12.0);
            return Ok(Some(style));
        }
        "shadow-2xl" => {
            style.elevation = Some(16.0);
            return Ok(Some(style));
        }
        _ => {}
    }

    if let Some(rest) = class_name.strip_prefix("elevation-") {
        if let Ok(elv) = rest.parse::<f32>() {
            style.elevation = Some(elv);
            return Ok(Some(style));
        }
    }

    // Unrecognized class (returns None so it can be handled by regular CSS or ignored)
    Ok(None)
}

/// Helper parsing `<color>-<shade>` or standalone color name.
fn parse_color_token(token: &str) -> Option<u32> {
    if let Some((name, shade_str)) = token.split_once('-') {
        if let Ok(shade) = shade_str.parse::<u16>() {
            return resolve_tailwind_color(name, Some(shade));
        }
    }
    resolve_tailwind_color(token, None)
}
