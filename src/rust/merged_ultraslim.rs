use std::collections::HashMap;

pub struct Link {
    pub a: String,
    pub u: i8,
    pub t: i8,
}

pub struct Variable {
    pub n: String,
    pub t: i8,
    pub d: Option<bool>,
    pub a: Vec<Link>,
}

pub struct MergedUltraSlim {
    pub vars: Vec<Variable>,
    pub version: i8,
    pub urls: Vec<String>,
    pub types: HashMap<i8, String>,
    pub var_types: HashMap<i8, String>,
}
/*
impl MergedUltraSlim {
    pub fn new() -> MergedUltraSlim {
        MergedUltraSlim {
            vars: vec![],
            version: 1,
            urls: vec![],
            types: HashMap::new(),
            var_types: HashMap::new()
        }
    }
}*/
pub const data: MergedUltraSlim = MergedUltraSlim {
    version: 1,
    vars: [Variable; 0] = [],
    urls: [String; 0] = [],
    types: HashMap::new(),
    var_types: HashMap::new()
};
