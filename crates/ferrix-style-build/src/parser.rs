//! CSS subset parser evaluating selectors, at-rules, shorthands, and native layout properties.

use std::collections::HashMap;

use ferrix_layout::{
    AlignItems, AlignSelf, DimensionValue, Display, FlexDirection, FlexWrap, JustifyContent,
    Position,
};
use ferrix_style::{CascadeRule, RuleSource, Specificity, StyleIr};

use crate::diagnostics::StyleDiagnostic;
use crate::shorthands::{
    expand_border, expand_border_radius, expand_flex, expand_gap, expand_margin, expand_padding,
};
use crate::values::{parse_color, parse_dimension, parse_font_size};

/// Parses CSS source code into a list of `CascadeRule`s and `:root` variables.
pub struct CssParser<'a> {
    source: &'a str,
    file_name: &'a str,
    rules: Vec<CascadeRule>,
    variables: HashMap<String, u32>,
    diagnostics: Vec<StyleDiagnostic>,
    source_order: u32,
}

pub type ParseResult = Result<(Vec<CascadeRule>, HashMap<String, u32>), Vec<StyleDiagnostic>>;

impl<'a> CssParser<'a> {
    pub fn new(source: &'a str, file_name: &'a str) -> Self {
        Self {
            source,
            file_name,
            rules: Vec::new(),
            variables: HashMap::new(),
            diagnostics: Vec::new(),
            source_order: 0,
        }
    }

    pub fn parse(mut self, default_source: RuleSource) -> ParseResult {
        let clean = self.strip_comments(self.source);
        self.parse_blocks(&clean, default_source, 0, None, None);

        let errors: Vec<StyleDiagnostic> = self
            .diagnostics
            .iter()
            .filter(|d| d.severity == crate::diagnostics::DiagnosticSeverity::Error)
            .cloned()
            .collect();

        if !errors.is_empty() {
            Err(self.diagnostics)
        } else {
            Ok((self.rules, self.variables))
        }
    }

