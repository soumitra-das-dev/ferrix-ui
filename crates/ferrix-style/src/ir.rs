//! Native Style IR representing resolved CSS and StyleSheet declarations.
#![allow(clippy::clone_on_copy, clippy::too_many_lines)]

use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use ferrix_layout::{
    AlignContent, AlignItems, AlignSelf, DimensionValue, Display, FlexDirection, FlexWrap,
    JustifyContent, LayoutStyle, Position,
};
use ferrix_renderer_protocol::Property;

#[inline]
fn hash_f32<H: Hasher>(val: f32, state: &mut H) {
    let bits = if val == 0.0 { 0u32 } else { val.to_bits() };
    bits.hash(state);
}

#[inline]
fn eq_f32(a: f32, b: f32) -> bool {
    if a == 0.0 && b == 0.0 {
        true
    } else {
        a.to_bits() == b.to_bits()
    }
}

#[inline]
fn hash_opt_f32<H: Hasher>(opt: Option<f32>, state: &mut H) {
    if let Some(v) = opt {
        1u8.hash(state);
        hash_f32(v, state);
    } else {
        0u8.hash(state);
    }
}

#[inline]
fn eq_opt_f32(a: Option<f32>, b: Option<f32>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => eq_f32(x, y),
        (None, None) => true,
        _ => false,
    }
}

#[inline]
fn hash_dim<H: Hasher>(dim: &DimensionValue, state: &mut H) {
    match dim {
        DimensionValue::Auto => 0u8.hash(state),
        DimensionValue::Points(p) => {
            1u8.hash(state);
            hash_f32(*p, state);
        }
        DimensionValue::Percent(p) => {
            2u8.hash(state);
            hash_f32(*p, state);
        }
    }
}

#[inline]
fn eq_dim(a: &DimensionValue, b: &DimensionValue) -> bool {
    match (a, b) {
        (DimensionValue::Auto, DimensionValue::Auto) => true,
        (DimensionValue::Points(x), DimensionValue::Points(y)) => eq_f32(*x, *y),
        (DimensionValue::Percent(x), DimensionValue::Percent(y)) => eq_f32(*x, *y),
        _ => false,
    }
}

#[inline]
fn hash_opt_dim<H: Hasher>(opt: &Option<DimensionValue>, state: &mut H) {
    if let Some(d) = opt {
        1u8.hash(state);
        hash_dim(d, state);
    } else {
        0u8.hash(state);
    }
}

#[inline]
fn eq_opt_dim(a: &Option<DimensionValue>, b: &Option<DimensionValue>) -> bool {
    match (a, b) {
        (Some(x), Some(y)) => eq_dim(x, y),
        (None, None) => true,
        _ => false,
    }
}

#[inline]
fn hash_discriminant<T, H: Hasher>(val: &Option<T>, state: &mut H) {
    if let Some(v) = val {
        1u8.hash(state);
        std::mem::discriminant(v).hash(state);
    } else {
        0u8.hash(state);
    }
}

/// Native Style IR containing layout, visual appearance, and typography rules.
#[derive(Clone, Debug, Default)]
pub struct StyleIr {
    // Layout & Flexbox (React Native defaults: display: Flex, column, relative)
    pub display: Option<Display>,
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

    // Visual Appearance (Mapped to JNI Properties)
    pub background_color: Option<u32>,
    pub opacity: Option<f32>,
    pub border_width: Option<(f32, f32, f32, f32)>,
    pub border_color: Option<u32>,
    pub border_radius: Option<(f32, f32, f32, f32)>,
    pub elevation: Option<f32>,
    pub transform: Option<(f32, f32, f32, f32, f32)>,
    pub ripple_color: Option<u32>,
    pub hit_slop: Option<(f32, f32, f32, f32)>,

    // Typography
    pub color: Option<u32>,
    pub font_size: Option<f32>,
    pub font_weight: Option<u16>,
    pub font_style: Option<u8>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
    pub text_align: Option<u8>,
    pub text_decoration: Option<u8>,
    pub number_of_lines: Option<u32>,

