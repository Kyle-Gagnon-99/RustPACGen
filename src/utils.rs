use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrNumber {
    String(String),
    Number(usize),
}

impl Into<String> for StringOrNumber {
    fn into(self) -> String {
        match self {
            StringOrNumber::String(s) => s,
            StringOrNumber::Number(n) => n.to_string(),
        }
    }
}

impl Into<usize> for StringOrNumber {
    fn into(self) -> usize {
        match self {
            StringOrNumber::String(s) => s.parse().unwrap(),
            StringOrNumber::Number(n) => n,
        }
    }
}
