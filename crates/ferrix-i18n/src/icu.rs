//! ICU Message Format parser and renderer.

use std::collections::HashMap;

/// Renders an ICU message pattern using the provided variable arguments.
pub fn format_message(pattern: &str, params: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = pattern.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut block = String::new();
            let mut depth = 1;
            for c in chars.by_ref() {
                if c == '{' {
                    depth += 1;
                    block.push(c);
                } else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    block.push(c);
                } else {
                    block.push(c);
                }
            }

            let rendered = format_icu_block(&block, params);
            result.push_str(&rendered);
        } else {
            result.push(ch);
        }
    }

    result
}

fn format_icu_block(block: &str, params: &HashMap<String, String>) -> String {
    let parts: Vec<&str> = block.splitn(3, ',').map(|s| s.trim()).collect();

    if parts.len() == 1 {
        // Simple variable interpolation: {name}
        let var_name = parts[0];
        return params
            .get(var_name)
            .cloned()
            .unwrap_or_else(|| format!("{{{}}}", var_name));
    }

    let var_name = parts[0];
    let format_type = parts[1];
    let raw_options = if parts.len() > 2 { parts[2] } else { "" };

    let value = params.get(var_name).map(|s| s.as_str()).unwrap_or("");

    match format_type {
        "plural" => {
            let num = value.parse::<i64>().unwrap_or(0);
            let options = parse_sub_options(raw_options);

            // Determine plural category
            let category = if num == 0 && options.contains_key("zero") {
                "zero"
            } else if num == 1 && options.contains_key("one") {
                "one"
            } else if num == 2 && options.contains_key("two") {
                "two"
            } else {
                "other"
            };

            let template = options
                .get(category)
                .or_else(|| options.get("other"))
                .cloned()
                .unwrap_or_default();
            // Replace '#' with the number
            template.replace('#', &num.to_string())
        }
        "select" => {
            let options = parse_sub_options(raw_options);
            options
                .get(value)
                .or_else(|| options.get("other"))
                .cloned()
                .unwrap_or_default()
        }
        _ => format!("{{{}}}", block),
    }
}

fn parse_sub_options(raw: &str) -> HashMap<String, String> {
    let mut options = HashMap::new();
    let mut chars = raw.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }

        // Read branch key (e.g. "one", "other", "male")
        let mut key = String::new();
        key.push(ch);
        while let Some(&next) = chars.peek() {
            if next == '{' || next.is_whitespace() {
                break;
            }
            key.push(chars.next().unwrap());
        }

        // Skip to '{'
        while let Some(&next) = chars.peek() {
            if next == '{' {
                chars.next();
                break;
            }
            chars.next();
        }

        // Read content until matching '}'
        let mut content = String::new();
        let mut depth = 1;
        for c in chars.by_ref() {
            if c == '{' {
                depth += 1;
                content.push(c);
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    break;
                }
                content.push(c);
            } else {
                content.push(c);
            }
        }

        options.insert(key.trim().to_string(), content);
    }

    options
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interpolation() {
        let mut params = HashMap::new();
        params.insert("name".to_string(), "Soumo".to_string());
        let res = format_message("Hello, {name}!", &params);
        assert_eq!(res, "Hello, Soumo!");
    }

    #[test]
    fn test_plural_formatting() {
        let mut params = HashMap::new();
        params.insert("count".to_string(), "1".to_string());
        let pattern = "You have {count, plural, zero {no items} one {# item} other {# items}}.";
        assert_eq!(format_message(pattern, &params), "You have 1 item.");

        params.insert("count".to_string(), "5".to_string());
        assert_eq!(format_message(pattern, &params), "You have 5 items.");

        params.insert("count".to_string(), "0".to_string());
        assert_eq!(format_message(pattern, &params), "You have no items.");
    }

    #[test]
    fn test_select_formatting() {
        let mut params = HashMap::new();
        params.insert("gender".to_string(), "female".to_string());
        let pattern = "{gender, select, male {He} female {She} other {They}} liked this.";
        assert_eq!(format_message(pattern, &params), "She liked this.");

        params.insert("gender".to_string(), "unknown".to_string());
        assert_eq!(format_message(pattern, &params), "They liked this.");
    }
}
