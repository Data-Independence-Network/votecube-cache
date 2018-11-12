extern crate byteorder;
extern crate bytes;
extern crate core;
extern crate evmap;
extern crate int_hash;
extern crate lazy_static;

extern crate common;
extern crate server;


pub mod cache;
pub mod data;
pub mod logic;
pub mod app;

use server::cache::app::App;
use server::cache::server::Server;

use app::app::CompleteCacheApp;
use cache::cache::Cache;
use cache::cache_reader::CacheReader;

fn main() {
    println!("VC Complete (not-split up) Cache v0.1.0");

    let cache: Box<CacheReader + Send + Sync> = Box::new(Cache::new());
    let cache_server: Box<App + Sync + Send> = Box::new(CompleteCacheApp::new(cache));
    let server: Server = Server::new(cache_server);

    /**
    Cache moving logic

    In MVP version, cache does not do any database lookups.  Whatever poll ids have been accumulated
    during a particular time period are then used to initialize vote count caches for current & past
    time periods.

    The logic to initialize vote counts needs to run:
        After the addition of poll ids for future periods have finished
        Probably periodically throughout the 24 hour period, as the counts are locked down for
            each timezone.

    */

    Server::start_small_load_optimized(server, "0.0.0.0", 4321, 5432);
}
