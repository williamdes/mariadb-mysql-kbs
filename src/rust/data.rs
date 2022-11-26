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

#[derive(Debug, PartialEq)]
pub struct KbParsedEntry {
    pub cli: Option<String>,
    pub default: Option<String>,
    pub r#type: Option<String>,
    pub dynamic: Option<bool>,
    pub id: String,
    pub name: Option<String>,
    pub scope: Option<Vec<String>>,
}
