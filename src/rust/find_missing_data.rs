use crate::{
    extract::{get_html_from_url, ExtractionPreference},
    mariadb,
};
use mariadb_mysql_kbs::{merged_ultraslim::SearchType, search};
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

pub fn run(only: ExtractionPreference) {
    println!("Run checking...");
    match only {
        ExtractionPreference::All => {
            check_mariadb();
        }
        ExtractionPreference::MySQL => {}
        ExtractionPreference::MariaDB => {
            check_mariadb();
        }
    };

    println!("All done.");
    println!("End !");
}

fn check_mariadb() {
    check_mariadb_url("https://mariadb.com/kb/en/mysqld-options/");
    check_mariadb_url("https://mariadb.com/kb/en/server-system-variables/");
    check_mariadb_url("https://mariadb.com/kb/en/server-status-variables/");
}

fn check_mariadb_url(url: &str) {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();

    match get_html_from_url(agent, url) {
        Ok(response) => {
            println!("Checking URL: {}", url);
            let data = mariadb::extract_mariadb_from_text(response);
            let loaded_data = search::load_data();
            for entry in data {
                if entry.name.is_some() {
                    let entry_name = entry.name.unwrap();
                    match loaded_data.get_by_name(&entry_name, SearchType::MariaDB) {
                        Ok(_) => {}
                        Err(_) => {
                            if !entry_name.starts_with("--") {
                                println!("Missing: {:?}", entry_name);
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            eprintln!(
                "URL : {} ended in {} ({})",
                url,
                err.code.unwrap_or(0),
                err.url.unwrap_or("".to_string())
            );
        }
    }
}
