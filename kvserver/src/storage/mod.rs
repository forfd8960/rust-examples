use crate::{Value, KvError, KVpair};

mod memory;
pub use memory::MemTable;


pub trait Storage: Send + Sync + 'static {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    fn get_all(&self, table: &str) -> Result<Vec<KVpair>, KvError>;
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KVpair>>, KvError>;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memtable_basic_interface_should_work() {
        let store = MemTable::new();
        test_basi_interface(store);
    }

    #[test]
    fn memtable_get_all_should_work() {
        let store = MemTable::new();
        test_get_all(store);
    }

    #[test]
    fn memtable_get_iter_should_work() {
        let store = MemTable::new();
        test_get_iter(store);
    }

    fn test_basi_interface(store: impl Storage) {
        let v = store.set("t1", "hello".into(), "kv".into());
        assert!(v.unwrap().is_none());

        let v1 = store.set("t1", "hello".into(), "kv1".into());
        assert_eq!(v1, Ok(Some("kv".into())));

        let v = store.get("t1", "hello");
        assert_eq!(v, Ok(Some("kv1".into())));
        assert_eq!(Ok(None), store.get("t1", "hello1"));
        assert!(store.get("t2", "hello1").unwrap().is_none());

        assert_eq!(store.contains("t1", "hello"), Ok(true));
        assert_eq!(store.contains("t1", "hello1"), Ok(false));
        assert_eq!(store.contains("t2", "hello"), Ok(false));

        let v = store.del("t1", "hello");
        assert_eq!(v, Ok(Some("kv1".into())));

        assert_eq!(Ok(None), store.del("t1", "hello1"));
        assert_eq!(Ok(None), store.del("t2", "hello"));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data = store.get_all("t2").unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(
            data,
            vec![
                KVpair::new("k1", "v1".into()),
                KVpair::new("k2", "v2".into())
            ]
        )
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1".into(), "v1".into()).unwrap();
        store.set("t2", "k2".into(), "v2".into()).unwrap();

        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());

        assert_eq!(
            data,
            vec![
                KVpair::new("k1", "v1".into()),
                KVpair::new("k2", "v2".into())
            ]
        )
    }
}
