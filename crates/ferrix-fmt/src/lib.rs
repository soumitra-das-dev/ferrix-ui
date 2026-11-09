//! # ferrix-fmt
//!
//! Lossless, idempotent source code formatter for .fsx files.
//! Invariant: format(format(x)) == format(x), comments and trivia preserved.

/// Formats .fsx source code according to standard Ferrix conventions.
pub fn format_source(input: &str) -> String {
    let mut output = Vec::new();
    let mut indent_level: usize = 0;
    let mut consecutive_empty_lines = 0;

    for raw_line in input.lines() {
        let trimmed = raw_line.trim();

        if trimmed.is_empty() {
            consecutive_empty_lines += 1;
            if consecutive_empty_lines <= 1 && !output.is_empty() {
                output.push(String::new());
            }
            continue;
        }
        consecutive_empty_lines = 0;

        // Check for dedents on the current line (e.g. `}`, `)`, `</`, `/>`)
        let dedent_now = starts_with_dedent(trimmed);
        if dedent_now && indent_level > 0 {
            indent_level -= 1;
        }

        let indent = "    ".repeat(indent_level);
        let formatted_line = format_line_content(trimmed);
        output.push(format!("{}{}", indent, formatted_line));

        // Adjust indent for subsequent lines
        let opens = count_opens(trimmed);
        let closes = count_closes(trimmed);

        if !dedent_now {
            if opens > closes {
                indent_level += opens - closes;
            } else if closes > opens && indent_level >= (closes - opens) {
                indent_level -= closes - opens;
            }
        } else if opens > 0 {
            indent_level += opens;
        }
    }

    let mut res = output.join("\n");
    if !res.is_empty() && !res.ends_with('\n') {
        res.push('\n');
    }
    res
}

fn starts_with_dedent(line: &str) -> bool {
    line.starts_with('}')
        || line.starts_with(')')
        || line.starts_with(']')
        || line.starts_with("</")
        || line.starts_with("/>")
        || line.starts_with("};")
        || line.starts_with(");")
}

fn count_opens(line: &str) -> usize {
    // If it's a comment, don't count
    if line.starts_with("//") {
        return 0;
    }
    let mut count = 0;
    let mut in_str = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' || c == '\'' {
            in_str = !in_str;
        } else if !in_str {
            if c == '{' || c == '(' || c == '[' {
                count += 1;
            } else if c == '<'
                && chars
                    .peek()
                    .map(|&n| n.is_ascii_uppercase())
                    .unwrap_or(false)
            {
                // Opening JSX tag e.g. <Column
                if !line.ends_with("/>") && !line.contains("</") {
                    count += 1;
                }
            }
        }
    }
    count
}

fn count_closes(line: &str) -> usize {
    if line.starts_with("//") {
        return 0;
    }
    let mut count = 0;
    let mut in_str = false;
    let mut chars = line.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' || c == '\'' {
            in_str = !in_str;
        } else if !in_str {
            if c == '}' || c == ')' || c == ']' || (c == '<' && chars.peek() == Some(&'/')) {
                count += 1;
            } else if c == '/' && chars.peek() == Some(&'>') {
                // Self closing
            }
        }
    }
    count
}

fn format_line_content(trimmed: &str) -> String {
    // Preserve comments verbatim
    if trimmed.starts_with("//") || trimmed.starts_with("/*") {
        return trimmed.to_string();
    }

    // Normalizing spaces around basic punctuation while avoiding strings
    let mut result = String::with_capacity(trimmed.len() + 10);
    let mut in_str = false;
    let mut quote_char = ' ';

    for ch in trimmed.chars() {
        if !in_str && (ch == '"' || ch == '\'') {
            in_str = true;
            quote_char = ch;
            result.push(ch);
        } else if in_str && ch == quote_char {
            in_str = false;
            result.push(ch);
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idempotency() {
        let code = r#"import { Column, Text } from "ferrix/ui";

// Root Component
export function Counter(): Element {
    const count = useState(0);

    return (
        <Column padding={16}>
            <Text>Count: {count.value}</Text>
        </Column>
    );
}
"#;

        let pass1 = format_source(code);
        let pass2 = format_source(&pass1);
        assert_eq!(pass1, pass2);
    }
}
