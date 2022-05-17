use crate::*;
use http::StatusCode;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}


impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
       match self.pairs {
           Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
            Ok(Some(v)) => v.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
           },
           None => Value::default().into(),
       }

    }
}

impl From<Value> for CommandResponse {
    fn from(v: Value) -> Self {
        Self { status: StatusCode::OK.as_u16() as _,  values: vec![v], ..Default::default() }
    }
}

impl From<Vec<KVpair>> for CommandResponse {
    fn from(v: Vec<KVpair>) -> Self {
        Self { status: StatusCode::OK.as_u16() as _,  pairs: v, ..Default::default() }
    }
}

impl From<KvError> for CommandResponse {
    fn from(e: KvError) -> Self {
        let mut result = Self { 
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as _,
            message: e.to_string(),
            pairs: vec![],
            values: vec![],
        };

        match e {
            KvError::NotFound(_, _) => result.status = StatusCode::NOT_FOUND.as_u16() as _,
            KvError::InvalidCommand(_) => result.status = StatusCode::BAD_REQUEST.as_u16() as _,
            _ => {}
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commond_request::RequestData;

    #[test]
    fn hset_should_work() {
        let store = MemTable::new();
        let cmd = CommondRequest::new_hset("t1", "hello", "world".into());

        let res = dispatch(cmd.clone(), &store);
        assert_res_ok(res, &[Value::default()], &[]);
    }

    #[test]
    fn hget_should_work() {
        let store = MemTable::new();
        let cmd = CommondRequest::new_hset("score", "s1", 10.into());
        dispatch(cmd, &store);
        
        let get_cmd = CommondRequest::new_hget("score", "s1");
        let res = dispatch(get_cmd, &store);
        assert_res_ok(res, &[10.into()], &[]);
    }
    
    #[test]
    fn hgetall_should_work() {
        let store = MemTable::new();
        let cmds = vec![
            CommondRequest::new_hset("score", "s1", 8.into()),
            CommondRequest::new_hset("score", "s2", 6.into()),
            CommondRequest::new_hset("score", "s3", 9.into()),
            CommondRequest::new_hset("score", "s1", 10.into()),
        ];

        for cmd in cmds {
            dispatch(cmd, &store);
        }

        let cmd = CommondRequest::new_hgetall("score");
        let res = dispatch(cmd, &store);
        let pairs = &[
            KVpair::new("s1", 10.into()),
            KVpair::new("s2", 6.into()),
            KVpair::new("s3", 9.into()),
        ];
        assert_res_ok(res, &[], pairs);
    }

    fn assert_res_ok(mut res: CommandResponse, values: &[Value], pairs: &[KVpair]) {
        res.pairs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(res.status, 200);
        assert_eq!(res.message, "");
        assert_eq!(res.values, values);
        assert_eq!(res.pairs, pairs);
    }
}

