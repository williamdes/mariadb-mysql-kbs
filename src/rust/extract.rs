use crate::data::{DataFile, PageProcess, QueryResponse};
use crate::{mariadb, mysql};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::{FROM, USER_AGENT};
use serde::Serialize;
use std::{env, fs};

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

fn add_headers(rb: RequestBuilder) -> RequestBuilder {
    rb.header(FROM, UA_FROM).header(USER_AGENT, UA)
}

pub fn get_html_from_url(client: Client, url: &str) -> QueryResponse {
    let mut request = client.get(url);
    request = add_headers(request);

    let response = request
        .send()
        .expect("Url should be fetched")
        .text()
        .unwrap();

    QueryResponse {
        body: response,
        url: url,
    }
}

fn extract_page(page: PageProcess) {
    println!("URL : {}", &page.url);
    let client = Client::new();
    let response = get_html_from_url(client, &page.url);
    let data = DataFile {
        data: match page.is_mariadb_page() {
            true => mariadb::extract_mariadb_from_text(response),
            false => mysql::extract_mysql_from_text(response),
        },
        url: &page.url,
        name: &page.name,
    };
    write_page(page.data_type, page.get_data_prefix(), data);
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
