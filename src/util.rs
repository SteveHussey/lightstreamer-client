/// Clean the message from newlines and carriage returns and convert it to lowercase. Also remove all brackets.
pub fn clean_message(text: &str) -> String {
    let mut result = String::new();
    let mut inside_braces = false;

    for part in text.split_inclusive(&['{', '}']) {
        if part.starts_with('{') && part.ends_with('}') {
            // Part is fully inside braces
            inside_braces = true;
            result.push_str(part);
        } else if inside_braces {
            // We're processing a segment after an opening brace
            inside_braces = false;
            result.push_str(&part);
        } else {
            // Process the part outside braces
            result.push_str(&part.replace('\n', "").replace('\r', "").to_lowercase());
        }
    }

    result
}

pub fn parse_arguments(input: &str) -> Vec<&str> {
    let mut arguments = Vec::new();
    let mut start = 0;
    let mut in_brackets = 0; // Tracks nesting level for curly braces

    for (i, c) in input.chars().enumerate() {
        match c {
            '{' => in_brackets += 1,
            '}' => in_brackets -= 1,
            ',' if in_brackets == 0 => {
                // Outside of brackets, treat comma as a delimiter
                let slice = &input[start..i].trim();
                if !slice.is_empty() {
                    arguments.push(*slice); // Dereference slice here
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    // Push the final argument if it's not empty
    if start < input.len() {
        let slice = &input[start..].trim();
        if !slice.is_empty() {
            arguments.push(*slice); // Dereference slice here
        }
    }

    arguments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_message_no_braces() {
        let input = "HELLO\nWORLD\r\nTEST";
        let result = clean_message(input);
        assert_eq!(result, "helloworldtest");
    }

    #[test]
    fn test_clean_message_with_braces() {
        let input = "hello {\"key\": \"value\"} world";
        let result = clean_message(input);
        assert_eq!(result, "hello {\"key\": \"value\"} world");
    }

    #[test]
    fn test_clean_message_braces_preserved_case() {
        let input = "HELLO {\"Key\": \"Value\"} WORLD";
        let result = clean_message(input);
        assert_eq!(result, "hello {\"key\": \"value\"} world");
    }

    #[test]
    fn test_clean_message_nested_braces() {
        let input = "a {b {c} d} e";
        let result = clean_message(input);
        assert_eq!(result, "a {b {c} d} e");
    }

    #[test]
    fn test_clean_message_empty() {
        let result = clean_message("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_message_only_newlines() {
        let result = clean_message("\n\r\n\r");
        assert_eq!(result, "");
    }

    #[test]
    fn test_clean_message_mixed_content() {
        let input = "u,1,1,{\"price\": 100}\n\r";
        let result = clean_message(input);
        assert_eq!(result, "u,1,1,{\"price\": 100}");
    }

    #[test]
    fn test_parse_arguments_simple() {
        let input = "a,b,c";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_arguments_with_brackets() {
        let input = "a{b,c},d,e{f,g}";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a{b,c}", "d", "e{f,g}"]);
    }

    #[test]
    fn test_parse_arguments_nested_brackets() {
        let input = "a{b{c,d},e},f";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a{b{c,d},e}", "f"]);
    }

    #[test]
    fn test_parse_arguments_single_value() {
        let input = "hello";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn test_parse_arguments_empty() {
        let input = "";
        let result = parse_arguments(input);
        assert_eq!(result, Vec::<&str>::new());
    }

    #[test]
    fn test_parse_arguments_with_spaces() {
        let input = " a , b , c ";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_arguments_trailing_comma() {
        let input = "a,b,";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_parse_arguments_consecutive_commas() {
        let input = "a,,b";
        let result = parse_arguments(input);
        assert_eq!(result, vec!["a", "b"]);
    }

    #[test]
    fn test_parse_arguments_complex_message() {
        let input = "u,1,1,price|volume|status";
        let args = parse_arguments(input);
        assert_eq!(args.len(), 4);
        assert_eq!(args[0], "u");
        assert_eq!(args[1], "1");
        assert_eq!(args[2], "1");
        assert_eq!(args[3], "price|volume|status");
    }

    #[test]
    fn test_parse_arguments_with_json_payload() {
        let input = "u,1,1,{\"price\":100,\"volume\":50}";
        let args = parse_arguments(input);
        assert_eq!(args[0], "u");
        assert_eq!(args[1], "1");
        assert_eq!(args[2], "1");
        assert_eq!(args[3], "{\"price\":100,\"volume\":50}");
    }
}