    // CSS Variables Map
    pub variables: BTreeMap<String, String>,

    // Reactive / Pseudo-Class Variants
    pub active: Option<Box<StyleIr>>,
    pub focus: Option<Box<StyleIr>>,
    pub disabled: Option<Box<StyleIr>>,
    pub checked: Option<Box<StyleIr>>,

    // Media Query Variants
    pub dark: Option<Box<StyleIr>>,
    pub sm: Option<Box<StyleIr>>, // >= 640dp
    pub md: Option<Box<StyleIr>>, // >= 768dp
    pub lg: Option<Box<StyleIr>>, // >= 1024dp
}

impl PartialEq for StyleIr {
    fn eq(&self, other: &Self) -> bool {
        self.display == other.display
            && self.flex_direction == other.flex_direction
            && eq_opt_f32(self.flex_grow, other.flex_grow)
            && eq_opt_f32(self.flex_shrink, other.flex_shrink)
            && eq_opt_dim(&self.flex_basis, &other.flex_basis)
            && self.align_items == other.align_items
            && self.align_self == other.align_self
            && self.align_content == other.align_content
            && self.justify_content == other.justify_content
            && self.flex_wrap == other.flex_wrap
            && match (self.gap, other.gap) {
                (Some((gx1, gy1)), Some((gx2, gy2))) => eq_f32(gx1, gx2) && eq_f32(gy1, gy2),
                (None, None) => true,
                _ => false,
            }
            && self.position == other.position
            && eq_opt_dim(&self.width, &other.width)
            && eq_opt_dim(&self.height, &other.height)
            && eq_opt_dim(&self.min_width, &other.min_width)
            && eq_opt_dim(&self.max_width, &other.max_width)
            && eq_opt_dim(&self.min_height, &other.min_height)
            && eq_opt_dim(&self.max_height, &other.max_height)
            && eq_opt_f32(self.aspect_ratio, other.aspect_ratio)
            && eq_opt_dim(&self.left, &other.left)
            && eq_opt_dim(&self.right, &other.right)
            && eq_opt_dim(&self.top, &other.top)
            && eq_opt_dim(&self.bottom, &other.bottom)
            && eq_opt_dim(&self.start, &other.start)
            && eq_opt_dim(&self.end, &other.end)
            && eq_opt_dim(&self.margin_left, &other.margin_left)
            && eq_opt_dim(&self.margin_right, &other.margin_right)
            && eq_opt_dim(&self.margin_top, &other.margin_top)
            && eq_opt_dim(&self.margin_bottom, &other.margin_bottom)
            && eq_opt_dim(&self.margin_start, &other.margin_start)
            && eq_opt_dim(&self.margin_end, &other.margin_end)
            && eq_opt_dim(&self.padding_left, &other.padding_left)
            && eq_opt_dim(&self.padding_right, &other.padding_right)
            && eq_opt_dim(&self.padding_top, &other.padding_top)
            && eq_opt_dim(&self.padding_bottom, &other.padding_bottom)
            && eq_opt_dim(&self.padding_start, &other.padding_start)
            && eq_opt_dim(&self.padding_end, &other.padding_end)
            && self.background_color == other.background_color
            && eq_opt_f32(self.opacity, other.opacity)
            && match (self.border_width, other.border_width) {
                (Some((l1, t1, r1, b1)), Some((l2, t2, r2, b2))) => {
                    eq_f32(l1, l2) && eq_f32(t1, t2) && eq_f32(r1, r2) && eq_f32(b1, b2)
                }
                (None, None) => true,
                _ => false,
            }
            && self.border_color == other.border_color
            && match (self.border_radius, other.border_radius) {
                (Some((tl1, tr1, br1, bl1)), Some((tl2, tr2, br2, bl2))) => {
                    eq_f32(tl1, tl2) && eq_f32(tr1, tr2) && eq_f32(br1, br2) && eq_f32(bl1, bl2)
                }
                (None, None) => true,
                _ => false,
            }
            && eq_opt_f32(self.elevation, other.elevation)
            && match (self.transform, other.transform) {
                (Some((tx1, ty1, sx1, sy1, r1)), Some((tx2, ty2, sx2, sy2, r2))) => {
                    eq_f32(tx1, tx2)
                        && eq_f32(ty1, ty2)
                        && eq_f32(sx1, sx2)
                        && eq_f32(sy1, sy2)
                        && eq_f32(r1, r2)
                }
                (None, None) => true,
                _ => false,
            }
            && self.ripple_color == other.ripple_color
            && match (self.hit_slop, other.hit_slop) {
                (Some((l1, t1, r1, b1)), Some((l2, t2, r2, b2))) => {
                    eq_f32(l1, l2) && eq_f32(t1, t2) && eq_f32(r1, r2) && eq_f32(b1, b2)
                }
                (None, None) => true,
                _ => false,
            }
            && self.color == other.color
            && eq_opt_f32(self.font_size, other.font_size)
            && self.font_weight == other.font_weight
            && self.font_style == other.font_style
            && eq_opt_f32(self.line_height, other.line_height)
            && eq_opt_f32(self.letter_spacing, other.letter_spacing)
            && self.text_align == other.text_align
            && self.text_decoration == other.text_decoration
            && self.number_of_lines == other.number_of_lines
            && self.variables == other.variables
            && self.active == other.active
            && self.focus == other.focus
            && self.disabled == other.disabled
            && self.checked == other.checked
            && self.dark == other.dark
            && self.sm == other.sm
            && self.md == other.md
            && self.lg == other.lg
    }
}

