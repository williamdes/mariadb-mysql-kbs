use select::{
    document::Document,
    node::Node,
    predicate::{Class, Name},
};

use crate::{
    cleaner,
    data::{KbParsedEntry, Page, PageProcess, QueryResponse},
};

const KB_URL: &str = "https://mariadb.com/kb/en/library/documentation/";
const STORAGE_ENGINES: [&str; 7] = [
    "s3-storage-engine",
    "aria",
    "myrocks",
    "cassandra",
    "galera-cluster",
    "myisam",
    "connect",
];
const PLUGINS: [&str; 13] = [
    "mariadb-audit-plugin",
    "semisynchronous-replication-plugin",
    "sql-error-log-plugin",
    "authentication-plugin-gssapi",
    "authentication-plugin-pam",
    "aws-key-management-encryption-plugin",
    "cracklib-password-check-plugin",
    "disks-plugin",
    "feedback-plugin",
    "file-key-management-encryption-plugin",
    "query-cache-information-plugin",
    "query-response-time-plugin",
    "simple-password-check-plugin",
];

const SYSTEM_VARIABLES: [&str; 7] = [
    "pbxt",
    "mroonga",
    "tokudb",
    "xtradbinnodb-server",
    "mariadb-audit-plugin",
    "ssltls",
    "performance-schema",
];

const CUSTOM_PAGES: [Page; 9] = [
    Page {
        url: "columns-storage-engines-and-plugins/storage-engines/spider/spider-server-system-variables/",
        name: "spider-server-system-variables",
    },
    Page {
        url: "semisynchronous-replication/",
        name: "semisynchronous-replication-system-variables",
    },
    Page {
        url: "replication-and-binary-log-server-system-variables/",
        name: "replication-and-binary-log-server-system-variables",
    },
    Page {
        url: "gtid/",
        name: "gtid-system-variables",
    },
    Page {
        url: "replication/optimization-and-tuning/system-variables/server-system-variables/",
        name: "server-system-variables",
    },
    Page {
        url: "system-versioned-tables/",
        name: "versioned-tables-system-variables",
    },
    Page {
        url: "handlersocket-configuration-options/",
        name: "handlersocket-configuration-options-variables",
    },
    Page {
        url: "storage-engine-independent-column-compression/",
        name: "storage-engine-independent-column-compression-variables",
    },
    Page {
        url: "user-statistics/",
        name: "user-statistics-variables",
    },
];

const STATUS: [&str; 15] = [
    "server",
    "galera-cluster",
    "aria-server",
    "cassandra",
    "mroonga",
    "spider-server",
    "sphinx",
    "tokudb",
    "xtradbinnodb-server",
    "replication-and-binary-log",
    "oqgraph-system-and",
    "thread-pool-system-and",
    "ssltls",
    "performance-schema",
    "myrocks",
];

pub fn get_pages() -> Vec<PageProcess<'static>> {
    let mut pages = vec![];

    for se in &STORAGE_ENGINES {
        pages.push(PageProcess {
            url: KB_URL.to_owned()
                + "columns-storage-engines-and-plugins/storage-engines/"
                + se
                + "/"
                + se
                + "-system-variables/",
            name: format!("{}-system-variables", se),
            data_type: "variables",
        });
    }

    for cu in &CUSTOM_PAGES {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + cu.url,
            name: cu.name.to_string(),
            data_type: "variables",
        });
    }

    for status_name in &STATUS {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + status_name + "-status-variables/",
            name: format!("{}-status-variables", status_name),
            data_type: "variables",
        });
    }

    for system_variable_name in &SYSTEM_VARIABLES {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + system_variable_name + "-system-variables/",
            name: format!("{}-system-variables", system_variable_name),
            data_type: "variables",
        });
    }

    for plugin_name in &PLUGINS {
        pages.push(PageProcess {
            url: KB_URL.to_owned() + plugin_name + "/",
            name: format!("{}-variables", plugin_name),
            data_type: "variables",
        });
    }

    pages
}

