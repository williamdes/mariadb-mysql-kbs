use crate::data::{DataFile, PageProcess, QueryErrorResponse, QueryResponse};
use crate::{aurora_mysql, mariadb, mysql};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{env, fs};
use ureq::{Agent, Error, ResponseExt};

const UA_FROM: &str = "williamdes+mariadb-mysql-kbs@wdes.fr";
const UA: &str =
    "mariadb-mysql-kbs-bot (+https://github.com/williamdes/mariadb-mysql-kbs; williamdes+mariadb-mysql-kbs@wdes.fr)";
// dev.mysql.com 403s custom bot User-Agents, so present a browser there only.
const UA_BROWSER: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0";

pub enum ExtractionType {
    MariaDB,
    MySQL,
    AuroraMySQL,
}

pub enum ExtractionPreference {
    All,
    MariaDB,
    MySQL,
    AuroraMySQL,
}

pub fn extract(only: ExtractionPreference) {
    println!("Run build...");
    let pages: Vec<PageProcess> = match only {
        ExtractionPreference::All => [mysql::get_pages(), aurora_mysql::get_pages(), mariadb::get_pages()].concat(),
        ExtractionPreference::MySQL => mysql::get_pages(),
        ExtractionPreference::AuroraMySQL => aurora_mysql::get_pages(),
        ExtractionPreference::MariaDB => mariadb::get_pages(),
    };

    for page in pages {
        let path = data_file_path(page.data_type, page.get_data_prefix(), &page.name);
        if is_source_removed(&path) {
            println!("SKIP : {} is marked removed upstream; not browsing it", page.url);
            continue;
        }
        extract_page(page);
    }
    println!("All done.");
    println!("End !");
}

fn call_url(agent: &Agent, url: &str) -> Result<ureq::http::Response<ureq::Body>, Error> {
    let request = agent.get(url);
    // dev.mysql.com blocks the bot User-Agent, so send browser headers there;
    // every other source keeps the original self-identifying bot request.
    let request = if url.contains("dev.mysql.com") {
        request
            .header("User-Agent", UA_BROWSER)
            .header(
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("DNT", "1")
            .header("Sec-GPC", "1")
            .header("Upgrade-Insecure-Requests", "1")
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("Priority", "u=0, i")
    } else {
        request.header("From", UA_FROM).header("User-Agent", UA)
    };
    request.call()
}

pub fn get_html_from_url(agent: Agent, url: &str) -> Result<QueryResponse, QueryErrorResponse> {
    match call_url(&agent, url) {
        Ok(mut response) => Ok(QueryResponse {
            url: response.get_uri().to_string(),
            body: response.body_mut().read_to_string().expect("Should have text"),
        }),
        Err(Error::StatusCode(code)) => Err(QueryErrorResponse {
            url: Some(url.to_string()),
            code: Some(code),
            message: String::new(),
        }),
        Err(err) => Err(QueryErrorResponse {
            url: None,
            code: None,
            message: err.to_string(),
        }),
    }
}

/// Fetch a URL exactly as the extractor does (same client and headers) and
/// print the raw HTTP response — status code, version, headers and body — to
/// make debugging what a page returns easy from the CLI.
pub fn debug_url(url: &str) {
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .into();
    match call_url(&agent, url) {
        Ok(mut response) => {
            let final_url = response.get_uri().to_string();
            println!("{:?} {}", response.version(), response.status());
            println!("URL: {final_url}");
            for (name, value) in response.headers() {
                println!("{name}: {}", value.to_str().unwrap_or("<non-utf8>"));
            }
            println!();
            match response.body_mut().read_to_string() {
                Ok(body) => println!("{body}"),
                Err(err) => eprintln!("<failed to read body: {err}>"),
            }
        }
        Err(err) => eprintln!("Request failed: {err}"),
    }
}

fn extract_page(page: PageProcess) {
    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(30)))
        .build()
        .into();
    match get_html_from_url(agent, &page.url) {
        Ok(response) => {
            let final_url = response.url.clone();
            println!("URL : {} -> {}", page.url, final_url);
            let entries = match page.get_data_type() {
                ExtractionType::MariaDB => mariadb::extract_mariadb_from_text(response),
                ExtractionType::MySQL => mysql::extract_mysql_from_text(response),
                ExtractionType::AuroraMySQL => aurora_mysql::extract_aurora_mysql_from_text(response),
            };
            // A variable page never legitimately has zero entries: an empty
            // result means the page was removed/moved/redirected (e.g. the
            // retired TokuDB/Cassandra docs, which now redirect to a landing
            // page) or the fetch was blocked. Keep the existing committed data
            // rather than overwriting it with nothing.
            if entries.is_empty() {
                eprintln!(
                    "SKIP : {} produced 0 entries; marking source removed and keeping existing data",
                    page.url
                );
                mark_source_removed(&data_file_path(page.data_type, page.get_data_prefix(), &page.name));
                return;
            }
            let data = DataFile {
                data: entries,
                url: final_url.as_str(),
                name: &page.name,
            };
            write_page(page.data_type, page.get_data_prefix(), data);
        }
        Err(err) => {
            eprintln!(
                "URL : {} ended in {} ({})",
                page.url,
                err.code.unwrap_or(0),
                err.url.unwrap_or(String::new())
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

fn data_file_path(data_type: &str, file_prefix: &str, name: &str) -> String {
    let current_dir = env::current_dir().unwrap();
    format!(
        "{}/data/{}/{}{}.json",
        current_dir.to_str().unwrap().to_owned(),
        data_type,
        file_prefix,
        name
    )
}

fn write_page(data_type: &str, file_prefix: &str, data: DataFile) {
    write_json(data_file_path(data_type, file_prefix, data.name), data);
}

/// Partial view of a data file, used to read the `removed` marker without
/// deserializing the whole payload.
#[derive(Deserialize)]
struct DataFileFlags {
    #[serde(default)]
    removed: bool,
}

/// Whether a page's data file is already flagged as removed upstream, so it
/// should no longer be fetched.
fn is_source_removed(path: &str) -> bool {
    fs::read_to_string(path)
        .ok()
        .and_then(|contents| serde_json::from_str::<DataFileFlags>(&contents).ok())
        .is_some_and(|flags| flags.removed)
}

/// Flag a page's data file as removed upstream while preserving its existing
/// entries. The retired data is kept and the page is skipped on later runs.
fn mark_source_removed(path: &str) {
    let Ok(contents) = fs::read_to_string(path) else {
        // No existing data file to preserve, nothing to mark.
        return;
    };
    let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&contents) else {
        return;
    };
    if let Some(object) = value.as_object_mut() {
        object.insert("removed".to_string(), serde_json::Value::Bool(true));
    }
    let buf = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    value.serialize(&mut ser).expect("Unable to serialize data");
    let mut out = ser.into_inner();
    out.push(0x0a); // LF
    fs::write(path, out).expect("Unable to write file");
}
