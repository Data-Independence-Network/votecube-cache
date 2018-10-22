use futures::future;
use futures::Future;
use net2::TcpBuilder;
#[cfg(not(windows))]
use net2::unix::UnixTcpBuilderExt;
use num_cpus;
// use std::sync::Arc;
use std::io;
use std::thread;
use tokio;
use tokio::net::{TcpListener};
// use tokio::net::{TcpStream, TcpListener};
use tokio::prelude::*;
use tokio_codec::Framed;


use super::super::context::Context;
use super::super::http::Http;
use super::super::request::Request;
use super::super::codes;
use super::super::response::Response;


pub trait App {

    fn get_response(
        &self,
        path: str,
        request_body: &[u8],
    ) -> Vec<u8>;

}

pub struct Server {

    app: &'static App,
}

impl Server {

    pub fn new (app: &'static App) -> Server {
        Server {
            app
        }
    }

    ///
    /// Starts the app with a thread pool optimized for small requests and quick timeouts. This
    /// is done internally by spawning a separate thread for each reactor core. This is valuable
    /// if all app endpoints are similar in their load, as work is divided evenly among threads.
    /// As seanmonstar points out though, this is a very specific use case and might not be useful
    /// for everyday work loads.alloc
    ///
    /// See the discussion here for more information:
    ///
    /// https://users.rust-lang.org/t/getting-tokio-to-match-actix-web-performance/18659/7
    ///
    pub fn start_small_load_optimized(mut self, host: &str, port: u16) {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
        let mut threads = Vec::new();
//        app._route_parser.optimize();
//        let arc_app = Arc::new(app);

        for _ in 0..num_cpus::get() {
//            let arc_app = arc_app.clone();
            threads.push(thread::spawn(move || {
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
//                        process(Arc::clone(&arc_app), socket);
//                        process(socket);
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
            }));
        }

        println!("Server running on {}", addr);

        for thread in threads {
            thread.join().unwrap();
        }

//        fn process<T: Context + Send>(app: Arc<Server<T>>, socket: TcpStream) {
//        fn process<T: Context + Send>(socket: TcpStream) {
//            let framed = Framed::new(socket, Http);
//            let (tx, rx) = framed.split();
//
//            let task = tx.send_all(rx.and_then(move |request: Request| {
//                self.resolve(request)
//            })).then(|_| future::ok(()));
//
//            // Spawn the task that handles the connection.
//            tokio::spawn(task);
//        }
    }

    #[inline]
    fn get_response(&self, request: &Request) -> Response {
        let mut response = Response::new();

        if request.method() != "PUT" {
            response.body_vec(codes::INVALID_DATA_FORMAT_RESPONSE);
            return response;
        }

        let path = request.path().as_ref();
        let request_body = request.raw_body();

        let data = self.app.get_response(*path, request_body);

        response.body_vec(data);

        response
    }

    /// Resolves a request, returning a future that is processable into a Response

    fn resolve(&self, mut request: Request) -> impl Future<Item=Response, Error=io::Error> + Send {
        let response = self.get_response(&request);
//        request.set_params(matched_route.params);

//        let context = (self.context_generator)(request);
//        let return_value = Box::new(future::ok(context));

//        return_value
//            .and_then(|context| {
//
        future::ok(response)
//            })
    }

}