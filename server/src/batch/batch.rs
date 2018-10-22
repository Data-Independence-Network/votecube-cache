// use futures::task;
use futures::task::Task;

use super::super::response::Response;

// pub static mut INCOMING_BATCH: Vec<BatchedRequest<I>> = Vec::with_capacity(2048);

pub struct BatchedRequestData<'a, T: 'a> {
    pub input: &'a T,
    pub is_not_valid: bool,
    pub output: &'a Response,
}

pub struct BatchedRequest<'a, T: 'a> {
    pub input: &'a T,
    pub output: &'a Response,
    pub task: &'a Task,
}
