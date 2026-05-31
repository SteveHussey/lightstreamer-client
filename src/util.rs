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

/// Redacts the contents of the specified fields within a LightStreamer encoded message with "****".
///
/// The `encoded` parameter is expected to be a URL-encoded query string (e.g.,
/// `LS_adapter=DEFAULT&LS_user=john&LS_password=secret`). The function replaces the
/// values of any fields listed in `fields` with `****`.
///
/// This is primarily used to safely log messages that may contain sensitive information
/// such as passwords or credentials.
///
/// # Examples
///
/// ```
/// use lightstreamer_client::util::redact_message_fields;
///
/// let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
/// let redacted = redact_message_fields(input, vec!["LS_password"]);
/// assert_eq!(redacted, "LS_adapter=DEFAULT&LS_user=john&LS_password=****");
/// ```
pub fn redact_message_fields(encoded: &str, fields: Vec<&str>) -> String {
    if fields.is_empty() {
        return encoded.to_string();
    }

    let mut result = String::new();
    let pairs: Vec<&str> = encoded.split('&').collect();

    for (i, pair) in pairs.iter().enumerate() {
        if let Some((key, _)) = pair.split_once('=') {
            if fields.contains(&key) {
                if i > 0 {
                    result.push('&');
                }
                result.push_str(key);
                result.push('=');
                result.push_str("****");
            } else {
                if i > 0 {
                    result.push('&');
                }
                result.push_str(pair);
            }
        } else {
            if i > 0 {
                result.push('&');
            }
            result.push_str(pair);
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

    #[test]
    fn test_redact_single_field() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_user=john&LS_password=****");
    }

    #[test]
    fn test_redact_multiple_fields() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_user", "LS_password"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_user=****&LS_password=****");
    }

    #[test]
    fn test_redact_no_match_unchanged() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_token"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_user=john&LS_password=secret");
    }

    #[test]
    fn test_redact_empty_fields_unchanged() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, Vec::<&str>::new());
        assert_eq!(result, "LS_adapter=DEFAULT&LS_user=john&LS_password=secret");
    }

    #[test]
    fn test_redact_empty_input_unchanged() {
        let result = redact_message_fields("", vec!["LS_password"]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_redact_first_field() {
        let input = "LS_password=secret&LS_adapter=DEFAULT";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_password=****&LS_adapter=DEFAULT");
    }

    #[test]
    fn test_redact_middle_field() {
        let input = "LS_adapter=DEFAULT&LS_password=secret&LS_user=john";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_password=****&LS_user=john");
    }

    #[test]
    fn test_redact_last_field() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_user=john&LS_password=****");
    }

    #[test]
    fn test_redact_empty_value() {
        let input = "LS_adapter=DEFAULT&LS_password=&LS_user=john";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_password=****&LS_user=john");
    }

    #[test]
    fn test_redact_special_chars_in_value() {
        let input = "LS_adapter=DEFAULT&LS_user=john%40example.com&LS_password=p%40ss";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(
            result,
            "LS_adapter=DEFAULT&LS_user=john%40example.com&LS_password=****"
        );
    }

    #[test]
    fn test_redact_all_fields() {
        let input = "LS_adapter=DEFAULT&LS_user=john&LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_adapter", "LS_user", "LS_password"]);
        assert_eq!(result, "LS_adapter=****&LS_user=****&LS_password=****");
    }

    #[test]
    fn test_redact_single_pair() {
        let input = "LS_password=secret";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(result, "LS_password=****");
    }

    #[test]
    fn test_redact_duplicate_field_keys() {
        let input = "LS_adapter=DEFAULT&LS_password=secret1&LS_password=secret2";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(
            result,
            "LS_adapter=DEFAULT&LS_password=****&LS_password=****"
        );
    }

    #[test]
    fn test_redact_value_with_equals() {
        let input = "LS_adapter=DEFAULT&LS_token=a=b=c";
        let result = redact_message_fields(input, vec!["LS_token"]);
        assert_eq!(result, "LS_adapter=DEFAULT&LS_token=****");
    }

    #[test]
    fn test_redact_case_sensitive() {
        let input = "LS_adapter=DEFAULT&LS_password=secret&ls_password=visible";
        let result = redact_message_fields(input, vec!["LS_password"]);
        assert_eq!(
            result,
            "LS_adapter=DEFAULT&LS_password=****&ls_password=visible"
        );
    }
}
