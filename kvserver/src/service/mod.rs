use std::sync::Arc;
use tracing::debug;

use crate::*;

mod command_service;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

pub fn dispatch(cmd: CommondRequest, store: &impl Storage) -> CommandResponse {
    match cmd.request_data {
        Some(commond_request::RequestData::Hget(param)) => param.execute(store),
        Some(commond_request::RequestData::Hgetall(param)) => param.execute(store),
        Some(commond_request::RequestData::Hset(param)) => param.execute(store),
        None => KvError::InvalidCommand("request has no data".into()).into(),
        _ => KvError::Internal("not implementned".into()).into(),
    }
}

pub struct Service {
    pub store: Arc<dyn Storage>,
}

impl Clone for Service {
    fn clone(&self) -> Self {
        Self { 
            store: Arc::clone(&self.store),
        }
    }
}

pub struct ServiceInner<Store> {
    store: Store,
}

impl Service {
    pub fn new<S: Storage>(store: S) -> Self {
        Self { 
            store: Arc::new(store),
         }
    }

    pub fn execute(&self, cmd: CommondRequest) -> CommandResponse {
        debug!("got request: {:?}", cmd);

        let res = dispatch(cmd, &self.store);
        debug!("response: {:?}", res);

        res
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;
    use crate::{MemTable, Value};

    #[test]
    fn service_should_work() {
        let service = Service::new(MemTable::default());
        let cloned = service.clone();
        let handle = thread::spawn(move || {
            let res = cloned.execute(CommondRequest::new_hset("t1", "k1", "v1".into()));
            assert_res_ok(res, &[Value::default()], &[]);
        });
        handle.join().unwrap();

        let res = service.execute(CommondRequest::new_hget("t1", "k1"));
        assert_res_ok(res, &["v1".into()], &[]);
    }
}

#[cfg(test)]
fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[KVpair]) {
    res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(res.status, 200);
    assert_eq!(res.message, "");
    assert_eq!(res.values, values);
    assert_eq!(res.pairs, pairs);
}

#[cfg(test)]
fn assert_res_error(res: CommandResponse, code: u32, msg: &str) {
    assert_eq!(res.status, code);
    assert!(res.message.contains(msg));
    assert_eq!(res.values, &[]);
    assert_eq!(res.pairs, &[]);
}