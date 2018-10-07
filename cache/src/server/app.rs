use std::io;
use std::io::Cursor;
use std::thread;
use tokio::net::{TcpStream, TcpListener};
use std::sync::Arc;
use tokio_codec::Framed;
use tokio::prelude::*;
use byteorder::{BigEndian, ReadBytesExt};

use super::response::Response;
use super::request::Request;
use super::http::Http;
use super::codes;

use super::super::logic::serve::rankings::category;
use super::super::logic::serve::rankings::location;
use super::super::logic::serve::rankings::location_category;

use super::super::logic::serve::recent::category::get_day_after_tomorrows_category_polls;
use super::super::logic::serve::recent::category::get_next_months_category_polls;
use super::super::logic::serve::recent::category::get_next_weeks_category_polls;
use super::super::logic::serve::recent::category::get_tomorrows_category_polls;

use super::super::logic::serve::recent::category::get_day_after_tomorrows_category_polls;
use super::super::logic::serve::recent::category::get_next_months_category_polls;
use super::super::logic::serve::recent::category::get_next_weeks_category_polls;
use super::super::logic::serve::recent::category::get_tomorrows_category_polls;

use super::super::logic::serve::recent::location::get_day_after_tomorrows_location_polls;
use super::super::logic::serve::recent::location::get_next_months_location_polls;
use super::super::logic::serve::recent::location::get_next_weeks_location_polls;
use super::super::logic::serve::recent::location::get_tomorrows_location_polls;

use super::super::logic::serve::recent::location_category::get_day_after_tomorrows_category_location_polls;
use super::super::logic::serve::recent::location_category::get_next_months_category_location_polls;
use super::super::logic::serve::recent::location_category::get_next_weeks_category_location_polls;
use super::super::logic::serve::recent::location_category::get_tomorrows_location_category_polls;


pub struct App<T: 'static + Context + Send> {
    //    pub _route_parser: RouteParser<T>,
    ///
    /// Generate context is common to all `App`s. It's the function that's called upon receiving a request
    /// that translates an acutal `Request` struct to your custom Context type. It should be noted that
    /// the context_generator should be as fast as possible as this is called with every request, including
    /// 404s.
    pub context_generator: fn(Request) -> T
}

