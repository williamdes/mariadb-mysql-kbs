use crate::cleaner;
use crate::data::KbParsedEntry;
use crate::data::PageProcess;
use crate::data::QueryResponse;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};

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
    let anchor_name_node = node
        .parent()
        .expect("Has a parent")
        .find(Name("a"))
        .filter(|el| el.attr("name").is_some() && el.attr("class").is_none())
        .next();
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
            .next()
            .expect("Anchor to have #")
            .to_string(),
    }
}

fn process_row_to_entry(
    row_name: String,
    row_value: String,
    mut entry: KbParsedEntry,
    table_node: Node,
) -> KbParsedEntry {
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
        ("default value" | "default, range") => {
            entry.default = Some(cleaner::clean_default(row_value.trim().to_string()));
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
        key => {
            println!("tr: {} -> {}", row_name, row_value);
            println!("missing: {}", key);
        }
    }

    entry
    /*
        case "type: default, range":
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
            }
            break;
        case "valid values":
            doc.validValues = $(value)
                .find("code")
                .get()
                .map((el) => $(el).text());
            break;
        case "minimum value":
            if (doc.range == undefined) {
                doc.range = {};
            }
            doc.range.from = parseFloat(value.text().trim());
            break;
        case "maximum value":
            if (doc.range == undefined) {
                doc.range = {};
            }
            doc.range.to = parseFloat(value.text().trim());
            break;
    }*/
}

fn extract_mysql_from_text(qr: QueryResponse) -> Vec<KbParsedEntry> {
    let document = Document::from(qr.body.as_str());

    // TODO: .informaltable
    document
        .find(Class("table"))
        .filter(|elem| {
            let summary_attr = elem.attr("summary");
            match summary_attr {
                Some(attr) => attr.contains("Properties for"),
                None => match elem.find(Name("th")).next() {
                    Some(e) => e.text() == "Property",
                    None => false,
                },
            }
        })
        .map(|table_node| {
            let mut entry = KbParsedEntry {
                cli: None,
                default: None,
                dynamic: None,
                id: find_table_archor(table_node),
                name: None,
                scope: None,
                r#type: None,
            };

            for tbody in table_node.find(Name("tbody")) {
                for tr in tbody.find(Name("tr")) {
                    let mut tds = tr.find(Name("td"));
                    let row_name: String = tds
                        .next()
                        .expect("Node to exist")
                        .text()
                        .to_lowercase()
                        .trim()
                        .to_owned();
                    let row_value = tds.next().expect("Node to exist").text();
                    entry = process_row_to_entry(row_name, row_value, entry, table_node);
                }
            }

            /*
            var name = tds.first().text().toLowerCase().trim();
            var value = tds.last();
            let ths = $(elem).find("th"); // Fallback if the key is in a th
            if (ths.length > 0) {
                name = ths.first().text().toLowerCase().trim();
            }*/
            entry
        })
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
            if (!doc.name && doc.cli) {
                var matches = doc.cli.match(cleaner.regexCli);
                doc.name = matches[2].replace(/-/g, "_");
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
    use pretty_assertions::{assert_eq, assert_ne};
    use std::env;
    use std::fs;

    fn get_test_data(file_name: &str) -> String {
        let test_dir = env::current_dir().unwrap();
        fs::read_to_string(test_dir.to_str().unwrap().to_owned() + "/src/rust/data/" + file_name)
            .expect("Should have been able to read the test data file")
    }

    #[test]
    fn test_case_1() {
        //let cli = ;
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
                },
                KbParsedEntry {
                    cli: Some("--ndb-allow-copying-alter-table=[ON|OFF]".to_string()),
                    default: Some("ON (Version: NDB 7.5-7.6)".to_string()),
                    dynamic: Some(true),
                    id: "option_mysqld_ndb-allow-copying-alter-table".to_string(),
                    name: Some("ndb-allow-copying-alter-table".to_string()),
                    scope: Some(vec!["global".to_string(), "session".to_string()]),
                    r#type: None,
                },
            ],
            entries
        );
    }
}
