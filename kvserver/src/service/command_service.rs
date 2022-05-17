use crate::*;

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

