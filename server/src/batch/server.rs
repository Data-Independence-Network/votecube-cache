use futures::Future;
use futures::future;
use futures::task;
use net2::TcpBuilder;
#[cfg(not(windows))]
use net2::unix::UnixTcpBuilderExt;
use num_cpus;
use std::sync::Arc;
use std::io;
use std::thread;
use std::time::{Duration, Instant};
use tokio;
use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
use tokio::timer::Interval;
use tokio_codec::Framed;

use super::super::http::Http;
use super::super::request::Request;
use super::super::codes;
use super::super::response::Response;
use super::batch::BatchedRequest;
use super::batch::BatchedRequestData;
use super::batch::INCOMING_BATCH;
use super::future::BatchedFuture;

pub trait App<I> {

    fn preprocess(&self, mut request: Request) -> BatchedRequestData<I>;

    fn process_batch(&mut self, mut batch_in_processing: Vec<BatchedRequest<I>>);

}

/**
 *
 *  Right now a single batch service is assumed to server only one type of request.
 *  This allows for the simplest possible solution, with one OS process per service.
 *  This also allows for very simple up-scaling.  Once a given request is known
 *  to be running too hot for too long, another service instance can simply be stood
 *  up (and added to the load balancer) for that type of request.
 *  This also makes upgrading a bit easier - if code is changed for one type of
 *  request then only that type of request is affected by the upgrade.  All the
 *  rest of the services can continue running without any interruption.
 *
 *  The downside is that this creates lots of processes to manage at the OS level.
 *  But, on the plus side, it is easy to see which request is eating up the CPU,
 *  using just the regular OS level tools like top.
 *
 *  For now, this is considered to be the preferred approach given that there
 *  aren't a lot of operations in VC:
 *
 *  Direction: Create + Read
 *  Dimension: Create + Read
 *  Poll:      Create + Read
 *  Vote:      Create
 *
 *
 *
 */
pub struct Server<I: 'static + Context + Send> {
    app: &'static App<I>,
}

impl<T: Context + Send> Server<I> {

    pub fn new(app: &App<I>) -> Server<I> {
        Server {
            app
        }
    }

    pub fn start_single_threaded(mut self, host: &str, port: u16) {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();

        let thread = thread::spawn(move || {
            let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();

            let server = future::lazy(move || {
                let listener = {
                    let builder = TcpBuilder::new_v4().unwrap();
                    #[cfg(not(windows))]
                        builder.reuse_address(true).unwrap();
                    #[cfg(not(windows))]
                        builder.reuse_port(true).unwrap();
                    builder.bind(addr).unwrap();
                    builder.listen(2048).unwrap()
                };
                let listener = TcpListener::from_std(listener, &tokio::reactor::Handle::current()).unwrap();

                listener.incoming().for_each(move |socket| {
                    let framed = Framed::new(socket, Http);
                    let (tx, rx) = framed.split();

                    let task = tx.send_all(rx.and_then(move |request: Request| {
                        self.resolve(request)
                    })).then(|_| future::ok(()));

                    // Spawn the task that handles the connection.
                    tokio::spawn(task);
                    Ok(())
                }).map_err(|err| eprintln!("accept error = {:?}", err))
            });

            runtime.spawn(server);
            runtime.run().unwrap();
        });

        println!("Server running on {}", addr);

        thread.join().unwrap();

        // Setup the batching process
        let task = Interval::new(Instant::now(), Duration::from_millis(2000))
            .for_each(|instant| {
                println!("fire; instant={:?}", instant);

                let batch_in_processing: Vec<BatchedRequest<I>> = INCOMING_BATCH;

                INCOMING_BATCH = Vec::with_capacity(2048);

                self.app.process_batch(batch_in_processing);

                for batchRequest in &batch_in_processing {
                    batchRequest.task.notify();
                }

                Ok(())
            })
            .map_err(|e| panic!("interval errored; err={:?}", e));

        // Start the batching process
        tokio::run(task);
    }

    /// Resolves a request, returning a future that is processable into a Response
    fn resolve(&self, mut request: Request) -> impl Future<Item=Response, Error=io::Error> + Send {
        let data = self.app.preprocess(request);

        if data.is_not_valid {
            return future::ok(data.output);
        }

        let request = BatchedRequest {
            input: data.input,
            // Park the request until the batch job timer wakes up
            // Should work as park and suspend the task
            task: task::current(),
            output: data.output,
        };

        INCOMING_BATCH.push(request);

        BatchedFuture::new(request)
    }
    
}
