use serde_aux::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ijk {
    pub group: String,
    pub options: Vec<Opt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opt {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub category: i32,
    pub name: String,
    pub value: String,
}