impl Eq for StyleIr {}

impl Hash for StyleIr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_discriminant(&self.display, state);
        hash_discriminant(&self.flex_direction, state);
        hash_opt_f32(self.flex_grow, state);
        hash_opt_f32(self.flex_shrink, state);
        hash_opt_dim(&self.flex_basis, state);
        hash_discriminant(&self.align_items, state);
        hash_discriminant(&self.align_self, state);
        hash_discriminant(&self.align_content, state);
        hash_discriminant(&self.justify_content, state);
        hash_discriminant(&self.flex_wrap, state);
        if let Some((gx, gy)) = self.gap {
            1u8.hash(state);
            hash_f32(gx, state);
            hash_f32(gy, state);
        } else {
            0u8.hash(state);
        }
        hash_discriminant(&self.position, state);
        hash_opt_dim(&self.width, state);
        hash_opt_dim(&self.height, state);
        hash_opt_dim(&self.min_width, state);
        hash_opt_dim(&self.max_width, state);
        hash_opt_dim(&self.min_height, state);
        hash_opt_dim(&self.max_height, state);
        hash_opt_f32(self.aspect_ratio, state);
        hash_opt_dim(&self.left, state);
        hash_opt_dim(&self.right, state);
        hash_opt_dim(&self.top, state);
        hash_opt_dim(&self.bottom, state);
        hash_opt_dim(&self.start, state);
        hash_opt_dim(&self.end, state);

        hash_opt_dim(&self.margin_left, state);
        hash_opt_dim(&self.margin_right, state);
        hash_opt_dim(&self.margin_top, state);
        hash_opt_dim(&self.margin_bottom, state);
        hash_opt_dim(&self.margin_start, state);
        hash_opt_dim(&self.margin_end, state);
        hash_opt_dim(&self.padding_left, state);
        hash_opt_dim(&self.padding_right, state);
        hash_opt_dim(&self.padding_top, state);
        hash_opt_dim(&self.padding_bottom, state);
        hash_opt_dim(&self.padding_start, state);
        hash_opt_dim(&self.padding_end, state);

        self.background_color.hash(state);
        hash_opt_f32(self.opacity, state);
        if let Some((l, t, r, b)) = self.border_width {
            1u8.hash(state);
            hash_f32(l, state);
            hash_f32(t, state);
            hash_f32(r, state);
            hash_f32(b, state);
        } else {
            0u8.hash(state);
        }
        self.border_color.hash(state);
        if let Some((tl, tr, br, bl)) = self.border_radius {
            1u8.hash(state);
            hash_f32(tl, state);
            hash_f32(tr, state);
            hash_f32(br, state);
            hash_f32(bl, state);
        } else {
            0u8.hash(state);
        }
        hash_opt_f32(self.elevation, state);
        if let Some((tx, ty, sx, sy, r)) = self.transform {
            1u8.hash(state);
            hash_f32(tx, state);
            hash_f32(ty, state);
            hash_f32(sx, state);
            hash_f32(sy, state);
            hash_f32(r, state);
        } else {
            0u8.hash(state);
        }
        self.ripple_color.hash(state);
        if let Some((l, t, r, b)) = self.hit_slop {
            1u8.hash(state);
            hash_f32(l, state);
            hash_f32(t, state);
            hash_f32(r, state);
            hash_f32(b, state);
        } else {
            0u8.hash(state);
        }

        self.color.hash(state);
        hash_opt_f32(self.font_size, state);
        self.font_weight.hash(state);
        self.font_style.hash(state);
        hash_opt_f32(self.line_height, state);
        hash_opt_f32(self.letter_spacing, state);
        self.text_align.hash(state);
        self.text_decoration.hash(state);
        self.number_of_lines.hash(state);

        self.variables.hash(state);
        self.active.hash(state);
        self.focus.hash(state);
        self.disabled.hash(state);
        self.checked.hash(state);
        self.dark.hash(state);
        self.sm.hash(state);
        self.md.hash(state);
        self.lg.hash(state);
    }
}

