use futures::task;
use futures::task::Task;

pub static mut PROCESSING_BATCH: Vec<BatchRequest> = Vec::new();
pub static mut INCOMING_BATCH: Vec<BatchRequest> = Vec::new();

pub struct BatchRequestData<T> {
    isValid: boolean,
    input: T
}

pub struct BatchedRequest<T> {
    input: T,
    task: Task,
}