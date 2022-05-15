use crate::{KvError, Value, KVpair};

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<KVpair>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KVpair>>, KvError>;
}