    fn strip_comments(&self, input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '/' && chars.peek() == Some(&'*') {
                chars.next(); // consume '*'
                while let Some(c2) = chars.next() {
                    if c2 == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // consume '/'
                        break;
                    }
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    fn parse_blocks(
        &mut self,
        input: &str,
        default_source: RuleSource,
        layer: u32,
        media_dark: Option<bool>,
        media_bp: Option<&str>,
    ) {
        let mut pos = 0;
        let bytes = input.as_bytes();

        while pos < bytes.len() {
            while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if pos >= bytes.len() {
                break;
            }

            // Check for at-rules (@media, @layer, @import)
            if bytes[pos] == b'@' {
                let start = pos;
                while pos < bytes.len() && bytes[pos] != b'{' && bytes[pos] != b';' {
                    pos += 1;
                }
                if pos >= bytes.len() {
                    break;
                }
                let at_header = input[start..pos].trim();
                if bytes[pos] == b';' {
                    // e.g. @import "..."
                    pos += 1;
                    continue;
                }
                pos += 1; // consume '{'

                // Find matching closing brace
                let block_start = pos;
                let mut brace_depth = 1;
                while pos < bytes.len() && brace_depth > 0 {
                    if bytes[pos] == b'{' {
                        brace_depth += 1;
                    } else if bytes[pos] == b'}' {
                        brace_depth -= 1;
                    }
                    pos += 1;
                }
                let inner_block = &input[block_start..pos - 1];

                if at_header.starts_with("@media") {
                    let is_dark = at_header.contains("prefers-color-scheme: dark");
                    let bp = if at_header.contains("min-width: 640") {
                        Some("sm")
                    } else if at_header.contains("min-width: 768") {
                        Some("md")
                    } else if at_header.contains("min-width: 1024") {
                        Some("lg")
                    } else {
                        None
                    };
                    self.parse_blocks(
                        inner_block,
                        default_source,
                        layer,
                        if is_dark { Some(true) } else { media_dark },
                        bp.or(media_bp),
                    );
                } else if at_header.starts_with("@layer") {
                    self.parse_blocks(inner_block, default_source, layer + 1, media_dark, media_bp);
                } else {
                    self.parse_blocks(inner_block, default_source, layer, media_dark, media_bp);
                }
                continue;
            }

            // Normal CSS rule: selector { ... }
            let sel_start = pos;
            while pos < bytes.len() && bytes[pos] != b'{' {
                pos += 1;
            }
            if pos >= bytes.len() {
                break;
            }
            let selector_str = input[sel_start..pos].trim();
            pos += 1; // consume '{'

            let body_start = pos;
            let mut brace_depth = 1;
            while pos < bytes.len() && brace_depth > 0 {
                if bytes[pos] == b'{' {
                    brace_depth += 1;
                } else if bytes[pos] == b'}' {
                    brace_depth -= 1;
                }
                pos += 1;
            }
            let body_str = &input[body_start..pos - 1];

            // Calculate line and col for diagnostics
            let (line, col) = self.get_line_col(sel_start);

            // Check for unsupported pseudo-elements
            if selector_str.contains("::before") || selector_str.contains("::after") {
                self.diagnostics.push(StyleDiagnostic::error(
                    "Pseudo-elements '::before' and '::after' are not supported in native mobile layouts",
                    self.file_name,
                    line,
                    col,
                ));
                continue;
            }

            // Check for unsupported web pseudo-classes
            if selector_str.contains(":hover")
                || selector_str.contains(":visited")
                || selector_str.contains(":target")
            {
                self.diagnostics.push(StyleDiagnostic::error(
                    "Web pseudo-classes ':hover', ':visited', ':target' are not supported on touch interfaces",
                    self.file_name,
                    line,
                    col,
                ));
                continue;
            }

            // Handle :root variables
            if selector_str == ":root" {
                self.parse_root_variables(body_str);
                continue;
            }

            // Parse declarations
            let mut style = self.parse_declarations(body_str, line);

            // Apply pseudo-class states (e.g. .btn:active)
            let (clean_selector, pseudo_active, pseudo_focus, pseudo_disabled, pseudo_checked) =
                self.extract_pseudo_classes(selector_str);

            if pseudo_active {
                let mut base = StyleIr::new();
                base.active = Some(Box::new(style.clone()));
                style = base;
            }
            if pseudo_focus {
                let mut base = StyleIr::new();
                base.focus = Some(Box::new(style.clone()));
                style = base;
            }
            if pseudo_disabled {
                let mut base = StyleIr::new();
                base.disabled = Some(Box::new(style.clone()));
                style = base;
            }
            if pseudo_checked {
                let mut base = StyleIr::new();
                base.checked = Some(Box::new(style.clone()));
                style = base;
            }

            // Wrap in media query variant if applicable
            if media_dark == Some(true) {
                let mut base = StyleIr::new();
                base.dark = Some(Box::new(style));
                style = base;
            }
            if let Some(bp) = media_bp {
                let mut base = StyleIr::new();
                match bp {
                    "sm" => base.sm = Some(Box::new(style)),
                    "md" => base.md = Some(Box::new(style)),
                    "lg" => base.lg = Some(Box::new(style)),
                    _ => {}
                }
                style = base;
            }

            self.source_order += 1;
            let specificity = self.calculate_specificity(&clean_selector);
            let rule = CascadeRule::new(
                clean_selector,
                default_source,
                specificity,
                self.source_order,
                style,
            )
            .with_layer(layer);
            self.rules.push(rule);
        }
    }

    fn extract_pseudo_classes(&self, sel: &str) -> (String, bool, bool, bool, bool) {
        let mut clean = sel.to_string();
        let mut active = false;
        let mut focus = false;
        let mut disabled = false;
        let mut checked = false;

        if clean.contains(":active") {
            clean = clean.replace(":active", "");
            active = true;
        }
        if clean.contains(":focus") {
            clean = clean.replace(":focus", "");
            focus = true;
        }
        if clean.contains(":disabled") {
            clean = clean.replace(":disabled", "");
            disabled = true;
        }
        if clean.contains(":checked") {
            clean = clean.replace(":checked", "");
            checked = true;
        }

        (clean.trim().to_string(), active, focus, disabled, checked)
    }

    fn parse_root_variables(&mut self, body: &str) {
        for decl in body.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }
            if let Some((name, val)) = decl.split_once(':') {
                let name = name.trim();
                let val = val.trim();
                if name.starts_with("--") {
                    if let Some(color) = parse_color(val) {
                        self.variables.insert(name.to_string(), color);
                    }
                }
            }
        }
    }

