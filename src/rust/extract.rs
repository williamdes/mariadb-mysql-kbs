use crate::data::{DataFile, PageProcess, QueryErrorResponse, QueryResponse};
use crate::{mariadb, mysql};
use serde::Serialize;
use std::time::Duration;
use std::{env, fs};
use ureq::{Agent, AgentBuilder, Error};

const UA_FROM: &str = "williamdes+mariadb-mysql-kbs@wdes.fr";
const UA: &str = "mariadb-mysql-kbs-bot (+https://github.com/williamdes/mariadb-mysql-kbs; williamdes+mariadb-mysql-kbs@wdes.fr)";

pub enum ExtractionPreference {
    All,
    MariaDB,
    MySQL,
}

pub fn extract(only: ExtractionPreference) {
    println!("Run build...");
    let pages: Vec<PageProcess> = match only {
        ExtractionPreference::All => [mysql::get_pages(), mariadb::get_pages()].concat(),
        ExtractionPreference::MySQL => mysql::get_pages(),
        ExtractionPreference::MariaDB => mariadb::get_pages(),
    };

    for page in pages {
        extract_page(page);
    }
    println!("All done.");
    println!("End !");
}

pub fn get_html_from_url(agent: Agent, url: &str) -> Result<QueryResponse, QueryErrorResponse> {
    match agent
        .get(url)
        .set("From", UA_FROM)
        .set("User-Agent", UA)
        .call()
    {
        Ok(response) => Ok(QueryResponse {
            url: response.get_url().to_owned(),
            body: response.into_string().expect("Should have text"),
        }),
        Err(Error::Status(code, response)) => Err(QueryErrorResponse {
            url: Some(response.get_url().to_owned()),
            code: Some(code),
            message: response.into_string().expect("Should have text"),
        }),
        Err(err) => Err(QueryErrorResponse {
            url: None,
            code: None,
            message: err.to_string(),
        }),
    }
}

fn extract_page(page: PageProcess) {
    let agent: Agent = AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    match get_html_from_url(agent, &page.url) {
        Ok(response) => {
            let final_url = response.url.clone();
            println!("URL : {} -> {}", &page.url, final_url);
            let data = DataFile {
                data: match page.is_mariadb_page() {
                    true => mariadb::extract_mariadb_from_text(response),
                    false => mysql::extract_mysql_from_text(response),
                },
                url: final_url.as_str(),
                name: &page.name,
            };
            write_page(page.data_type, page.get_data_prefix(), data);
        }
        Err(err) => {
            eprintln!(
                "URL : {} ended in {} ({})",
                &page.url,
                err.code.unwrap_or(0),
                err.url.unwrap_or("".to_string())
            );
        }
    }
}

fn write_json(filename: String, data: DataFile) {
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    data.serialize(&mut ser).expect("Unable to serialize data");
    let mut data = ser.into_inner();
    data.push(0x0a); // LF
    fs::write(filename, data).expect("Unable to write file");
}

fn write_page(data_type: &str, file_prefix: &str, data: DataFile) {
    let current_dir = env::current_dir().unwrap();
    write_json(
        format!(
            "{}/data/{}/{}{}.json",
            current_dir.to_str().unwrap().to_owned(),
            data_type,
            file_prefix,
            data.name
        ),
        data,
    );
}