fn process_li(mut entry: KbParsedEntry, li_node: Node) -> KbParsedEntry {
    let mut key_name: String = li_node
        .find(Name("strong"))
        .next()
        .expect("li to have strong")
        .text();
    let mut row_value: String = li_node.text();
    row_value = row_value
        .split_once(key_name.as_str())
        .expect("It splits")
        .1
        .trim()
        .to_string();

    key_name = key_name.to_lowercase().replace(":", "").trim().to_string();

    match key_name.as_str() {
        "dynamic" => {
            entry.dynamic = Some(row_value.to_lowercase() == "yes");
        }
        "data type" | "type" => {
            if li_node.find(Name("code")).count() == 1 {
                entry.r#type = Some(
                    li_node
                        .find(Name("code"))
                        .next()
                        .unwrap()
                        .text()
                        .to_lowercase()
                        .trim()
                        .to_string(),
                );
            } else {
                entry.r#type = Some(row_value.to_lowercase().trim().to_string());
            }

            if entry.r#type == Some("number".to_string()) {
                entry.r#type = Some("integer".to_string());
            }

            if entry.r#type != Some("".to_string()) {
                entry.r#type = cleaner::clean_type(entry.r#type.unwrap());
            }
            if entry.r#type == Some("".to_string()) {
                entry.r#type = None;
            }
            if entry.r#type == Some("numeric".to_string()) {
                entry.r#type = Some("integer".to_string());
            }
        }
        "default value" | "default" => {
            if li_node.find(Name("code")).count() == 1 {
                entry.default = Some(cleaner::clean_default(
                    li_node
                        .find(Name("code"))
                        .next()
                        .unwrap()
                        .text()
                        .trim()
                        .to_string(),
                ));
            } else {
                entry.default = Some(cleaner::clean_default(row_value));
            }
        }
        "commandline" => {
            if row_value.to_lowercase() != "no"
                && row_value.to_lowercase() != "none"
                && row_value.to_lowercase() != "n/a"
                && row_value.to_lowercase() != "no commandline option"
            {
                entry.cli = cleaner::clean_cli(row_value, true);
            }
        }

        "scope" => {
            let scope = row_value.to_lowercase().trim().to_string();
            if scope != "" {
                let values: Vec<String> = scope
                    .split(",")
                    .map(|item| item.to_lowercase())
                    .filter(|item| item.contains("session") || item.contains("global"))
                    .map(|item| {
                        if item.contains("session") {
                            return "session".to_string();
                        } else if item.contains("global") {
                            return "global".to_string();
                        }

                        return "".to_string();
                    })
                    .collect();
                entry.scope = Some(values);
            }
            if entry.scope.is_some() {
                // TODO: cleanup scope
                //entry.scope = entry.scope.filter(|e| e == "0" || e.is_some());
            }
        }
        "valid values" => {
            if li_node.find(Name("code")).next().is_some() {
                let mut values = vec![];
                for code_node in li_node.find(Name("code")) {
                    values.push(code_node.text());
                }
                entry.valid_values = Some(values);
            } else {
                let clean_value = cleaner::clean_text_valid_values(row_value.trim().to_string());
                if clean_value != "" {
                    entry.valid_values = Some(
                        clean_value
                            .split(',')
                            .map(|el| el.trim().to_string())
                            .collect(),
                    );
                }
            }
        }
        "range" => {
            if li_node.find(Name("code")).next().is_some() {
                let mut values = vec![];
                for code_node in li_node.find(Name("code")).filter(|e| e.text().trim() != "") {
                    values.push(code_node.text());
                }
                if values.len() == 1 {
                    let first_value = values.first().expect("Should have a first value");
                    if first_value.contains('-') {
                        // try x-y
                        entry.init_range();
                        match entry.range {
                            Some(ref mut r) => {
                                let range = first_value.split_once('-').unwrap();

                                r.try_fill_from(range.0.to_string());
                                r.try_fill_to(range.1.to_string());
                            }
                            None => {}
                        }
                    }
                    if first_value.contains("to") {
                        // try x to y
                        entry.init_range();
                        match entry.range {
                            Some(ref mut r) => {
                                let range = first_value.split_once("to").unwrap();

                                r.try_fill_from(range.0.to_string());
                                r.try_fill_to(range.1.to_string());
                            }
                            None => {}
                        }
                    }
                    if li_node.text().contains("upwards") {
                        // try x upwards
                        entry.init_range();
                        match entry.range {
                            Some(ref mut r) => {
                                r.try_fill_from(first_value.to_string());
                                r.to_upwards = Some("upwards".to_string());
                            }
                            None => {}
                        }
                    }
                } else if values.len() == 2 {
                    entry.init_range();
                    match entry.range {
                        Some(ref mut r) => {
                            r.try_fill_from(values.first().unwrap().to_string());
                            r.try_fill_to(values.last().unwrap().to_string());
                        }
                        None => {}
                    }
                } else if values.len() == 4 {
                    // from <code>0</code> to <code>16</code> (version x.y.z)
                    // from <code>0</code> to <code>10</code> (version a.b.c)

                    // "from" values are equal
                    if values.first() == values.get(2) {
                        entry.init_range();
                        match entry.range {
                            Some(ref mut r) => {
                                r.try_fill_from(values.first().unwrap().to_string());
                            }
                            None => {}
                        }
                    }

                    // "to" values are equal
                    if values.last() == values.get(1) {
                        entry.init_range();
                        match entry.range {
                            Some(ref mut r) => {
                                r.try_fill_to(values.last().unwrap().to_string());
                            }
                            None => {}
                        }
                    }
                } else {
                    println!("range: {}", values.len());
                }
            }
        }
        "description" => {
            entry.has_description = true;

            if entry.r#type.is_none() {
                entry.r#type = cleaner::clean_type(row_value.to_lowercase());
            }
        }
        _key => {
            //println!("{} '{}' -> '{}'", li_node.html(), key_name, row_value);
            //println!("tr: {} -> {}", row_name, row_value);
            //println!("missing: {}", key);
        }
    }

    entry
}

