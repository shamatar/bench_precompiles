use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct GethJsonFormat {
    #[serde(rename = "Input")]
    input: String,
    #[serde(rename = "Expected")]
    output: String,
    #[serde(rename = "Name")]
    name: String
}

impl GethJsonFormat {
    pub fn new_from_data_and_name(input: &[u8], output: &[u8], name: String) -> Self {
        Self {
            input: hex::encode(input),
            output: hex::encode(output),
            name
        }
    }
}