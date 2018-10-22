use futures::Async;
use futures::Future;
use std::io;

// use super::server::App;
use super::batch::BatchedRequest;
use super::super::response::Response;

pub struct BatchedFuture<'a, 'b: 'a, I: 'b> {
    request: &'a mut BatchedRequest<&'b I>,
}

impl<'a, 'b, I> BatchedFuture<'a, 'b, I> {

    pub fn new(
        request: &'a mut BatchedRequest<&'b I>
    ) -> BatchedFuture<'a, 'b, I> {
        BatchedFuture {
            request
        }
    }

}

impl<'a, 'b, I> Future for BatchedFuture<'a, 'b, I> {
    type Item = Response;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        Ok(Async::Ready(self.request.output))
    }
}
