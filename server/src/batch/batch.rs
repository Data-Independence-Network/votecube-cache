// use futures::task;
use futures::task::Task;

use super::super::request::Request;
use super::super::response::Response;

// pub static mut INCOMING_BATCH: Vec<BatchedRequest<I>> = Vec::with_capacity(2048);

pub struct BatchedRequestData<'a> {
    pub input: &'a Request,
    pub is_not_valid: bool,
    pub output: &'a Response,
}

pub struct BatchedRequest<'a> {
    pub input: &'a Request,
    pub output: &'a Response,
    pub task: &'a Task,
}
