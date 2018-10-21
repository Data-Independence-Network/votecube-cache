use futures::Async;
use futures::Future;
use std::io;

use super::server::App;
use super::batch::BatchedRequest;
use super::super::response::Response;

pub struct BatchedFuture<'a, I> {
    request: &'a BatchedRequest<I>,
}

impl<I, O> BatchedFuture<I> {

    pub fn new(
        request: &BatchedRequest<I>
    ) -> BatchedFuture<I> {
        BatchedFuture {
            request
        }
    }

}

impl<I, O> Future for BatchedFuture<I> {
    type Item = Response;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Async::Ready(self.request.output)
    }
}