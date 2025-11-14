use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRecord {
    pub id: usize,
    pub level: LogLevel,
    pub msg: String,
    pub ts: usize,
}

