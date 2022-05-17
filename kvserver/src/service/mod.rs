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