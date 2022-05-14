use crate::pb::{CommondRequest, CommandResponse};

mod command;
mod storage;

pub trait Storage {

}

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}