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
    if type_str == "bool" {
        return Some("boolean".to_string());
    } else if type_str == "varchar" || type_str == "text" {
        return Some("string".to_string());
    } else if type_str == "filename" {
        return Some("file name".to_string());
    } else if type_str == "double" {
        return Some("numeric".to_string());
    } else if type_str == "ip address" {
        return Some("string".to_string());
    } else if type_str == "datetime" {
        return Some("string".to_string());
    }

    if REAL_TYPES.into_iter().find(|t| t.to_string() == type_str) == None {
        if type_str.contains("in bytes")
            || type_str.contains("number of bytes")
            || type_str.contains("size in mb")
            || type_str.contains("bytes read from")
            || type_str.contains("bytes written to")
        {
            return Some("byte".to_string());
        } else if type_str.contains("number of")
            || type_str.contains("size of")
            || type_str.contains("batch size")
            || type_str.contains("in microseconds")
            || type_str.contains("in seconds")
        {
            return Some("integer".to_string());
        } else if type_str.contains("numeric (64-bit unsigned integer)")
            || type_str.contains("numeric (32-bit unsigned integer)")
        {
            return Some("numeric".to_string());
        } else if type_str.contains("enum") {
            //enumerated
            return Some("enumeration".to_string());
        } else if type_str.contains("directory name")
            || type_str.contains("path name")
            || type_str.contains("path to")
            || type_str.ends_with("directory.")
        {
            return Some("directory name".to_string());
        } else if type_str.contains("filename") {
            return Some("file name".to_string());
        } else if type_str.ends_with("unused.") || type_str.contains("unused since") {
            return None;
        } else if type_str.ends_with("removed.") {
            return None;
        }

        if type_str.len() < 30 && type_str.len() > 0 {
            println!("not known type: {}", type_str);
        }

        return None;
    }
    Some(type_str)
}

pub fn get_clean_type_from_mixed_string(mixed_string: String) -> Option<String> {
    match REAL_TYPES
        .into_iter()
        .find(|real_type_to_test| mixed_string.contains(real_type_to_test))
    {
        Some(val) => Some(val.to_string()),
        None => None,
    }
}

const REGEX_CLI: &str = r"(?i)([-]{2})([0-9a-z-_]+)";

