use regex::Regex;

const REAL_TYPES: [&str; 9] = [
    "string",
    "boolean",
    "integer",
    "numeric",
    "enumeration",
    "set",
    "directory name",
    "file name",
    "byte",
];

/**
 * Clean type using real types
 * @return Option<String> The cleaned type
 */
pub fn clean_type(type_str: String) -> Option<String> {
    if REAL_TYPES.into_iter().find(|t| t.to_string() == type_str) == None {
        if type_str.contains("in bytes")
            || type_str.contains("number of bytes")
            || type_str.contains("size in mb")
        {
            return Some("byte".to_string());
        } else if type_str.contains("number of")
            || type_str.contains("size of")
            || type_str.contains("in microseconds")
            || type_str.contains("in seconds")
        {
            return Some("integer".to_string());
        } else if type_str.contains("numeric (64-bit unsigned integer)")
            || type_str.contains("numeric (32-bit unsigned integer)")
        {
            return Some("numeric".to_string());
        } else if
        //enumerated
        type_str.contains("enum") {
            return Some("enumeration".to_string());
        }

        return None;
    }
    Some(type_str)
}

pub fn get_clean_type_from_mixed_string(mixed_string: String) -> Option<&'static str> {
    REAL_TYPES
        .into_iter()
        .find(|real_type_to_test| mixed_string.contains(real_type_to_test))
}

/**
 * Clean cli argument
 * @param String cli The command line string
 * @param bool skipRegex Skip regex check
 * @return Option<String> The cleaned cli
 */
pub fn clean_cli(mut cli: String, skip_regex: bool) -> Option<String> {
    if cli.contains("<code>") || cli.contains("</code>") {
        cli = cli.replace("<code>", "");
        cli = cli.replace("</code>", "");
        cli = cli.replace(">", "");
        cli = cli.replace("<", "");
    }

    let regex_cli = Regex::new(r"(?i)([-]{2})([0-9a-z-_]+)").expect("regex should compile");
    if skip_regex == false && regex_cli.is_match(&cli) == false {
        return None;
    }

    Some(cli)
}

/**
 * Clean the range object
 * @param {Object} range The range object
 * @return {Object} The cleaned range object
 */
pub fn clean_range(range: Option<String>) -> Option<String> {
    if range != None {
        // clean range
        // TODO: re-implement
        /*if (typeof range.from != "number" || isNaN(range.from)) {
            delete range.from;
        }
        if (typeof range.to == "string" && range.to.is_match(/upwards/i)) {
            range.to = "upwards";
        } else if (typeof range.to != "number" || isNaN(range.to)) {
            delete range.to;
        }*/
    }
    return range;
}

/**
 * Clean a default value
 * @param String defaultValue The default value
 * @return String The same or an alternative formated text
 */
pub fn clean_default(default_value: String) -> String {
    let values: Vec<String> = default_value
        .split("\n")
        .map(|el| clean_text_default(el.to_string().trim().to_string()))
        .collect();
    return values.join(", ");
}

/**
 * Clean text of a default value
 * @param String default_text_value The default text value
 * @return String The same or an alternative text
 */
pub fn clean_text_default(default_text_value: String) -> String {
    if default_text_value == "Autosized (see description)" {
        return "(autosized)".to_string();
    }
    if default_text_value.contains("Based on the number of processors") {
        return "(based on the number of processors)".to_string();
    }
    if default_text_value == "The MariaDB data directory" {
        return "(the MariaDB data directory)".to_string();
    }
    if default_text_value.contains("-1 (signifies (autoscaling); do not assign this literal value)")
    {
        return "(-1 signifies autoscaling; do not use -1)".to_string();
    }
    if default_text_value.contains("-1 (signifies (autosizing); do not assign this literal value)")
    {
        return "(-1 signifies autosizing; do not use -1)".to_string();
    }
    return default_text_value;
}

/**
 * Clean text of a valid values list
 */
