extern crate byteorder;
extern crate bytes;
extern crate int_hash;
extern crate core;

extern crate common;
extern crate server;


pub mod cache;
pub mod data;
pub mod logic;
pub mod app;

use server::cache::server::Server;

use app::App;

fn main() {
    println!("Hello, world!");

    let test: Vec<u32> = Vec::with_capacity(4);

    let app: App = App::new();

    let server: Server = Server::new();

    server::start_small_load_optimized(app, "0.0.0.0", 4321);
}
