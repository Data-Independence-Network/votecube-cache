extern crate byteorder;
extern crate bytes;
extern crate int_hash;
extern crate smallvec;
extern crate tokio;
extern crate tokio_codec;
extern crate tokio_core;
extern crate tokio_io;
extern crate core;


pub mod cache;
pub mod data;
pub mod logic;
pub mod server;

use server::app::App;

fn main() {
    println!("Hello, world!");

    let test: Vec<u32> = Vec::with_capacity(4);

    App::start_small_load_optimized(app, "0.0.0.0", 4321);
}
