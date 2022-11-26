pub struct Page<'a> {
    pub url: &'a str,
    pub name: &'a str,
}

pub struct PageProcess<'a> {
    pub url: String,
    pub name: String,
    pub data_type: &'a str,
}