fn process_ul(mut entry: KbParsedEntry, ul_node: Node) -> KbParsedEntry {
    for li in ul_node.find(Name("li")) {
        if li.find(Name("strong")).next().is_some() {
            entry = process_li(entry, li)
        }
    }

    entry
}

fn process_block(header_node: Node) -> KbParsedEntry {
    let mut entry = KbParsedEntry {
        has_description: false,
        cli: None,
        default: None,
        dynamic: None,
        id: header_node.attr("id").unwrap().to_string(),
        name: Some(header_node.text().trim().to_string()),
        scope: None,
        r#type: None,
        valid_values: None,
        range: None,
    };

    let mut node_count = 30;
    let mut node_cur: Option<Node> = Some(header_node);

    loop {
        // Current node is None exit
        if node_cur.is_none() {
            break;
        }
        // Move cursor to previous and bump count
        node_cur = node_cur.unwrap().next();
        node_count = node_count - 1;
        // If still is None or count too low exit
        if node_cur.is_none() || node_count < 1 {
            break;
        }

        let n = node_cur.unwrap();

        // We hit the next header
        if n.is(Class("anchored_heading")) {
            break;
        }

        if n.is(Name("ul")) && n.find(Name("li")).next().is_some() {
            entry = process_ul(entry, n);
        }
    }

    /*
    const ulElementList = $(element).nextUntil('.anchored_heading');
    if (ulElementList.find('li > strong').length === 0) {
        return { id: null };
    }*/

    entry
}

