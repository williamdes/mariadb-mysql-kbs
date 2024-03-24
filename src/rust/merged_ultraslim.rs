use std::collections::HashMap;

use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum DataType {
    MySQL = 1,
    MariaDB = 2,
    AuroraMySQL = 3,
}

#[derive(Deserialize)]
pub struct Link {
    pub a: Option<String>,
    pub u: usize,
    pub t: DataType,
}

#[derive(Deserialize)]
pub struct Variable {
    pub n: Option<String>,
    pub t: Option<usize>,
    pub d: Option<bool>,
    pub a: Vec<Link>,
}

#[derive(Deserialize)]
pub struct MergedUltraSlim {
    pub vars: HashMap<String, Variable>,
    pub version: usize,
    pub urls: Vec<String>,
    pub types: HashMap<usize, String>,
    #[serde(rename = "varTypes")]
    pub var_types: HashMap<usize, String>,
}

#[derive(PartialEq)]
pub enum SearchType {
    Any = -1,
    MySQL = 1,
    MariaDB = 2,
    AuroraMySQL = 3,
}

#[derive(Debug)]
pub struct SearchError {}

impl MergedUltraSlim {
    pub fn get_by_name(
        &self,
        variable_name: &str,
        search_type: SearchType,
    ) -> Result<String, SearchError> {
        let kb_entry = self.get_variable(variable_name);
        match kb_entry {
            Ok(entry) => {
                let found = entry.a.iter().find(|link| {
                    (search_type == SearchType::Any)
                        | (search_type == SearchType::MySQL && link.t == DataType::MySQL)
                        | (search_type == SearchType::MariaDB && link.t == DataType::MariaDB)
                });
                match found {
                    Some(link) => Ok(match &link.a {
                        Some(anchor) => format!(
                            "{}#{}",
                            self.urls.get(link.u).expect("Url to exist in table"),
                            anchor,
                        ),
                        None => {
                            format!("{}", self.urls.get(link.u).expect("Url to exist in table"),)
                        }
                    }),
                    None => Err(SearchError {}),
                }
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_variable(&self, variable_name: &str) -> Result<&Variable, SearchError> {
        match self.vars.get(variable_name) {
            Some(entry) => Ok(entry),
            None => Err(SearchError {}),
        }
    }
}