impl StyleIr {
    pub fn new() -> Self {
        Self::default()
    }

    /// Merges `other` on top of `self`. Specified properties in `other` override `self`.
    pub fn merge(&self, other: &Self) -> Self {
        let mut vars = self.variables.clone();
        for (k, v) in &other.variables {
            vars.insert(k.clone(), v.clone());
        }

        Self {
            display: other.display.or(self.display),
            flex_direction: other.flex_direction.or(self.flex_direction),
            flex_grow: other.flex_grow.or(self.flex_grow),
            flex_shrink: other.flex_shrink.or(self.flex_shrink),
            flex_basis: other.flex_basis.clone().or_else(|| self.flex_basis.clone()),
            align_items: other.align_items.or(self.align_items),
            align_self: other.align_self.or(self.align_self),
            align_content: other.align_content.or(self.align_content),
            justify_content: other.justify_content.or(self.justify_content),
            flex_wrap: other.flex_wrap.or(self.flex_wrap),
            gap: other.gap.or(self.gap),
            position: other.position.or(self.position),
            width: other.width.clone().or_else(|| self.width.clone()),
            height: other.height.clone().or_else(|| self.height.clone()),
            min_width: other.min_width.clone().or_else(|| self.min_width.clone()),
            max_width: other.max_width.clone().or_else(|| self.max_width.clone()),
            min_height: other.min_height.clone().or_else(|| self.min_height.clone()),
            max_height: other.max_height.clone().or_else(|| self.max_height.clone()),
            aspect_ratio: other.aspect_ratio.or(self.aspect_ratio),
            left: other.left.clone().or_else(|| self.left.clone()),
            right: other.right.clone().or_else(|| self.right.clone()),
            top: other.top.clone().or_else(|| self.top.clone()),
            bottom: other.bottom.clone().or_else(|| self.bottom.clone()),
            start: other.start.clone().or_else(|| self.start.clone()),
            end: other.end.clone().or_else(|| self.end.clone()),
            margin_left: other
                .margin_left
                .clone()
                .or_else(|| self.margin_left.clone()),
            margin_right: other
                .margin_right
                .clone()
                .or_else(|| self.margin_right.clone()),
            margin_top: other.margin_top.clone().or_else(|| self.margin_top.clone()),
            margin_bottom: other
                .margin_bottom
                .clone()
                .or_else(|| self.margin_bottom.clone()),
            margin_start: other
                .margin_start
                .clone()
                .or_else(|| self.margin_start.clone()),
            margin_end: other.margin_end.clone().or_else(|| self.margin_end.clone()),
            padding_left: other
                .padding_left
                .clone()
                .or_else(|| self.padding_left.clone()),
            padding_right: other
                .padding_right
                .clone()
                .or_else(|| self.padding_right.clone()),
            padding_top: other
                .padding_top
                .clone()
                .or_else(|| self.padding_top.clone()),
            padding_bottom: other
                .padding_bottom
                .clone()
                .or_else(|| self.padding_bottom.clone()),
            padding_start: other
                .padding_start
                .clone()
                .or_else(|| self.padding_start.clone()),
            padding_end: other
                .padding_end
                .clone()
                .or_else(|| self.padding_end.clone()),
            background_color: other.background_color.or(self.background_color),
            opacity: other.opacity.or(self.opacity),
            border_width: other.border_width.or(self.border_width),
            border_color: other.border_color.or(self.border_color),
            border_radius: other.border_radius.or(self.border_radius),
            elevation: other.elevation.or(self.elevation),
            transform: other.transform.or(self.transform),
            ripple_color: other.ripple_color.or(self.ripple_color),
            hit_slop: other.hit_slop.or(self.hit_slop),
            color: other.color.or(self.color),
            font_size: other.font_size.or(self.font_size),
            font_weight: other.font_weight.or(self.font_weight),
            font_style: other.font_style.or(self.font_style),
            line_height: other.line_height.or(self.line_height),
            letter_spacing: other.letter_spacing.or(self.letter_spacing),
            text_align: other.text_align.or(self.text_align),
            text_decoration: other.text_decoration.or(self.text_decoration),
            number_of_lines: other.number_of_lines.or(self.number_of_lines),
            variables: vars,
            active: other
                .active
                .as_ref()
                .map(|oa| {
                    Box::new(match &self.active {
                        Some(sa) => sa.merge(oa),
                        None => *oa.clone(),
                    })
                })
                .or_else(|| self.active.clone()),
            focus: other
                .focus
                .as_ref()
                .map(|of| {
                    Box::new(match &self.focus {
                        Some(sf) => sf.merge(of),
                        None => *of.clone(),
                    })
                })
                .or_else(|| self.focus.clone()),
            disabled: other
                .disabled
                .as_ref()
                .map(|od| {
                    Box::new(match &self.disabled {
                        Some(sd) => sd.merge(od),
                        None => *od.clone(),
                    })
                })
                .or_else(|| self.disabled.clone()),
            checked: other
                .checked
                .as_ref()
                .map(|oc| {
                    Box::new(match &self.checked {
                        Some(sc) => sc.merge(oc),
                        None => *oc.clone(),
                    })
                })
                .or_else(|| self.checked.clone()),
            dark: other
                .dark
                .as_ref()
                .map(|od| {
                    Box::new(match &self.dark {
                        Some(sd) => sd.merge(od),
                        None => *od.clone(),
                    })
                })
                .or_else(|| self.dark.clone()),
            sm: other
                .sm
                .as_ref()
                .map(|os| {
                    Box::new(match &self.sm {
                        Some(ss) => ss.merge(os),
                        None => *os.clone(),
                    })
                })
                .or_else(|| self.sm.clone()),
            md: other
                .md
                .as_ref()
                .map(|om| {
                    Box::new(match &self.md {
                        Some(sm) => sm.merge(om),
                        None => *om.clone(),
                    })
                })
                .or_else(|| self.md.clone()),
            lg: other
                .lg
                .as_ref()
                .map(|ol| {
                    Box::new(match &self.lg {
                        Some(sl) => sl.merge(ol),
                        None => *ol.clone(),
                    })
                })
                .or_else(|| self.lg.clone()),
        }
    }

