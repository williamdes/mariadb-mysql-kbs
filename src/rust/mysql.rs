use crate::cleaner;
use crate::data::{KbParsedEntry, PageProcess, QueryResponse, Range};
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

/**
 * Complete a doc element with info found in table
 * @param {HTMLTableRowElement[]} rows The table rows
 * @param {Object} doc The doc object
 *
function completeDoc($, rows, doc) {
    $(rows).each((i, elem) => {
        let tds = $(elem).find("td"); // first is key and last is value
        var name = tds.first().text().toLowerCase().trim();
        var value = tds.last();
        let ths = $(elem).find("th"); // Fallback if the key is in a th
        if (ths.length > 0) {
            name = ths.first().text().toLowerCase().trim();
        }
    });
}*/

/**
 * Create a doc element
 * @param {Element} element The root element
 * @returns object The doc object
 */
/*
function createDoc($, element, doc) {
    completeDoc($, $(element).find("tbody > tr"), doc);
    if (doc.range !== undefined) {
        doc.range = cleaner.cleanRange(doc.range);
    }

    if (doc.name && doc.name.match(cleaner.regexCli)) {
        delete doc.name;
    }

    return doc;
}*/

fn find_table_archor(node: Node) -> String {
    let mut collected_p_nodes: Vec<Node> = vec![];
    let mut node_count = 10;
    let mut node_cur: Option<Node> = Some(node);

    loop {
        // Current node is None exit
        if node_cur.is_none() {
            break;
        }
        // Move cursor to previous and bump count
        node_cur = node_cur.unwrap().prev();
        node_count = node_count - 1;
        // If still is None or count too low exit
        if node_cur.is_none() || node_count < 1 {
            break;
        }

        let n = node_cur.unwrap();
        if n.is(Name("p")) {
            collected_p_nodes.push(n);
        }
    }

    let anchor_name_node = collected_p_nodes
        .iter()
        .filter(|el| el.find(Name("a")).next().is_some())
        .map(|el| el.find(Name("a")).next().unwrap())
        .find(|el| el.attr("name").is_some() && el.attr("class").is_none());

    match anchor_name_node {
        Some(node) => node.attr("name").unwrap().to_string(),
        None => node
            .parent()
            .expect("Has a parent")
            .find(Class("link"))
            .next()
            .unwrap()
            .attr("href")
            .expect("Missing href attr")
            .split("#")
            .last()
            .expect("Anchor to have #")
            .to_string(),
    }
}

