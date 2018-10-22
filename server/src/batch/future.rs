use futures::Async;
use futures::Future;
use futures::Poll;
use std::io;

// use super::server::App;
use super::batch::BatchedRequest;
use super::super::response::Response;


pub type MiddlewareReturnValue<T> = Box<Future<Item=T, Error=io::Error> + Send>;

pub struct BatchedFuture<'a, 'b: 'a, T: 'b + Sync> {
    request: &'a mut BatchedRequest<'b, T>,
}

impl<'a, 'b, T: 'b + Sync> Future for BatchedFuture<'a, 'b, T> {
    type Item = Response;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready(*self.request.output))
    }
}
