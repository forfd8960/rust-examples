#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommondRequest {
    #[prost(oneof="commond_request::RequestData", tags="1, 2")]
    pub request_data: ::core::option::Option<commond_request::RequestData>,
}
/// Nested message and enum types in `CommondRequest`.
pub mod commond_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestData {
        #[prost(message, tag="1")]
        Hget(super::Hget),
        #[prost(message, tag="2")]
        Hgetall(super::Hgetall),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResponse {
    #[prost(uint32, tag="1")]
    pub status: u32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    #[prost(message, repeated, tag="4")]
    pub pairs: ::prost::alloc::vec::Vec<KVpair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hget {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hgetall {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmget {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag="1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag="2")]
        Binary(::prost::alloc::vec::Vec<u8>),
        #[prost(int64, tag="3")]
        Integer(i64),
        #[prost(double, tag="4")]
        Float(f64),
        #[prost(bool, tag="5")]
        Bool(bool),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KVpair {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hset {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pairs: ::core::option::Option<KVpair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmset {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub pairs: ::prost::alloc::vec::Vec<KVpair>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hdel {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmdel {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hexists {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmexist {
    #[prost(string, tag="1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}