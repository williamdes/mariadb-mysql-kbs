use serde::{Deserialize, Serialize};

pub struct Page<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

#[derive(Clone)]
pub struct PageProcess<'a> {
    pub url: String,
    pub name: String,
    pub data_type: &'a str,
}

pub struct QueryResponse<'a> {
    pub url: &'a str,
    pub body: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub from: Option<i128>,
    pub to: Option<i128>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KbParsedEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cli: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<bool>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<Range>,
}
