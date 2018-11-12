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

//use std::sync::Arc;

//use server::cache::app::App;
use server::cache::server::Server;
use server::cache::updater::Updater;

use app::app::CompleteCacheApp;
use app::updater::CompleteCacheUpdater;
use cache::cache::Cache;


fn main() {
    println!("VC Complete (not-split up) Cache v0.1.0");

    let cache = Box::new(Cache::new());
    let cache_ref: &'static Box<Cache> = &cache;
//    {
//        let (cache_server, cache_updater) = {
//            (
//                Box::new(CompleteCacheApp::new(cache_ref)),
//                Box::new(CompleteCacheUpdater::new(cache))
//            )
////        (
////            cache_updater,
////        )
//        };
//
////    let cache_updater = Box::new(CompleteCacheUpdater::new(cache));
////    let cache_server = Box::new(CompleteCacheApp::new(&*cache_updater.cache));
//
//        let updater: Updater = Updater::new(cache_updater);
//        Updater::start_small_load_optimized(updater, "0.0.0.0", 5321);
//
//        let server: Server = Server::new(cache_server);
//        Server::start_small_load_optimized(server, "0.0.0.0", 4321);
//    }
}
