pub mod abi;

use abi::{commond_request::RequestData, *};

impl CommondRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self{
            request_data: Some(RequestData::Hset(Hset{
                table: table.into(),
                pairs: Some(KVpair::new(key, value)),
            })),
        }
    }
}

impl KVpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}