fn process_row_to_entry(
    row_name: String,
    row_node: Node,
    mut entry: KbParsedEntry,
    table_node: Node,
) -> KbParsedEntry {
    let row_value = row_node.text();
    match row_name.as_str() {
        "dynamic" => entry.dynamic = Some(row_value.to_lowercase().trim() == "yes"),
        "name" => entry.name = Some(row_value.trim().to_string()),
        "system variable" => {
            // Do not overwrite the name
            if entry.name.is_none() {
                entry.name = Some(row_value.to_lowercase().trim().to_string())
            }
        }
        "type" => {
            entry.r#type = Some(row_value.to_lowercase().trim().to_string());

            if entry.r#type != Some("".to_string()) {
                entry.r#type = cleaner::clean_type(entry.r#type.unwrap());
            }
            if entry.r#type == Some("".to_string()) {
                entry.r#type = None;
            }
        }
        "command-line format" => {
            entry.cli = cleaner::clean_cli(row_value.trim().to_string(), false);
        }
        "default value" | "default, range" => {
            entry.default = Some(cleaner::clean_default(row_value.trim().to_string()));
        }
        "valid values" => {
            let mut values = vec![];
            for code_node in row_node.find(Name("code")) {
                values.push(code_node.text());
            }
            entry.valid_values = Some(values);
        }
        "type: default, range" => {
            /*
            let textValueDefaultRange = value.text().trim();
            let key = textValueDefaultRange.substring(0, textValueDefaultRange.indexOf(":") + 1);
            let val = textValueDefaultRange.substring(textValueDefaultRange.indexOf(":") + 1);
            if (typeof key === "string") {
                doc.type = cleaner.getCleanTypeFromMixedString(key);
                if (typeof doc.type !== "string") {
                    delete doc.type;
                }
            }
            if (typeof val === "string") {
                doc.default = cleaner.cleanDefault(val);
                if (typeof doc.default !== "string") {
                    delete doc.default;
                }
            } */
        }
        "minimum value" => {
            if entry.range.is_none() {
                entry.range = Some(Range {
                    from: None,
                    to: None,
                    from_f: None,
                    to_f: None,
                });
            }
            match entry.range {
                Some(ref mut r) => {
                    let val = match row_node.find(Name("code")).next() {
                        Some(code_node) => code_node.text(),
                        None => row_value.trim().to_string(),
                    };
                    match val.parse::<i128>() {
                        Ok(v) => r.from = Some(v),
                        _ => match val.parse::<f64>() {
                            Ok(v) => r.from_f = Some(v),
                            _ => {}
                        },
                    }
                }
                None => {}
            }
        }
        "maximum value" => {
            if entry.range.is_none() {
                entry.range = Some(Range {
                    from: None,
                    to: None,
                    from_f: None,
                    to_f: None,
                });
            }
            match entry.range {
                Some(ref mut r) => {
                    let val = match row_node.find(Name("code")).next() {
                        Some(code_node) => code_node.text(),
                        None => row_value.trim().to_string(),
                    };
                    match val.parse::<i128>() {
                        Ok(v) => r.to = Some(v),
                        _ => match val.parse::<f64>() {
                            Ok(v) => r.to_f = Some(v),
                            _ => {}
                        },
                    }
                }
                None => {}
            }
        }

        "scope" => {
            let scope = row_value.to_lowercase().trim().to_string();
            if scope == "both" {
                // found on mysql-cluster-options-variables.html
                entry.scope = Some(vec!["global".to_string(), "session".to_string()]);
            } else if scope != "" {
                let values: Vec<String> = scope
                    .split(",")
                    .map(|item| {
                        if item.contains("session") {
                            return "session".to_string();
                        } else if item.contains("global") {
                            return "global".to_string();
                        }

                        return item.trim().to_string();
                    })
                    .collect();
                entry.scope = Some(values);
            }
            if entry.scope.is_some() {
                // TODO: cleanup scope
                //entry.scope = entry.scope.filter(|e| e == "0" || e.is_some());
            }
        }
        "command line" => {
            // Boolean (Yes)
            if row_value.to_lowercase().trim().to_string() == "yes" {
                let link_cli_code_child_cli = table_node
                    .parent()
                    .expect("Has a parent")
                    .find(Name("a"))
                    .filter(|e| e.attr("class").is_some() && e.attr("class").unwrap() == "link")
                    .next();
                if link_cli_code_child_cli.is_some() {
                    entry.cli = cleaner::clean_cli(
                        link_cli_code_child_cli.expect("Has one link").text(),
                        false,
                    );
                }

                if link_cli_code_child_cli.is_none() || entry.cli.is_none() {
                    let code_child_cli = table_node
                        .parent()
                        .expect("Has a parent")
                        .find(Name("code"))
                        .filter(|e| {
                            e.attr("class").is_some() && e.attr("class").unwrap() == "option"
                        })
                        .next();
                    if code_child_cli.is_some() {
                        entry.cli = cleaner::clean_cli(
                            code_child_cli.expect("Has one code tag").text(),
                            false,
                        );
                    }
                }
            }
        }
        _key => {
            //println!("tr: {} -> {}", row_name, row_value);
            //println!("missing: {}", key);
        }
    }

    entry
}

fn process_table(table_node: Node) -> KbParsedEntry {
    let mut entry = KbParsedEntry {
        cli: None,
        default: None,
        dynamic: None,
        id: find_table_archor(table_node),
        name: None,
        scope: None,
        r#type: None,
        valid_values: None,
        range: None,
    };

    for tbody in table_node.find(Name("tbody")) {
        for tr in tbody.find(Name("tr")) {
            match tr.find(Name("td")).into_selection().len() == 1
                && tr.find(Name("th")).into_selection().len() == 1
            {
                // It is a mix of a th for the header and a td for the data
                true => {
                    let row_name: String = tr
                        .find(Name("th"))
                        .next()
                        .expect("Node to exist")
                        .text()
                        .to_lowercase()
                        .trim()
                        .to_owned();
                    let row_value: Node = tr.find(Name("td")).next().expect("Node to exist");
                    entry = process_row_to_entry(row_name, row_value, entry, table_node);
                }
                false => {
                    let mut tds = tr.find(Name("td"));
                    let row_name: String = tds
                        .next()
                        .expect("Node to exist")
                        .text()
                        .to_lowercase()
                        .trim()
                        .to_owned();
                    let row_value: Node = tds.next().expect("Node to exist");
                    entry = process_row_to_entry(row_name, row_value, entry, table_node);
                }
            }
        }
    }

    if entry.name.is_none() && entry.cli.is_some() {
        entry.name = cleaner::transform_cli_into_name(entry.cli.as_ref().unwrap().to_string());
    }

    /*
    var name = tds.first().text().toLowerCase().trim();
    var value = tds.last();
    let ths = $(elem).find("th"); // Fallback if the key is in a th
    if (ths.length > 0) {
        name = ths.first().text().toLowerCase().trim();
    }*/
    entry
}

