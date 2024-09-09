use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Result {
    code: i32,
    message: String,
    data: Option<Value>,
}

pub fn success() -> Result {
    Result {
        code: 200,
        message: "200 OK".to_string(),
        data: None,
    }
}

pub fn success_with_data(data: Value) -> Result {
    Result {
        code: 200,
        message: "200 OK".to_string(),
        data: Some(data),
    }
}

pub fn failure(code: i32, message: &str) -> Result {
    Result {
        code,
        message: message.to_string(),
        data: None,
    }
}