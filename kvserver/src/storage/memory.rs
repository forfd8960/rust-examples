use crate::{Storage};


#[derive(Clone, Debug, Default)]
pub struct MemTable {

}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn set(&self, table: &str, key: String, value: crate::Value) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, crate::KvError> {
        todo!()
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<crate::Value>, crate::KvError> {
        todo!()
    }

    fn get_all(&self, table: &str) -> Result<Vec<crate::KVpair>, crate::KvError> {
        todo!()
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = crate::KVpair>>, crate::KvError> {
        todo!()
    }
}