    /// Converts visual appearance and typography into protocol `Property` records.
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
        if let Some((tx, ty, sx, sy, r)) = self.transform {
            props.push(Property::Transform {
                translate_x: tx,
                translate_y: ty,
                scale_x: sx,
                scale_y: sy,
                rotate_deg: r,
            });
        }
        if let Some(rc) = self.ripple_color {
            props.push(Property::RippleColor(rc));
        }
        if let Some((l, t, r, b)) = self.hit_slop {
            props.push(Property::HitSlop {
                left: l,
                top: t,
                right: r,
                bottom: b,
            });
        }

        // Padding (if non-zero points)
        let pl = match self.padding_left.as_ref().or(self.padding_start.as_ref()) {
            Some(DimensionValue::Points(p)) => *p,
            _ => 0.0,
        };
        let pt = match self.padding_top.as_ref() {
            Some(DimensionValue::Points(p)) => *p,
            _ => 0.0,
        };
        let pr = match self.padding_right.as_ref().or(self.padding_end.as_ref()) {
            Some(DimensionValue::Points(p)) => *p,
            _ => 0.0,
        };
        let pb = match self.padding_bottom.as_ref() {
            Some(DimensionValue::Points(p)) => *p,
            _ => 0.0,
        };
        if pl > 0.0 || pt > 0.0 || pr > 0.0 || pb > 0.0 {
            props.push(Property::Padding {
                left: pl,
                top: pt,
                right: pr,
                bottom: pb,
            });
        }