    fn parse_declarations(&mut self, body: &str, base_line: usize) -> StyleIr {
        let mut style = StyleIr::new();

        for decl in body.split(';') {
            let decl = decl.trim();
            if decl.is_empty() {
                continue;
            }
            let (prop, val) = match decl.split_once(':') {
                Some((p, v)) => (p.trim().to_lowercase(), v.trim()),
                None => continue,
            };

            // Check for unsupported properties
            match prop.as_str() {
                "float" | "clear" | "cursor" | "columns" | "column-count" | "table" | "filter"
                | "white-space" => {
                    self.diagnostics.push(StyleDiagnostic::error(
                        format!(
                            "CSS property '{}' is not supported in native mobile layouts",
                            prop
                        ),
                        self.file_name,
                        base_line,
                        1,
                    ));
                    continue;
                }
                _ => {}
            }

            // Handle CSS variables inside values: var(--accent)
            if val.starts_with("var(") && val.ends_with(')') {
                let inner = &val[4..val.len() - 1].trim();
                let var_name = inner.split(',').next().unwrap_or("").trim();
                style
                    .variables
                    .insert(var_name.to_string(), val.to_string());
                continue;
            }

            // Shorthands
            match prop.as_str() {
                "margin" => expand_margin(val, &mut style),
                "padding" => expand_padding(val, &mut style),
                "border" => expand_border(val, &mut style),
                "border-radius" => expand_border_radius(val, &mut style),
                "gap" => expand_gap(val, &mut style),
                "flex" => expand_flex(val, &mut style),

                // Longhand layout & flexbox
                "flex-direction" => {
                    style.flex_direction = match val {
                        "row" => Some(FlexDirection::Row),
                        "row-reverse" => Some(FlexDirection::RowReverse),
                        "column-reverse" => Some(FlexDirection::ColumnReverse),
                        _ => Some(FlexDirection::Column),
                    };
                }
                "flex-grow" => style.flex_grow = val.parse::<f32>().ok(),
                "flex-shrink" => style.flex_shrink = val.parse::<f32>().ok(),
                "flex-basis" => style.flex_basis = parse_dimension(val),
                "align-items" => {
                    style.align_items = match val {
                        "flex-start" => Some(AlignItems::FlexStart),
                        "center" => Some(AlignItems::Center),
                        "flex-end" => Some(AlignItems::FlexEnd),
                        "stretch" => Some(AlignItems::Stretch),
                        "baseline" => Some(AlignItems::Baseline),
                        _ => None,
                    };
                }
                "align-self" => {
                    style.align_self = match val {
                        "flex-start" => Some(AlignSelf::FlexStart),
                        "center" => Some(AlignSelf::Center),
                        "flex-end" => Some(AlignSelf::FlexEnd),
                        "stretch" => Some(AlignSelf::Stretch),
                        "baseline" => Some(AlignSelf::Baseline),
                        _ => None,
                    };
                }
                "justify-content" => {
                    style.justify_content = match val {
                        "flex-start" => Some(JustifyContent::FlexStart),
                        "center" => Some(JustifyContent::Center),
                        "flex-end" => Some(JustifyContent::FlexEnd),
                        "space-between" => Some(JustifyContent::SpaceBetween),
                        "space-around" => Some(JustifyContent::SpaceAround),
                        "space-evenly" => Some(JustifyContent::SpaceEvenly),
                        _ => None,
                    };
                }
                "flex-wrap" => {
                    style.flex_wrap = match val {
                        "wrap" => Some(FlexWrap::Wrap),
                        "wrap-reverse" => Some(FlexWrap::WrapReverse),
                        _ => Some(FlexWrap::NoWrap),
                    };
                }
                "display" => {
                    style.display = match val {
                        "none" => Some(Display::None),
                        _ => Some(Display::Flex),
                    };
                }
                "position" => {
                    style.position = match val {
                        "absolute" => Some(Position::Absolute),
                        _ => Some(Position::Relative),
                    };
                }
                "width" => style.width = parse_dimension(val),
                "height" => style.height = parse_dimension(val),
                "min-width" => style.min_width = parse_dimension(val),
                "max-width" => style.max_width = parse_dimension(val),
                "min-height" => style.min_height = parse_dimension(val),
                "max-height" => style.max_height = parse_dimension(val),
                "aspect-ratio" => style.aspect_ratio = val.parse::<f32>().ok(),
                "top" => style.top = parse_dimension(val),
                "bottom" => style.bottom = parse_dimension(val),
                "left" => style.left = parse_dimension(val),
                "right" => style.right = parse_dimension(val),

                // Visual appearance
                "background-color" | "background" => style.background_color = parse_color(val),
                "color" => style.color = parse_color(val),
                "opacity" => style.opacity = val.parse::<f32>().ok(),
                "border-color" => style.border_color = parse_color(val),
                "border-width" => {
                    if let Some(DimensionValue::Points(w)) = parse_dimension(val) {
                        style.border_width = Some((w, w, w, w));
                    }
                }
                "elevation" | "box-shadow" => style.elevation = val.parse::<f32>().ok(),

                // Typography
                "font-size" => style.font_size = parse_font_size(val),
                "font-weight" => {
                    style.font_weight = match val {
                        "bold" => Some(700),
                        "normal" => Some(400),
                        _ => val.parse::<u16>().ok(),
                    };
                }
                "font-style" => {
                    style.font_style = match val {
                        "italic" => Some(1),
                        _ => Some(0),
                    };
                }
                "line-height" => style.line_height = parse_font_size(val),
                "letter-spacing" => style.letter_spacing = parse_font_size(val),
                "text-align" => {
                    style.text_align = match val {
                        "center" => Some(1),
                        "right" => Some(2),
                        "justify" => Some(3),
                        _ => Some(0),
                    };
                }
                "text-decoration" => {
                    style.text_decoration = match val {
                        "underline" => Some(1),
                        "line-through" => Some(2),
                        _ => Some(0),
                    };
                }
                _ => {}
            }
        }

        style
    }

    fn calculate_specificity(&self, selector: &str) -> Specificity {
        if selector == "*" {
            return Specificity::UNIVERSAL;
        }

        let mut b = 0;
        let mut c = 0;

        for part in selector.split_whitespace() {
            if part.starts_with('.') {
                b += part.matches('.').count() as u32;
            } else if !part.starts_with(':') && !part.starts_with('#') {
                c += 1;
            }
        }

        Specificity::new(0, b, c)
    }

    fn get_line_col(&self, byte_offset: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (i, c) in self.source.chars().enumerate() {
            if i >= byte_offset {
                break;
            }
            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
}
