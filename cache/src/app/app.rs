use common::url::cache::serve;

use server::codes;
use server::cache::server::App;


use server::read::read_five_ints;
use server::read::read_four_ints;
use server::read::read_four_ints_and_long;
use server::read::read_three_ints;
use server::read::read_three_ints_and_long;
use server::read::read_three_ints_and_two_longs;
use server::read::read_two_ints_and_long;

use server::read::wrong_request_length_12;
use server::read::wrong_request_length_16;
use server::read::wrong_request_length_20;
use server::read::wrong_request_length_24;
use server::read::wrong_request_length_28;

use super::super::logic::serve::rankings::category;
use super::super::logic::serve::rankings::location;
use super::super::logic::serve::rankings::location_category;

use super::super::logic::serve::recent::category::get_day_after_tomorrows_category_polls;
use super::super::logic::serve::recent::category::get_next_months_category_polls;
use super::super::logic::serve::recent::category::get_next_weeks_category_polls;
use super::super::logic::serve::recent::category::get_tomorrows_category_polls;

// use super::super::logic::serve::recent::location_and_loc_category::get_day_after_tomorrows_category_polls;
// use super::super::logic::serve::recent::location_and_loc_category::get_next_months_category_polls;
// use super::super::logic::serve::recent::location_and_loc_category::get_next_weeks_category_polls;
// use super::super::logic::serve::recent::location_and_loc_category::get_tomorrows_category_polls;

use super::super::logic::serve::recent::location::get_day_after_tomorrows_location_polls;
use super::super::logic::serve::recent::location::get_next_months_location_polls;
use super::super::logic::serve::recent::location::get_next_weeks_location_polls;
use super::super::logic::serve::recent::location::get_tomorrows_location_polls;

use super::super::logic::serve::recent::location_category::get_day_after_tomorrows_location_category_polls;
use super::super::logic::serve::recent::location_category::get_next_months_location_category_polls;
use super::super::logic::serve::recent::location_category::get_next_weeks_location_category_polls;
use super::super::logic::serve::recent::location_category::get_tomorrows_location_category_polls;


pub struct CompleteCacheApp<T: 'static + Send> {

    cache: T,

}


impl<T: Send> CompleteCacheApp<T> {

    pub fn new(
        cache: T
    ) -> CompleteCacheApp<T> {
        CompleteCacheApp {
            cache
        }
    }

}


impl<T: Send> App<T> for CompleteCacheApp<T> {