        props
    }

    /// Converts layout styling to `LayoutStyle` for Taffy computation.
    pub fn to_layout_style(&self) -> LayoutStyle {
        let mut ls = LayoutStyle::default();
        if let Some(d) = self.display {
            ls.display = d;
        }
        if let Some(fd) = self.flex_direction {
            ls.flex_direction = fd;
        }
        if let Some(fg) = self.flex_grow {
            ls.flex_grow = fg;
        }
        if let Some(fs) = self.flex_shrink {
            ls.flex_shrink = fs;
        }
        if let Some(ref fb) = self.flex_basis {
            ls.flex_basis = fb.clone();
        }
        if let Some(ai) = self.align_items {
            ls.align_items = Some(ai);
        }
        if let Some(as_) = self.align_self {
            ls.align_self = Some(as_);
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
        if let Some(g) = self.gap {
            ls.gap = g;
        }
        if let Some(p) = self.position {
            ls.position = p;
        }
        if let Some(ref w) = self.width {
            ls.width = w.clone();
        }
        if let Some(ref h) = self.height {
            ls.height = h.clone();
        }
        if let Some(ref mw) = self.min_width {
            ls.min_width = mw.clone();
        }
        if let Some(ref mw) = self.max_width {
            ls.max_width = mw.clone();
        }
        if let Some(ref mh) = self.min_height {
            ls.min_height = mh.clone();
        }
        if let Some(ref mh) = self.max_height {
            ls.max_height = mh.clone();
        }
        if let Some(ar) = self.aspect_ratio {
            ls.aspect_ratio = Some(ar);
        }
        if let Some(ref l) = self.left {
            ls.left = l.clone();
        }
        if let Some(ref r) = self.right {
            ls.right = r.clone();
        }
        if let Some(ref t) = self.top {
            ls.top = t.clone();
        }
        if let Some(ref b) = self.bottom {
            ls.bottom = b.clone();
        }
        if let Some(ref ml) = self.margin_left {
            ls.margin_left = ml.clone();
        }
        if let Some(ref mr) = self.margin_right {
            ls.margin_right = mr.clone();
        }
        if let Some(ref mt) = self.margin_top {
            ls.margin_top = mt.clone();
        }
        if let Some(ref mb) = self.margin_bottom {
            ls.margin_bottom = mb.clone();
        }
        if let Some(ref pl) = self.padding_left {
            ls.padding_left = pl.clone();
        }
        if let Some(ref pr) = self.padding_right {
            ls.padding_right = pr.clone();
        }
        if let Some(ref pt) = self.padding_top {
            ls.padding_top = pt.clone();
        }
        if let Some(ref pb) = self.padding_bottom {
            ls.padding_bottom = pb.clone();
        }
        ls
    }
}
