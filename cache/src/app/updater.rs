use server::codes;
use server::cache::app::App;

use super::super::cache::cache::Cache;

pub struct CompleteCacheUpdater {

    pub cache: Box<Cache>,

}


impl CompleteCacheUpdater {

    pub fn new(
        cache: Box<Cache>,
    ) -> CompleteCacheUpdater {
        CompleteCacheUpdater {
            cache,
        }
    }

//    pub fn get_cache(
//        &self,
//    ) -> &'static Box<Cache> {
//
//        self.cache
//    }

}


impl App for CompleteCacheUpdater {
    fn get_response(
        &self,
        _path: &str,
        _request_body: &[u8],
    ) -> Vec<u8> {
        codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
    }
}