fn filter_table(elem: &Node) -> bool {
    let element_attr = elem.attr("class");
    match element_attr {
        Some(attr) => match attr == "informaltable" {
            true => match elem.find(Name("table")).next() {
                Some(table) => match table.attr("summary") {
                    Some(attr) => attr.contains("Properties for"),
                    None => false,
                },
                None => false,
            },
            false => match elem.find(Name("th")).next() {
                Some(e) => e.text() == "Property",
                None => false,
            },
        },
        None => false,
    }
}

pub fn extract_mysql_from_text(qr: QueryResponse) -> Vec<KbParsedEntry> {
    let document = Document::from(qr.body.as_str());

    document
        .find(Class("table"))
        .chain(document.find(Class("informaltable")))
        .filter(|elem| filter_table(elem))
        .map(|table_node| process_table(table_node))
        .collect()
}

/*
        .each(function (i, elem) {
            let doc = {
                id:
            createDoc($, elem, doc);
            if (typeof doc.cli === "boolean") {
                doc.cli = $(elem).prevAll().find(".option").first().text();
                if (doc.cli === "") {
                    delete doc.cli;
                }
            }
            anchors.push(doc);
        });

    cbSuccess(anchors);
}*/

const KB_URL: &str = "https://dev.mysql.com/doc/refman/8.0/en/";
const KB_URL57: &str = "https://dev.mysql.com/doc/refman/5.7/en/";