impl<T: Context + Send> App<T> {
    ///
    /// Starts the app with a thread pool optimized for small requests and quick timeouts. This
    /// is done internally by spawning a separate thread for each reactor core. This is valuable
    /// if all server endpoints are similar in their load, as work is divided evenly among threads.
    /// As seanmonstar points out though, this is a very specific use case and might not be useful
    /// for everyday work loads.alloc
    ///
    /// See the discussion here for more information:
    ///
    /// https://users.rust-lang.org/t/getting-tokio-to-match-actix-web-performance/18659/7
    ///
    pub fn start_small_load_optimized(mut app: App<T>, host: &str, port: u16) {
        let addr = (host, port).to_socket_addrs().unwrap().next().unwrap();
        let mut threads = Vec::new();
        app._route_parser.optimize();
        let arc_app = Arc::new(app);

        for _ in 0..num_cpus::get() {
            let arc_app = arc_app.clone();
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
                        process(Arc::clone(&arc_app), socket);
                        Ok(())
                    })
                        .map_err(|err| eprintln!("accept error = {:?}", err))
                });

                runtime.spawn(server);
                runtime.run().unwrap();
            }));
        }

        println!("Server running on {}", addr);

        for thread in threads {
            thread.join().unwrap();
        }

        fn process<T: Context + Send>(app: Arc<App<T>>, socket: TcpStream) {
            let framed = Framed::new(socket, Http);
            let (tx, rx) = framed.split();

            let task = tx.send_all(rx.and_then(move |request: Request| {
                app.resolve(request)
            }))
                .then(|_| future::ok(()));

            // Spawn the task that handles the connection.
            tokio::spawn(task);
        }
    }

    #[inline]
    fn get_response(&self, request: &Request) -> Response {
        let mut response = Response::new();

        if request.method() != "PUT" {
            response.body_vec(&[codes::RESPONSE_INVALID_REQUEST]);
            return response;
        }

        let path = request.path().as_ref();
        let request_body = request.raw_body();

        let data = self.getResponse(request_body);

        response.body_vec(data);

        response
    }

    #[inline]
    fn getResponse(
        &self,
        request_body: &[u8],
    ) -> Vec<u8> {
        match path {
            /**
             *
             *  POLL RANKINGS
             *
             */

            // Category Poll Rankings

            codes::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_this_months_category_rankings_by_global_id(
                        vcMonthId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_this_months_category_rankings_by_cache_index(
                        vcMonthId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_this_weeks_category_rankings_by_global_id(
                        vcWeekId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_this_weeks_category_rankings_by_cache_index(
                        vcWeekId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_todays_category_rankings_by_global_id(
                        vcDayId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_todays_category_rankings_by_cache_index(
                        vcDayId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_last_months_category_rankings_by_global_id(
                        vcMonthId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_last_months_category_rankings_by_cache_index(
                        vcMonthId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_last_weeks_category_rankings_by_global_id(
                        vcWeekId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_last_weeks_category_rankings_by_cache_index(
                        vcWeekId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_yesterdays_category_rankings_by_global_id(
                        vcDayId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_yesterdays_category_rankings_by_cache_index(
                        vcDayId, blockIndex, categoryCacheIndex)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, globalCategoryId)
                    = readTwoIntsAndLong(request_body);
                    category::get_day_b4_yesterdays_category_rankings_by_global_id(
                        vcDayId, blockIndex, globalCategoryId)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, blockIndex, categoryCacheIndex)
                    = readThreeInts(request_body);
                    category::get_day_b4_yesterdays_category_rankings_by_cache_index(
                        vcDayId, blockIndex, categoryCacheIndex)
                }
            }

            // Location Poll Rankings

            codes::URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_this_months_location_rankings_by_global_id(
                        vcMonthId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_this_months_location_rankings_by_cache_index(
                        vcMonthId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_this_weeks_location_rankings_by_global_id(
                        vcWeekId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_this_weeks_location_rankings_by_cache_index(
                        vcWeekId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_todays_location_rankings_by_global_id(
                        vcDayId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_todays_location_rankings_by_cache_index(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_last_months_location_rankings_by_global_id(
                        vcMonthId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcMonthId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_last_months_location_rankings_by_cache_index(
                        vcMonthId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_last_weeks_location_rankings_by_global_id(
                        vcWeekId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcWeekId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_last_weeks_location_rankings_by_cache_index(
                        vcWeekId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_yesterdays_location_rankings_by_global_id(
                        vcDayId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_yesterdays_location_rankings_by_cache_index(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId)
                    = readThreeIntsAndLong(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_global_id(
                        vcDayId, timezoneId, blockIndex, globalLocationId)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex)
                    = readFourInts(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_cache_index(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex)
                }
            }

            // Location Category Poll Rankings

            codes::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_this_months_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_this_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_this_months_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_todays_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_todays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_todays_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_last_months_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_last_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_last_months_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                    = readThreeIntsAndTwoLongs(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_global_ids(
                        vcDayId, timezoneId, blockIndex, globalLocationId, globalCategoryId)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrongRequestLength24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                    = readFourIntsAndLong(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, globalCategoryId)
                }
            }
            codes::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                    = readFiveInts(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_cache_indexes(
                        vcDayId, timezoneId, blockIndex, locationCacheIndex, locationCategoryCacheIndex)
                }
            }

            /**
             *
             *  RECENT POLLS
             *
             */

            // Recent Polls by Location

            codes::URL_NEXT_MONTHS_LOCATION_POLLS => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (monthId, timezoneId, blockNumber, globalLocationId) = readThreeIntsAndLong(request_body);
                    get_next_months_location_polls(monthId, timezoneId, blockNumber, globalLocationId);
                }
            }
            codes::URL_NEXT_WEEKS_LOCATION_POLLS => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (weekId, timezoneId, blockNumber, globalLocationId) = readThreeIntsAndLong(request_body);
                    get_next_weeks_location_polls(weekId, timezoneId, blockNumber, globalLocationId);
                }
            }
            codes::URL_TOMORROWS_LOCATION_POLLS => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId, timezoneId, blockNumber, globalLocationId) = readThreeIntsAndLong(request_body);
                    get_tomorrows_location_polls(dayId, timezoneId, blockNumber, globalLocationId);
                }
            }
            codes::URL_DAY_AFTER_TOMORROWS_LOCATION_POLLS => {
                if wrongRequestLength20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId, timezoneId, blockNumber, globalLocationId) = readThreeIntsAndLong(request_body);
                    get_day_after_tomorrows_location_polls(dayId, timezoneId, blockNumber, globalLocationId);
                }
            }

            // Recent Polls by Category

            codes::URL_NEXT_MONTHS_CATEGORY_POLLS => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (monthId,blockNumber, globalCategoryId) = readTwoIntsAndLong(request_body);
                    get_next_months_category_polls(monthId, blockNumber, globalCategoryId);
                }
            }
            codes::URL_NEXT_WEEKS_CATEGORY_POLLS => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (weekId,blockNumber, globalCategoryId) = readTwoIntsAndLong(request_body);
                    get_next_weeks_category_polls(weekId, blockNumber, globalCategoryId);
                }
            }
            codes::URL_TOMORROWS_CATEGORY_POLLS => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId,blockNumber, globalCategoryId) = readTwoIntsAndLong(request_body);
                    get_tomorrows_category_polls(dayId, blockNumber, globalCategoryId);
                }
            }
            codes::URL_DAY_AFTER_TOMORROWS_CATEGORY_POLLS => {
                if wrongRequestLength16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId,blockNumber, globalCategoryId) = readTwoIntsAndLong(request_body);
                    get_day_after_tomorrows_category_polls(dayId, blockNumber, globalCategoryId);
                }
            }

            // Recent Polls by Location Category

            codes::URL_NEXT_MONTHS_LOCATION_CATEGORY_POLLS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (monthId, timezoneId, blockNumber, globalLocationId, globalCategoryId) = readThreeIntsAndTwoLongs(request_body);
                    get_next_months_location_category_polls(monthId, timezoneId, blockNumber, globalLocationId, globalCategoryId);
                }
            }
            codes::URL_NEXT_WEEKS_LOCATION_CATEGORY_POLLS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (weekId, timezoneId, blockNumber, globalLocationId, globalCategoryId) = readThreeIntsAndTwoLongs(request_body);
                    get_next_weeks_location_category_polls(weekId, timezoneId, blockNumber, globalLocationId, globalCategoryId);
                }
            }
            codes::URL_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId, timezoneId, blockNumber, globalLocationId, globalCategoryId) = readThreeIntsAndTwoLongs(request_body);
                    get_tomorrows_location_category_polls(dayId, timezoneId, blockNumber, globalLocationId, globalCategoryId);
                }
            }
            codes::URL_DAY_AFTER_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrongRequestLength28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE
                } else {
                    let (dayId, timezoneId, blockNumber, globalLocationId, globalCategoryId) = readThreeIntsAndTwoLongs(request_body);
                    get_day_after_tomorrows_location_category_polls(dayId, timezoneId, blockNumber, globalLocationId, globalCategoryId);
                }
            }

            _ => {
                codes::INVALID_DATA_FORMAT_RESPONSE
            }
        }
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

