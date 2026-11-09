//! Snapshot formatting for element hierarchies.

use ferrix_core::Element;
use ferrix_renderer_protocol::Property;

/// Converts an Element tree into a normalized, formatted XML/JSX-like snapshot string.
pub fn format_snapshot(element: &Element) -> String {
    let mut out = String::new();
    format_node(element, 0, &mut out);
    out
}

fn format_node(element: &Element, depth: usize, out: &mut String) {
    let indent = "  ".repeat(depth);
    let tag = format!("{:?}", element.view_type);

    let mut props_str = String::new();
    for prop in &element.props {
        match prop {
            Property::TestId(id) => props_str.push_str(&format!(" testID=\"{}\"", id)),
            Property::AccessibilityRole(role) => {
                props_str.push_str(&format!(" accessibilityRole=\"{}\"", role))
            }
            Property::AccessibilityLabel(lbl) => {
                props_str.push_str(&format!(" accessibilityLabel=\"{}\"", lbl))
            }
            _ => {}
        }
    }

    if element.children.is_empty() && element.text.is_none() {
        out.push_str(&format!("{}<{}{}/>\n", indent, tag, props_str));
    } else if element.children.is_empty() && element.text.is_some() {
        out.push_str(&format!(
            "{}<{}{}>{}</{}>\n",
            indent,
            tag,
            props_str,
            element.text.as_ref().unwrap(),
            tag
        ));
    } else {
        out.push_str(&format!("{}<{}{}>\n", indent, tag, props_str));
        if let Some(text) = &element.text {
            out.push_str(&format!("{}  {}\n", indent, text));
        }
        for child in &element.children {
            format_node(child, depth + 1, out);
        }
        out.push_str(&format!("{}</{}>\n", indent, tag));
    }
}
