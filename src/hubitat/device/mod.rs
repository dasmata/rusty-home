use crate::HClient;
use serde::{Deserialize, Serialize};

pub mod switch;

#[derive(Serialize, Deserialize)]
pub struct Device {
    pub id: String,
    pub name: Option<String>,
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub device_type: Option<String>,
    pub room: Option<String>,
    pub capabilities: Option<serde_json::Value>,
    pub commands: Option<Vec<String>>,
    pub attributes: Option<Vec<Attribute>>,
    #[serde(skip)]
    pub client: HClient,
    pub state: Option<String>
}

pub enum Attributes {
    Switch
}

impl Attributes {
    pub fn as_str(&self) -> &str {
        match self {
            Attributes::Switch => "switch" ,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    #[serde(rename = "dataType")]
    pub data_type: String,
    #[serde(rename = "currentValue")]
    pub current_value: serde_json::Value,
    pub values: Option<Vec<String>>
}


#[derive(Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "command")]
    pub name: String,
    #[serde(rename = "type")]
    pub value_type: Vec<String>
}