pub fn transform_cli_into_name(cli: String) -> Option<String> {
    let regex_cli = Regex::new(REGEX_CLI).expect("regex should compile");

    let matches = regex_cli.captures(&cli);
    match matches {
        Some(cap) => Some(cap.get(2).unwrap().as_str().replace("-", "_")),
        None => None,
    }
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

    let regex_cli = Regex::new(REGEX_CLI).expect("regex should compile");
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
 * Clean range to from values
 */
pub fn clean_range_from_to(default_text_value: String) -> String {
    if default_text_value.contains(" (log file block size)") {
        return default_text_value
            .replace(" (log file block size)", "")
            .trim()
            .to_string();
    }
    if default_text_value.contains(" (MIN_ACTIVATION_THRESHOLD)") {
        return default_text_value
            .replace(" (MIN_ACTIVATION_THRESHOLD)", "")
            .trim()
            .to_string();
    }
    if default_text_value.contains(" (MAX_ACTIVATION_THRESHOLD)") {
        return default_text_value
            .replace(" (MAX_ACTIVATION_THRESHOLD)", "")
            .trim()
            .to_string();
    }
    if default_text_value.contains('(') && default_text_value.contains(')') {
        println!("dtv: {}", default_text_value);
    }
    return default_text_value.trim().to_string();
}

/**
 * Determine if the default value should be extracted from code
 */
pub fn is_valid_default(text_value: &str) -> bool {
    let regex_with_comment = Regex::new(r".^* \([a-z0-9A-Z -]+\)$").expect("regex should compile");
    let regex_space = Regex::new(r": [0-9]+$").expect("regex should compile");
    return regex_with_comment.is_match(&text_value) || regex_space.is_match(&text_value);
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
    fn test_transform_cli_into_name() {
        let cli = transform_cli_into_name("--test-argument".to_string());
        assert_eq!(cli, Some("test_argument".to_string()));
    }

    #[test]
    fn test_transform_cli_into_name_invalid_1() {
        let cli = transform_cli_into_name("test-argument".to_string());
        assert_eq!(cli, None);
    }

    #[test]
    fn test_transform_cli_into_name_invalid_2() {
        let cli = transform_cli_into_name("".to_string());
        assert_eq!(cli, None);
    }

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
    fn clean_type_bool() {
        let type_str = clean_type("bool".to_string());
        assert_eq!(type_str, Some("boolean".to_string()));
    }

    #[test]
    fn clean_type_varchar() {
        let type_str = clean_type("varchar".to_string());
        assert_eq!(type_str, Some("string".to_string()));
    }

    #[test]
    fn clean_type_text() {
        let type_str = clean_type("text".to_string());
        assert_eq!(type_str, Some("string".to_string()));
    }

    #[test]
    fn clean_type_filename() {
        let type_str = clean_type("filename".to_string());
        assert_eq!(type_str, Some("file name".to_string()));
        let type_str = clean_type("wsrep status output filename".to_string());
        assert_eq!(type_str, Some("file name".to_string()));
    }

    #[test]
    fn clean_type_directory_name_s() {
        let type_str = clean_type("directory name/s".to_string());
        assert_eq!(type_str, Some("directory name".to_string()));
    }

    #[test]
    fn clean_type_path_name() {
        let type_str = clean_type("path name".to_string());
        assert_eq!(type_str, Some("directory name".to_string()));
    }

    #[test]
    fn clean_type_batch_size() {
        let type_str = clean_type("insert batch size.".to_string());
        assert_eq!(type_str, Some("integer".to_string()));
    }

    #[test]
    fn clean_type_datetime() {
        let type_str = clean_type("datetime".to_string());
        assert_eq!(type_str, Some("string".to_string()));
    }

    #[test]
    fn clean_type_double() {
        let type_str = clean_type("double".to_string());
        assert_eq!(type_str, Some("numeric".to_string()));
    }

    #[test]
    fn clean_type_ip_address() {
        let type_str = clean_type("ip address".to_string());
        assert_eq!(type_str, Some("string".to_string()));
    }

    #[test]
    fn clean_type_bytes_from_to() {
        let type_str = clean_type("bytes read from block cache.".to_string());
        assert_eq!(type_str, Some("byte".to_string()));
        let type_str = clean_type("bytes written to block cache.".to_string());
        assert_eq!(type_str, Some("byte".to_string()));
    }

    #[test]
    fn clean_datetime_type() {
        let type_str = clean_type("datetime".to_string());
        assert_eq!(type_str, Some("string".to_string()));
    }

    #[test]
    fn clean_removed_type() {
        let type_str = clean_type("removed.".to_string());
        assert_eq!(type_str, None);
    }

    #[test]
    fn clean_unused_type() {
        let type_str = clean_type("unused.".to_string());
        assert_eq!(type_str, None);
        let type_str = clean_type("unused since 10.1.4".to_string());
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
        assert_eq!(found_type, Some("boolean".to_string()));
    }

    #[test]
    fn get_clean_type_from_a_mixed_string_dataset_2() {
        let found_type = get_clean_type_from_mixed_string("numeric: 15".to_string());
        assert_eq!(found_type, Some("numeric".to_string()));
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

    #[test]
    fn clean_range_from_to_dataset_1() {
        let cleaned_value = clean_range_from_to("512 (log file block size)".to_string());
        assert_eq!(cleaned_value, "512");
    }

    #[test]
    fn clean_range_from_to_trim_dataset_2() {
        let cleaned_value = clean_range_from_to(" 512 (log file block size)".to_string());
        assert_eq!(cleaned_value, "512");
    }

    #[test]
    fn clean_range_from_to_trim_dataset_3() {
        let cleaned_value = clean_range_from_to("0 (MIN_ACTIVATION_THRESHOLD)".to_string());
        assert_eq!(cleaned_value, "0");
    }

    #[test]
    fn clean_range_from_to_trim_dataset_4() {
        let cleaned_value = clean_range_from_to("16 (MAX_ACTIVATION_THRESHOLD)".to_string());
        assert_eq!(cleaned_value, "16");
    }

    #[test]
    fn is_valid_default_dataset_1() {
        let is_valid = is_valid_default("512 (log file block size)");
        assert_eq!(is_valid, true);
    }

    #[test]
    fn is_valid_default_dataset_2() {
        let is_valid = is_valid_default(": 100");
        assert_eq!(is_valid, true);
    }

    #[test]
    fn is_valid_default_dataset_3() {
        let is_valid = is_valid_default("Depends on the system. Often /usr/share/cracklib/pw_dict");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn is_valid_default_dataset_4() {
        let is_valid = is_valid_default("Empty, previously 0.0.0.0");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn is_valid_default_dataset_5() {
        let is_valid = is_valid_default("NULL (>= MariaDB 10.2.2), . (<= MariaDB 10.2.1)");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn is_valid_default_dataset_6() {
        let is_valid = is_valid_default("134217728 (128M)");
        assert_eq!(is_valid, true);
    }

    #[test]
    fn is_valid_default_dataset_7() {
        let is_valid = is_valid_default("The lower of 900 and (50 + max_connections/5)");
        assert_eq!(is_valid, false);
    }

    #[test]
    fn is_valid_default_dataset_8() {
        let is_valid = is_valid_default("0 (non-segmented)");
        assert_eq!(is_valid, true);
    }
}
