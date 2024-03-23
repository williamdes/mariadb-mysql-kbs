use crate::cleaner;
use crate::data::{KbParsedEntry, PageProcess, QueryResponse};
use select::document::Document;
use select::node::Node;
use select::predicate::{Class, Name};

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
            let text_value_default_range = row_value.trim().to_string();
            let key = text_value_default_range.split_once(":");
            let val = text_value_default_range.split_once(":");
            if key.is_some() {
                entry.r#type =
                    cleaner::get_clean_type_from_mixed_string(key.unwrap().0.trim().to_string());
                if entry.r#type.is_none() {
                    entry.r#type = None;
                }
            }
            if val.is_some() {
                entry.default = Some(cleaner::clean_default(val.unwrap().1.trim().to_string()));
                if entry.default.is_none() {
                    entry.default = None;
                }
            }
        }
        "minimum value" => {
            entry.init_range();
            match entry.range {
                Some(ref mut r) => {
                    let val = match row_node.find(Name("code")).next() {
                        Some(code_node) => code_node.text(),
                        None => row_value.trim().to_string(),
                    };
                    r.try_fill_from(val);
                }
                None => {}
            }
        }
        "maximum value" => {
            entry.init_range();
            match entry.range {
                Some(ref mut r) => {
                    let val = match row_node.find(Name("code")).next() {
                        Some(code_node) => code_node.text(),
                        None => row_value.trim().to_string(),
                    };
                    r.try_fill_to(val);
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
        has_description: false,
        is_removed: false,
        cli: None,
        default: None,
        dynamic: None,
        id: Some(find_table_archor(table_node)),
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
    use crate::data::Range;

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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--ndbcluster".to_string()),
                    default: Some("FALSE (Version: NDB 7.5-7.6)".to_string()),
                    dynamic: Some(false),
                    id: Some("option_mysqld_ndbcluster".to_string()),
                    name: Some("ndbcluster".to_string()),
                    scope: None,
                    r#type: None,
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--ndb-allow-copying-alter-table=[ON|OFF]".to_string()),
                    default: Some("ON (Version: NDB 7.5-7.6)".to_string()),
                    dynamic: Some(true),
                    id: Some("option_mysqld_ndb-allow-copying-alter-table".to_string()),
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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--binlog-gtid-simple-recovery[={OFF|ON}]".to_string()),
                    default: Some("ON".to_string()),
                    dynamic: Some(false),
                    id: Some("sysvar_binlog_gtid_simple_recovery".to_string()),
                    name: Some("binlog_gtid_simple_recovery".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("boolean".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--enforce-gtid-consistency[=value]".to_string()),
                    default: Some("OFF".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_enforce_gtid_consistency".to_string()),
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
                    has_description: false,
                    is_removed: false,
                    dynamic: Some(false),
                    id: Some("sysvar_gtid_executed".to_string()),
                    name: Some("gtid_executed".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                    cli: None,
                    default: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--gtid-executed-compression-period=#".to_string()),
                    default: Some("1000".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_gtid_executed_compression_period".to_string()),
                    name: Some("gtid_executed_compression_period".to_string()),
                    range: Some(Range {
                        to_upwards: None,
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
                    has_description: false,
                    is_removed: false,
                    cli: Some("--gtid-mode=MODE".to_string()),
                    default: Some("OFF".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_gtid_mode".to_string()),
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
                    has_description: false,
                    is_removed: false,
                    default: Some("AUTOMATIC".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_gtid_next".to_string()),
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
                    has_description: false,
                    is_removed: false,
                    dynamic: Some(false),
                    id: Some("sysvar_gtid_owned".to_string()),
                    name: Some("gtid_owned".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                    cli: None,
                    default: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    dynamic: Some(true),
                    id: Some("sysvar_gtid_purged".to_string()),
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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![KbParsedEntry {
                has_description: false,
                is_removed: false,
                cli: None,
                default: Some("TRUE (Version: 5.1.51-ndb-7.2.0)".to_string()),
                dynamic: Some(true),
                id: Some("sysvar_ndb_join_pushdown".to_string()),
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
    fn test_case_4() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_4.html"),
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndbcluster".to_string()),
                    name: Some("ndbcluster".to_string()),
                    cli: Some("--ndbcluster".to_string()),
                    dynamic: Some(false),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                    scope: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-allow-copying-alter-table".to_string()),
                    name: Some("ndb-allow-copying-alter-table".to_string()),
                    cli: Some("--ndb-allow-copying-alter-table=[ON|OFF]".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("boolean".to_string()),
                    default: Some("ON (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-batch-size".to_string()),
                    name: Some("ndb-batch-size".to_string()),
                    cli: Some("--ndb-batch-size=#".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("integer".to_string()),
                    default: Some("32768 / 0 - 31536000 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-cluster-connection-pool".to_string()),
                    name: Some("ndb-cluster-connection-pool".to_string()),
                    cli: Some("--ndb-cluster-connection-pool=#".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    default: Some("1 / 1 - 63 (Version: NDB 7.5-7.6)".to_string()),
                    valid_values: None,
                    range: None,
                    r#type: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-cluster-connection-pool-nodeids".to_string()),
                    name: Some("ndb-cluster-connection-pool-nodeids".to_string()),
                    cli: Some("--ndb-cluster-connection-pool-nodeids=list".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("set".to_string()),
                    default: Some("/ (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-blob-read-batch-bytes".to_string()),
                    name: Some("ndb-blob-read-batch-bytes".to_string()),
                    cli: Some("--ndb-blob-read-batch-bytes=bytes".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("integer".to_string()),
                    default: Some("65536 / 0 - 4294967295 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-blob-write-batch-bytes".to_string()),
                    name: Some("ndb-blob-write-batch-bytes".to_string()),
                    cli: Some("--ndb-blob-write-batch-bytes=bytes".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("integer".to_string()),
                    default: Some("65536 / 0 - 4294967295 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-connectstring".to_string()),
                    name: Some("ndb-connectstring".to_string()),
                    cli: Some("--ndb-connectstring=connection_string".to_string()),
                    dynamic: Some(false),
                    r#type: Some("string".to_string()),
                    default: Some("(Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                    scope: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-default-column-format".to_string()),
                    name: Some("ndb-default-column-format".to_string()),
                    cli: Some("--ndb-default-column-format=[FIXED|DYNAMIC]".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("enumeration".to_string()),
                    default: Some("FIXED / FIXED, DYNAMIC (Version: 7.5.4)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-deferred-constraints".to_string()),
                    name: Some("ndb-deferred-constraints".to_string()),
                    cli: Some("--ndb-deferred-constraints=[0|1]".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("integer".to_string()),
                    default: Some("0 / 0 - 1 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-distribution".to_string()),
                    name: Some("ndb-distribution".to_string()),
                    cli: Some("--ndb-distribution=[KEYHASH|LINHASH]".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("enumeration".to_string()),
                    default: Some("KEYHASH / LINHASH, KEYHASH (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-apply-status".to_string()),
                    name: Some("ndb-log-apply-status".to_string()),
                    cli: Some("--ndb-log-apply-status".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-empty-epochs".to_string()),
                    name: Some("ndb-log-empty-epochs".to_string()),
                    cli: Some("--ndb-log-empty-epochs=[ON|OFF]".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-empty-update".to_string()),
                    name: Some("ndb-log-empty-update".to_string()),
                    cli: Some("--ndb-log-empty-update=[ON|OFF]".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-exclusive-reads".to_string()),
                    name: Some("ndb-log-exclusive-reads".to_string()),
                    cli: Some("--ndb-log-exclusive-reads=[0|1]".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("boolean".to_string()),
                    default: Some("0 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-orig".to_string()),
                    name: Some("ndb-log-orig".to_string()),
                    cli: Some("--ndb-log-orig".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-transaction-id".to_string()),
                    name: Some("ndb-log-transaction-id".to_string()),
                    cli: Some("--ndb-log-transaction-id".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-log-update-minimal".to_string()),
                    name: Some("ndb-log-update-minimal".to_string()),
                    cli: Some("--ndb-log-update-minimal".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("boolean".to_string()),
                    default: Some("OFF (Version: 7.6.3)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-mgmd-host".to_string()),
                    name: Some("ndb-mgmd-host".to_string()),
                    cli: Some("--ndb-mgmd-host=host[:port]".to_string()),
                    dynamic: Some(false),
                    r#type: Some("string".to_string()),
                    default: Some("localhost:1186 (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                    scope: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-nodeid".to_string()),
                    name: Some("ndb-nodeid".to_string()),
                    cli: Some("--ndb-nodeid=#".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    default: Some("/ 1 - 255 (Version: 5.1.5)".to_string()),
                    valid_values: None,
                    range: None,
                    r#type: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-optimization-delay".to_string()),
                    name: Some("ndb-optimization-delay".to_string()),
                    cli: Some("--ndb-optimization-delay=milliseconds".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(true),
                    r#type: Some("integer".to_string()),
                    default: Some("10 / 0 - 100000 (Version: NDB 7.5-7.6)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-recv-thread-activation-threshold".to_string()),
                    name: Some("ndb-recv-thread-activation-threshold".to_string()),
                    cli: Some("--ndb-recv-thread-activation-threshold=threshold".to_string()),
                    dynamic: Some(false),
                    r#type: Some("integer".to_string()),
                    default:
                    Some("8 / 0 (MIN_ACTIVATION_THRESHOLD) - 16, (MAX_ACTIVATION_THRESHOLD) (Version: NDB 7.5-7.6)".to_string()),
                    valid_values: None,
                    range: None,
                    scope: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-recv-thread-cpu-mask".to_string()),
                    name: Some("ndb-recv-thread-cpu-mask".to_string()),
                    cli: Some("--ndb-recv-thread-cpu-mask=bitmask".to_string()),
                    dynamic: Some(false),
                    default: Some("[empty] (Version: 5.7)".to_string()),
                    valid_values: None,
                    range: None,
                    scope: None,
                    r#type: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-wait-connected".to_string()),
                    name: Some("ndb-wait-connected".to_string()),
                    cli: Some("--ndb-wait-connected=seconds".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("integer".to_string()),
                    default: Some("30 / 0 - 31536000 (Version: NDB 7.5-7.6)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_ndb-wait-setup".to_string()),
                    name: Some("ndb-wait-setup".to_string()),
                    cli: Some("--ndb-wait-setup=seconds".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    dynamic: Some(false),
                    r#type: Some("integer".to_string()),
                    default: Some("30 / 0 - 31536000 (Version: NDB 7.5-7.6)".to_string()),
                    valid_values: None,
                    range: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    id: Some("option_mysqld_skip-ndbcluster".to_string()),
                    name: Some("skip-ndbcluster".to_string()),
                    cli: Some("--skip-ndbcluster".to_string()),
                    dynamic: Some(false),
                    valid_values: None,
                    range: None,
                    scope: None,
                    r#type: None,
                    default: None,
                },
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--ndb-transid-mysql-connection-map[=state]".to_string()),
                    default: Some("ON".to_string()),
                    dynamic: None,
                    id: Some("option_mysqld_ndb-transid-mysql-connection-map".to_string()),
                    name: Some("ndb_transid_mysql_connection_map".to_string()),
                    r#type: Some("enumeration".to_string()),
                    valid_values: Some(vec!["ON".to_string(), "OFF".to_string(), "FORCE".to_string()]),
                    range: None,
                    scope: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_5() {
        let entries = extract_mysql_from_text(QueryResponse {
            body: get_test_data("mysql_test_case_5.html"),
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![KbParsedEntry {
                has_description: false,
                is_removed: false,
                id: Some("option_mysqld_mysqlx".to_string()),
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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--auto-increment-increment=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_auto_increment_increment".to_string()),
                    name: Some("auto_increment_increment".to_string()),
                    range: Some(Range {
                        to_upwards: None,
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
                    has_description: false,
                    is_removed: false,
                    cli: Some("--auto-increment-offset=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_auto_increment_offset".to_string()),
                    name: Some("auto_increment_offset".to_string()),
                    range: Some(Range {
                        to_upwards: None,
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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: false,
                    is_removed: false,
                    cli: Some("--server-id=#".to_string()),
                    default: Some("1".to_string()),
                    dynamic: Some(true),
                    id: Some("sysvar_server_id".to_string()),
                    name: Some("server_id".to_string()),
                    range: Some(Range {
                        to_upwards: None,
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
                    has_description: false,
                    is_removed: false,
                    cli: None,
                    default: None,
                    dynamic: Some(false),
                    id: Some("sysvar_server_uuid".to_string()),
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
            url: "https://example.com".to_string(),
        });
        assert_eq!(
            vec![KbParsedEntry {
                has_description: false,
                is_removed: false,
                cli: Some("--basedir=dir_name".to_string()),
                default: Some("parent of mysqld installation directory".to_string()),
                dynamic: Some(false),
                id: Some("sysvar_basedir".to_string()),
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
