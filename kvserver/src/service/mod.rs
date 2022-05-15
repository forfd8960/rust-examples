use crate::pb::{CommondRequest, CommandResponse};

mod command;

pub trait CommandService {
    fn execute(self, store: &impl Storage) -> CommandResponse;
}