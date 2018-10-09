use futures::task;
use futures::task::Task;

use super::super::response::Response;

pub static mut INCOMING_BATCH: Vec<BatchRequest> = Vec::with_capacity(2048);

pub struct BatchedRequestData<I> {
    pub input: I,
    pub is_not_valid: boolean,
    pub output: Response,
}

pub struct BatchedRequest<I> {
    pub input: I,
    pub output: Response,
    pub task: Task,
}