pub fn clean_text_valid_values(valid_values_text: String) -> String {
    if Regex::new(r"^See .* for the full list\.$")
        .expect("regex should compile")
        .is_match(&valid_values_text)
    {
        return "".to_string();
    }
    if Regex::new(r"^.* or .*$")
        .expect("regex should compile")
        .is_match(&valid_values_text)
    {
        return valid_values_text.replace(" or ", ",");
    }
    if valid_values_text == "See description" {
        return "".to_string();
    }
    return valid_values_text;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn clean_cli_html_code() {
        let cli = clean_cli("<code>--test-argument</code>".to_string(), false);
        assert_eq!(cli, Some("--test-argument".to_string()));
    }

    #[test]
    fn clean_cli_html_code_not_closed() {
        let cli = clean_cli("<code>--test-argument".to_string(), false);
        assert_eq!(cli, Some("--test-argument".to_string()));
    }

    #[test]
    fn clean_cli_nothing_to_clean() {
        let cli = clean_cli("--test-argument".to_string(), false);
        assert_eq!(cli, Some("--test-argument".to_string()));
    }

    #[test]
    fn clean_cli_undefined() {
        let cli = clean_cli("".to_string(), false);
        assert_eq!(cli, None);
    }
    /*

    #[test]
    fn clean_range_undefined() {
        const range = cleaner.cleanRange(undefined);
        expect(range).to.deep.equal(undefined);
    }

    #[test]
    fn clean_range_from_typeof object (dataset-1)() {
        const range = cleaner.cleanRange({
            from: null,
            to: null,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-2)() {
        const range = cleaner.cleanRange({
            to: null,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-3)() {
        const range = cleaner.cleanRange({
            from: null,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-4)() {
        const range = cleaner.cleanRange({
            from: undefined,
            to: undefined,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-5)() {
        const range = cleaner.cleanRange({
            to: undefined,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-6)() {
        const range = cleaner.cleanRange({
            from: undefined,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof object (dataset-7)() {
        const range = cleaner.cleanRange({
            from: NaN,
            to: NaN,
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.from typeof int() {
        const range = cleaner.cleanRange({
            from: 1024,
        });
        expect(range).to.deep.equal({
            from: 1024,
        });
    }

    #[test]
    fn clean range.from typeof string() {
        const range = cleaner.cleanRange({
            from: "1024",
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.to typeof int() {
        const range = cleaner.cleanRange({
            to: 1024,
        });
        expect(range).to.deep.equal({
            to: 1024,
        });
    }

    #[test]
    fn clean range.to typeof string() {
        const range = cleaner.cleanRange({
            to: "1024",
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range.to typeof object() {
        const range = cleaner.cleanRange({
            to: {},
        });
        expect(range).to.deep.equal({});
    }

    #[test]
    fn clean range to upwards() {
        const range = cleaner.cleanRange({
            to: "upwards",
        });
        expect(range).to.deep.equal({
            to: "upwards",
        });
    }

    #[test]
    fn clean range to upwards match() {
        const range = cleaner.cleanRange({
            to: "(128KB) upwards",
        });
        expect(range).to.deep.equal({
            to: "upwards",
        });
    }*/

    #[test]
    fn clean_binary_types_in_bytes() {
        let type_str = clean_type("in bytes".to_string());
        assert_eq!(type_str, Some("byte".to_string()));
    }

    #[test]
    fn clean_binary_types_size_in_mb() {
        let type_str = clean_type("size in mb".to_string());
        assert_eq!(type_str, Some("byte".to_string()));
    }

    #[test]
    fn clean_binary_types_number_of_bytes() {
        let type_str = clean_type("number of bytes".to_string());
        assert_eq!(type_str, Some("byte".to_string()));
    }

    #[test]
    fn clean_binary_types_number_of() {
        let type_str = clean_type("number of".to_string());
        assert_eq!(type_str, Some("integer".to_string()));
    }

    #[test]
    fn clean_binary_types_size_of() {
        let type_str = clean_type("size of".to_string());
        assert_eq!(type_str, Some("integer".to_string()));
    }

    #[test]
    fn clean_binary_types_in_microseconds() {
        let type_str = clean_type("in microseconds".to_string());
        assert_eq!(type_str, Some("integer".to_string()));
    }

    #[test]
    fn clean_binary_types_in_seconds() {
        let type_str = clean_type("in seconds".to_string());
        assert_eq!(type_str, Some("integer".to_string()));
    }

    #[test]
    fn clean_wtf_type() {
        let type_str = clean_type("wtf".to_string());
        assert_eq!(type_str, None);
    }

    #[test]
    fn clean_enumeration_type() {
        let type_str = clean_type("enumeration".to_string());
        assert_eq!(type_str, Some("enumeration".to_string()));
    }

    #[test]
    fn clean_undefined_type() {
        let type_str = clean_type("undefined".to_string());
        assert_eq!(type_str, None);
    }

    #[test]
    fn clean_enumerated_type() {
        let type_str = clean_type("enumerated".to_string());
        assert_eq!(type_str, Some("enumeration".to_string()));
    }

    #[test]
    fn get_clean_type_from_a_mixed_string_dataset_1() {
        let found_type = get_clean_type_from_mixed_string("boolean: ON (Version: 5.7)".to_string());
        assert_eq!(found_type, Some("boolean"));
    }

    #[test]
    fn get_clean_type_from_a_mixed_string_dataset_2() {
        let found_type = get_clean_type_from_mixed_string("numeric: 15".to_string());
        assert_eq!(found_type, Some("numeric"));
    }

    #[test]
    fn get_clean_text_vie_valid_values_non_valid_value_dataset_1() {
        let cleaned_value = clean_text_valid_values("See description".to_string());
        assert_eq!(cleaned_value, "");
    }

    #[test]
    fn get_clean_text_vie_valid_values_non_valid_value_dataset_2() {
        let cleaned_value =
            clean_text_valid_values("See alter_algorithm for the full list.".to_string());
        assert_eq!(cleaned_value, "");
    }

    #[test]
    fn get_clean_text_vie_valid_values_non_valid_value_dataset_3() {
        let cleaned_value = clean_text_valid_values("See OLD Mode for the full list.".to_string());
        assert_eq!(cleaned_value, "");
    }

    #[test]
    fn get_clean_text_vie_valid_values_non_valid_value_dataset_4() {
        let cleaned_value = clean_text_valid_values("0 or 1".to_string());
        assert_eq!(cleaned_value, "0,1");
    }

    #[test]
    fn get_clean_text_vie_valid_values_non_valid_value_dataset_5() {
        let cleaned_value = clean_text_valid_values("\"\" or \"uncompressed\"".to_string());
        assert_eq!(cleaned_value, "\"\",\"uncompressed\"");
    }
}
