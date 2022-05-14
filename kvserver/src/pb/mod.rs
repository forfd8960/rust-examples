pub mod abi;

use abi::{commond_request::RequestData, *};

impl CommondRequest {
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self{
            request_data: Some(RequestData::Hset(Hset{
                table: table.into(),
                pairs: Some(KVpair::new(key, value)),
            }))
        }
    }
}

Impl KVpair {
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}