pub fn extract_mariadb_from_text(qr: QueryResponse) -> Vec<KbParsedEntry> {
    let document = Document::from(qr.body.as_str());

    document
        .find(Class("anchored_heading"))
        .filter(|elem| elem.is(Name("h3")) || elem.is(Name("h4")))
        .filter(|elem| elem.attr("id").is_some())
        // Handle an edge case for https://mariadb.com/kb/en/temporal-data-tables/
        .filter(|elem| elem.text().trim() != "SELECT" && elem.attr("id").unwrap() != "select")
        .filter(|elem| {
            elem.text().trim() != "system-variables"
                && elem.attr("id").unwrap() != "system-variables"
        })
        .map(|header_node| process_block(header_node))
        .filter(|entry| {
            entry.r#type.is_some()
                || entry.default.is_some()
                || entry.dynamic.is_some()
                || entry.has_description
        })
        .collect()
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
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_1.html"),
            url: "https://example.com",
        });
        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: true,
                    cli: Some("--query-cache-size=#".to_string()),
                    default: Some("1M (>= MariaDB, 10.1.7), 0 (<= MariaDB 10.1.6), (although frequently given a default value in some setups)".to_string()),
                    dynamic: Some(true),
                    id: "query_cache_size".to_string(),
                    name: Some("query_cache_size".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: Some(vec!["0".to_string()]),
                    range: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_2() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_2.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("query-cache-strip-comments".to_string()),
                default: Some("OFF".to_string()),
                dynamic: Some(true),
                id: "query_cache_strip_comments".to_string(),
                name: Some("query_cache_strip_comments".to_string()),
                scope: Some(vec!["session".to_string(), "global".to_string()]),
                r#type: Some("boolean".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_3() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_3.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: None,
                default: None,
                dynamic: None,
                id: "ssl_accept_renegotiates".to_string(),
                name: Some("Ssl_accept_renegotiates".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_4() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_4.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: true,
                    cli: Some("--server-audit-events=value".to_string()),
                    default: Some("Empty string".to_string()),
                    dynamic: Some(true),
                    id: "server_audit_events".to_string(),
                    name: Some("server_audit_events".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: Some(vec![
                        "CONNECT".to_string(),
                        "QUERY".to_string(),
                        "TABLE".to_string(),
                        "CONNECT".to_string(),
                        "QUERY".to_string(),
                        "TABLE".to_string(),
                        "QUERY_DDL".to_string(),
                        "QUERY_DML".to_string(),
                        "CONNECT".to_string(),
                        "QUERY".to_string(),
                        "TABLE".to_string(),
                        "QUERY_DDL".to_string(),
                        "QUERY_DML".to_string(),
                        "QUERY_DCL".to_string(),
                        "CONNECT".to_string(),
                        "QUERY".to_string(),
                        "TABLE".to_string(),
                        "QUERY_DDL".to_string(),
                        "QUERY_DML".to_string(),
                        "QUERY_DCL".to_string(),
                        "QUERY_DML_NO_SELECT".to_string(),
                    ]),
                    range: None,
                },
                KbParsedEntry {
                    has_description: true,
                    cli: Some("--server-audit-excl-users=value".to_string()),
                    default: Some("Empty string".to_string()),
                    dynamic: Some(true),
                    id: "server_audit_excl_users".to_string(),
                    name: Some("server_audit_excl_users".to_string()),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("string".to_string()),
                    valid_values: None,
                    range: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_5() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_5.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: true,
                    dynamic: Some(false),
                    id: "tokudb_version".to_string(),
                    name: Some("tokudb_version".to_string()),
                    r#type: Some("string".to_string()),
                    cli: None,
                    default: None,
                    range: None,
                    scope: None,
                    valid_values: None,
                },
                KbParsedEntry {
                    has_description: true,
                    default: Some("1000".to_string()),
                    dynamic: Some(true),
                    id: "tokudb_write_status_frequency".to_string(),
                    name: Some("tokudb_write_status_frequency".to_string()),
                    range: Some(Range {
                        to_upwards: None,
                        from: Some(0),
                        to: Some(4294967295),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("integer".to_string()),
                    cli: None,
                    valid_values: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_6() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_6.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![
                KbParsedEntry {
                    has_description: true,
                    cli: Some("--rpl-semi-sync-slave-trace_level[=#]".to_string()),
                    default: Some("32".to_string()),
                    dynamic: Some(true),
                    id: "rpl_semi_sync_slave_trace_level".to_string(),
                    name: Some("rpl_semi_sync_slave_trace_level".to_string()),
                    range: Some(Range {
                        to_upwards: None,
                        from: Some(0),
                        to: Some(18446744073709551615),
                        from_f: None,
                        to_f: None,
                    }),
                    scope: Some(vec!["global".to_string()]),
                    r#type: Some("integer".to_string()),
                    valid_values: None,
                },
                KbParsedEntry {
                    has_description: true,
                    cli: Some("--rpl-semi-sync-master=value".to_string()),
                    default: Some("ON".to_string()),
                    id: "rpl_semi_sync_master".to_string(),
                    name: Some("rpl_semi_sync_master".to_string()),
                    r#type: Some("enumeration".to_string()),
                    valid_values: Some(vec![
                        "OFF".to_string(),
                        "ON".to_string(),
                        "FORCE".to_string(),
                        "FORCE_PLUS_PERMANENT".to_string()
                    ]),
                    range: None,
                    scope: None,
                    dynamic: None,
                },
            ],
            entries
        );
    }

    #[test]
    fn test_case_7() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_7.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                dynamic: None,
                cli: Some("--wsrep-provider=value".to_string()),
                default: Some("None".to_string()),
                id: "wsrep_provider".to_string(),
                name: Some("wsrep_provider".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("string".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_8() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_8.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--tls-version=value".to_string()),
                default: Some("TLSv1.1,TLSv1.2,TLSv1.3".to_string()),
                dynamic: Some(false),
                id: "tls_version".to_string(),
                name: Some("tls_version".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("enumeration".to_string()),
                valid_values: Some(vec![
                    "TLSv1.0".to_string(),
                    "TLSv1.1".to_string(),
                    "TLSv1.2".to_string(),
                    "TLSv1.3".to_string()
                ]),
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_9() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_9.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--connect-work-size=#".to_string()),
                default: Some("67108864".to_string()),
                dynamic: Some(true),
                id: "connect_work_size".to_string(),
                name: Some("connect_work_size".to_string()),
                scope: Some(vec!["global".to_string(), "session".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: Some("upwards".to_string()),
                    from: Some(4194304),
                    from_f: None,
                    to: None,
                    to_f: None,
                }),
            },],
            entries
        );
    }

    #[test]
    fn test_case_10() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_10.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--wsrep-sync-wait=#".to_string()),
                default: Some("0".to_string()),
                dynamic: Some(true),
                id: "wsrep_sync_wait".to_string(),
                name: Some("wsrep_sync_wait".to_string()),
                scope: Some(vec!["global".to_string(), "session".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: None,
                    from: Some(0),
                    from_f: None,
                    to: None,
                    to_f: None,
                }),
            },],
            entries
        );
    }

    #[test]
    fn test_case_11() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_11.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--lock-wait-timeout=#".to_string()),
                default: Some(
                    "86400 (1 day) >= MariaDB 10.2.4, , 31536000 (1 year) <= MariaDB 10.2.3"
                        .to_string()
                ),
                dynamic: Some(true),
                id: "lock_wait_timeout".to_string(),
                name: Some("lock_wait_timeout".to_string()),
                scope: Some(vec!["global".to_string(), "session".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: None,
                    from: None,
                    from_f: None,
                    to: Some(31536000),
                    to_f: None,
                }),
            },],
            entries
        );
    }

    #[test]
    fn test_case_12() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_12.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: None,
                default: None,
                dynamic: None,
                id: "wsrep_cert_index_size".to_string(),
                name: Some("wsrep_cert_index_size".to_string()),
                scope: None,
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_13() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_13.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--system-versioning-insert-history[={0|1}]".to_string()),
                default: Some("OFF".to_string()),
                dynamic: Some(true),
                id: "system_versioning_insert_history".to_string(),
                name: Some("system_versioning_insert_history".to_string()),
                scope: Some(vec!["global".to_string(), "session".to_string()]),
                r#type: Some("boolean".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_14() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_14.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: false,
                cli: Some("--gtid-pos-auto-engines=value".to_string()),
                default: Some("empty".to_string()),
                dynamic: Some(true),
                id: "gtid_pos_auto_engines".to_string(),
                name: Some("gtid_pos_auto_engines".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("string".to_string()),
                valid_values: None,
                range: None,
            },],
            entries
        );
    }

    #[test]
    fn test_case_15() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_15.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--handlersocket-wrlock-timeout=\"value\"".to_string()),
                default: None,
                dynamic: Some(false),
                id: "handlersocket_wrlock_timeout".to_string(),
                name: Some("handlersocket_wrlock_timeout".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: None,
                    from: Some(0),
                    to: Some(3600),
                    from_f: None,
                    to_f: None,
                }),
            },],
            entries
        );
    }

    #[test]
    fn test_case_16() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_16.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--innodb-fast-shutdown[=#]".to_string()),
                default: Some("1".to_string()),
                dynamic: Some(true),
                id: "innodb_fast_shutdown".to_string(),
                name: Some("innodb_fast_shutdown".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: None,
                    from: Some(0),
                    to: None,
                    from_f: None,
                    to_f: None,
                }),
            },],
            entries
        );
    }

    #[test]
    fn test_case_17() {
        let entries = extract_mariadb_from_text(QueryResponse {
            body: get_test_data("mariadb_test_case_17.html"),
            url: "https://example.com",
        });

        assert_eq!(
            vec![KbParsedEntry {
                has_description: true,
                cli: Some("--innodb-fill-factor=#".to_string()),
                default: Some("100".to_string()),
                dynamic: Some(true),
                id: "innodb_fill_factor".to_string(),
                name: Some("innodb_fill_factor".to_string()),
                scope: Some(vec!["global".to_string()]),
                r#type: Some("integer".to_string()),
                valid_values: None,
                range: Some(Range {
                    to_upwards: None,
                    from: Some(10),
                    to: Some(100),
                    from_f: None,
                    to_f: None,
                }),
            },],
            entries
        );
    }
}