    fn get_response(
        &self,
        path: &str,
        request_body: &[u8],
    ) -> Vec<u8> {
        match path {
            /**
             *
             *  POLL RANKINGS
             *
             */

            // Category Poll Rankings

            serve::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_this_months_category_rankings_by_global_id(
                        vc_month_id, block_index, global_category_id)
                }
            }
            serve::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_this_months_category_rankings_by_cache_index(
                        vc_month_id, block_index, category_cache_index)
                }
            }
            serve::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_this_weeks_category_rankings_by_global_id(
                        vc_week_id, block_index, global_category_id)
                }
            }
            serve::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_this_weeks_category_rankings_by_cache_index(
                        vc_week_id, block_index, category_cache_index)
                }
            }
            serve::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_todays_category_rankings_by_global_id(
                        vc_day_id, block_index, global_category_id)
                }
            }
            serve::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_todays_category_rankings_by_cache_index(
                        vc_day_id, block_index, category_cache_index)
                }
            }
            serve::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_last_months_category_rankings_by_global_id(
                        vc_month_id, block_index, global_category_id)
                }
            }
            serve::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_last_months_category_rankings_by_cache_index(
                        vc_month_id, block_index, category_cache_index)
                }
            }
            serve::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_last_weeks_category_rankings_by_global_id(
                        vc_week_id, block_index, global_category_id)
                }
            }
            serve::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_last_weeks_category_rankings_by_cache_index(
                        vc_week_id, block_index, category_cache_index)
                }
            }
            serve::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_yesterdays_category_rankings_by_global_id(
                        vc_day_id, block_index, global_category_id)
                }
            }
            serve::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_yesterdays_category_rankings_by_cache_index(
                        vc_day_id, block_index, category_cache_index)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_category_id)
                    = read_two_ints_and_long(request_body);
                    category::get_day_b4_yesterdays_category_rankings_by_global_id(
                        vc_day_id, block_index, global_category_id)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, category_cache_index)
                    = read_three_ints(request_body);
                    category::get_day_b4_yesterdays_category_rankings_by_cache_index(
                        vc_day_id, block_index, category_cache_index)
                }
            }

            // Location Poll Rankings

            serve::URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_this_months_location_rankings_by_global_id(
                        vc_month_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_this_months_location_rankings_by_cache_index(
                        vc_month_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_this_weeks_location_rankings_by_global_id(
                        vc_week_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_this_weeks_location_rankings_by_cache_index(
                        vc_week_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_todays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_todays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_last_months_location_rankings_by_global_id(
                        vc_month_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_last_months_location_rankings_by_cache_index(
                        vc_month_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_last_weeks_location_rankings_by_global_id(
                        vc_week_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_last_weeks_location_rankings_by_cache_index(
                        vc_week_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_yesterdays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_yesterdays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index)
                }
            }

            // Location Category Poll Rankings

            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_this_months_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_this_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_this_months_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_this_weeks_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_todays_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_todays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_todays_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_last_months_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_last_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_last_months_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_last_weeks_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_yesterdays_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_category_id)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                    = read_four_ints_and_long(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index, global_category_id)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                    = read_five_ints(request_body);
                    location_category::get_day_b4_yesterdays_location_category_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index, location_category_cache_index)
                }
            }

            /**
             *
             *  RECENT POLLS
             *
             */

            // Recent Polls by Location

            serve::URL_NEXT_MONTHS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (month_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_next_months_location_polls(month_id, timezone_id, block_number, global_location_id)
                }
            }
            serve::URL_NEXT_WEEKS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_next_weeks_location_polls(week_id, timezone_id, block_number, global_location_id)
                }
            }
            serve::URL_TOMORROWS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_tomorrows_location_polls(day_id, timezone_id, block_number, global_location_id)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_day_after_tomorrows_location_polls(day_id, timezone_id, block_number, global_location_id)
                }
            }

            // Recent Polls by Category

            serve::URL_NEXT_MONTHS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (month_id,block_number, global_category_id) = read_two_ints_and_long(request_body);
                    get_next_months_category_polls(month_id, block_number, global_category_id)
                }
            }
            serve::URL_NEXT_WEEKS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id,block_number, global_category_id) = read_two_ints_and_long(request_body);
                    get_next_weeks_category_polls(week_id, block_number, global_category_id)
                }
            }
            serve::URL_TOMORROWS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id,block_number, global_category_id) = read_two_ints_and_long(request_body);
                    get_tomorrows_category_polls(day_id, block_number, global_category_id)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id,block_number, global_category_id) = read_two_ints_and_long(request_body);
                    get_day_after_tomorrows_category_polls(day_id, block_number, global_category_id)
                }
            }

            // Recent Polls by Location Category

            serve::URL_NEXT_MONTHS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (month_id, timezone_id, block_number, global_location_id, global_category_id) = read_three_ints_and_two_longs(request_body);
                    get_next_months_location_category_polls(month_id, timezone_id, block_number, global_location_id, global_category_id)
                }
            }
            serve::URL_NEXT_WEEKS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id, timezone_id, block_number, global_location_id, global_category_id) = read_three_ints_and_two_longs(request_body);
                    get_next_weeks_location_category_polls(week_id, timezone_id, block_number, global_location_id, global_category_id)
                }
            }
            serve::URL_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id, global_category_id) = read_three_ints_and_two_longs(request_body);
                    get_tomorrows_location_category_polls(day_id, timezone_id, block_number, global_location_id, global_category_id)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id, global_category_id) = read_three_ints_and_two_longs(request_body);
                    get_day_after_tomorrows_location_category_polls(day_id, timezone_id, block_number, global_location_id, global_category_id)
                }
            }

            _ => {
                codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
            }
        }
    }
}
