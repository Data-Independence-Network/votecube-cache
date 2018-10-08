use futures::Async;
use futures::Future;

pub struct BatchedFuture {
    index: usize
}

impl BatchedFuture {

    pub fn new(
        index: usize
    ) -> BatchedFuture {
        BatchedFuture {
        }
    }

}

impl Future for BatchedFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<<Self as Future>::Item>, <Self as Future>::Error> {
        unimplemented!()
    }
}