#[inline]
fn wrongRequestLength12(requestBody: &[u8]) -> boolean {
    requestBody.len() != 12
}

#[inline]
fn wrongRequestLength16(requestBody: &[u8]) -> boolean {
    requestBody.len() != 16
}

#[inline]
fn wrongRequestLength20(requestBody: &[u8]) -> boolean {
    requestBody.len() != 20
}

#[inline]
fn wrongRequestLength24(requestBody: &[u8]) -> boolean {
    requestBody.len() != 24
}

#[inline]
fn wrongRequestLength28(requestBody: &[u8]) -> boolean {
    requestBody.len() != 28
}

#[inline]
fn readTwoIntsAndLong(requestBody: &[u8]) -> (u32, u32, u64) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn readThreeInts(requestBody: &[u8]) -> (u32, u32, u32) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap()
    );
}

#[inline]
fn readThreeIntsAndLong(requestBody: &[u8]) -> (u32, u32, u32, u64) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn readFourInts(requestBody: &[u8]) -> (u32, u32, u32, u32) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap()
    );
}

#[inline]
fn readThreeIntsAndTwoLongs(requestBody: &[u8]) -> (u32, u32, u32, u64, u64) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u64::<BigEndian>().unwrap(),
        requestDataReader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn readFourIntsAndLong(requestBody: &[u8]) -> (u32, u32, u32, u32, u64) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn readFiveInts(requestBody: &[u8]) -> (u32, u32, u32, u32, u32) {
    let mut requestDataReader = Cursor::new(requestBody);
    return (
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap(),
        requestDataReader.read_u32::<BigEndian>().unwrap()
    );
}