pub fn get_pages() -> Vec<PageProcess<'static>> {
    [
        PageProcess {
            url: KB_URL.to_owned() + "server-system-variables.html",
            name: "server-system-variables".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "innodb-parameters.html",
            name: "innodb-parameters".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "performance-schema-system-variables.html",
            name: "performance-schema-system-variables".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "x-plugin-options-system-variables.html",
            name: "x-plugin-options-system-variables".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "replication-options-binary-log.html",
            name: "replication-options-binary-log".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL57.to_owned() + "replication-options-binary-log.html",
            name: "replication-options-binary-log_5.7".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "pluggable-authentication-system-variables.html",
            name: "pluggable-authentication-system-variables".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "audit-log-reference.html",
            name: "audit-log-reference".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "replication-options-gtids.html",
            name: "replication-options-gtids".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "replication-options-replica.html",
            name: "replication-options-replica".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "replication-options-source.html",
            name: "replication-options-source".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "replication-options.html",
            name: "replication-options".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL57.to_owned() + "mysql-cluster-options-variables.html",
            name: "mysql-cluster-options-variables".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "server-options.html",
            name: "server-options".to_string(),
            data_type: "variables",
        },
        PageProcess {
            url: KB_URL.to_owned() + "version-tokens-reference.html",
            name: "version-tokens-reference".to_string(),
            data_type: "variables",
        },
    ]
    .to_vec()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq;
    use std::env;
    use std::fs;

    fn get_test_data(file_name: &str) -> String {
        let test_dir = env::current_dir().unwrap();
        fs::read_to_string(test_dir.to_str().unwrap().to_owned() + "/src/rust/data/" + file_name)
            .expect("Should have been able to read the test data file")
    }

    #[test]
    fn test_case_1() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_1.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    cli: Some("--ndbcluster".to_string()),
                    default: Some("FALSE (Version: NDB 7.5-7.6)".to_string()),
                    dynamic: Some(false),
                    id: "option_mysqld_ndbcluster".to_string(),
                    name: Some("ndbcluster".to_string()),
                    scope: None,
                    r#type: None,
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    cli: Some("--ndb-allow-copying-alter-table=[ON|OFF]".to_string()),
                    default: Some("ON (Version: NDB 7.5-7.6)".to_string()),
                    dynamic: Some(true),
                    id: "option_mysqld_ndb-allow-copying-alter-table".to_string(),
                    name: Some("ndb-allow-copying-alter-table".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: None,
                    valid_values: None,
                    range: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_2() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_2.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    cli: Some("--binlog-gtid-simple-recovery[={OFF|ON}]".to_string()),
                    default: Some("ON".to_string()),
                    dynamic: Some(false),
                    id: "sysvar_binlog_gtid_simple_recovery".to_string(),
                    name: Some("binlog_gtid_simple_recovery".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("boolean".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    cli: Some("--enforce-gtid-consistency[=value]".to_string()),
                    default: Some("OFF".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_enforce_gtid_consistency".to_string(),
                    name: Some("enforce_gtid_consistency".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("enumeration".to_string()),
                    valid_values: Some(vec![
                        "OFF".to_string(),
                        "ON".to_string(),
                        "WARN".to_string()
                    ]),
                    range: None,
                },
                KbParsedEntry {
                    dynamic: Some(false),
                    id: "sysvar_gtid_executed".to_string(),
                    name: Some("gtid_executed".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                    cli: None,
                    default: None,
                },
                KbParsedEntry {
                    cli: Some("--gtid-executed-compression-period=#".to_string()),
                    default: Some("1000".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_gtid_executed_compression_period".to_string(),
                    name: Some("gtid_executed_compression_period".to_string()),
                    range: Some(Range {
                        from: Some(0),
                        to: Some(4294967295),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: None,
                },
                KbParsedEntry {
                    cli: Some("--gtid-mode=MODE".to_string()),
                    default: Some("OFF".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_gtid_mode".to_string(),
                    name: Some("gtid_mode".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("enumeration".to_string()),
                    valid_values: Some(vec![
                        "OFF".to_string(),
                        "OFF_PERMISSIVE".to_string(),
                        "ON_PERMISSIVE".to_string(),
                        "ON".to_string()
                    ]),
                    range: None,
                },
                KbParsedEntry {
                    default: Some("AUTOMATIC".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_gtid_next".to_string(),
                    name: Some("gtid_next".to_string()),
                    scope: Some(vec!["session".to_string()]),
                    r#type: Some("enumeration".to_string()),
                    valid_values: Some(vec![
                        "AUTOMATIC".to_string(),
                        "ANONYMOUS".to_string(),
                        "UUID:NUMBER".to_string()
                    ]),
                    range: None,
                    cli: None,
                },
                KbParsedEntry {
                    dynamic: Some(false),
                    id: "sysvar_gtid_owned".to_string(),
                    name: Some("gtid_owned".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                    cli: None,
                    default: None,
                },
                KbParsedEntry {
                    dynamic: Some(true),
                    id: "sysvar_gtid_purged".to_string(),
                    name: Some("gtid_purged".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                    cli: None,
                    default: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_3() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_3.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![KbParsedEntry {
                cli: None,
                default: Some("TRUE (Version: 5.1.51-ndb-7.2.0)".to_string()),
                dynamic: Some(true),
                id: "sysvar_ndb_join_pushdown".to_string(),
                name: Some("ndb_join_pushdown".to_string()),
                scope: Some(vec!["global".to_string(), "session".to_string()]),
                r#type: None,
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_5() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_5.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![KbParsedEntry {
                id: "option_mysqld_mysqlx".to_string(),
                cli: Some("--mysqlx[=value]".to_string()),
                r#type: Some("enumeration".to_string()),
                default: Some("ON".to_string()),
                valid_values: Some(vec![
                    "ON".to_string(),
                    "OFF".to_string(),
                    "FORCE".to_string(),
                    "FORCE_PLUS_PERMANENT".to_string(),
                ]),
                name: Some("mysqlx".to_string()),
                scope: None,
                range: None,
                dynamic: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_6() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_6.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    cli: Some("--auto-increment-increment=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_auto_increment_increment".to_string(),
                    name: Some("auto_increment_increment".to_string()),
                    range: Some(Range {
                        from: Some(1),
                        to: Some(65535),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: None,
                },
                KbParsedEntry {
                    cli: Some("--auto-increment-offset=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_auto_increment_offset".to_string(),
                    name: Some("auto_increment_offset".to_string()),
                    range: Some(Range {
                        from: Some(1),
                        to: Some(65535),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_7() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_7.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    cli: Some("--server-id=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: "sysvar_server_id".to_string(),
                    name: Some("server_id".to_string()),
                    range: Some(Range {
                        from: Some(0),
                        to: Some(4294967295),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: None,
                },
                KbParsedEntry {
                    cli: None,
                    default: None,
                    dynamic: Some(false),
                    id: "sysvar_server_uuid".to_string(),
                    name: Some("server_uuid".to_string()),
                    range: None,
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_8() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_8.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![KbParsedEntry {
                cli: Some("--basedir=dir_name".to_string()),
                default: Some("parent of mysqld installation directory".to_string()),
                dynamic: Some(false),
                id: "sysvar_basedir".to_string(),
                name: Some("basedir".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("directory name".to_string()),
                valid_values: None,
                range: None,
            }],
            entries
        );
    }
}
