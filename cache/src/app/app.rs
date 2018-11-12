use common::url::cache::serve;

use server::codes;
use server::cache::app::App;


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

use super::super::logic::serve::rankings::label;
use super::super::logic::serve::rankings::location;
use super::super::logic::serve::rankings::location_label;

use super::super::logic::serve::recent::label::get_day_after_tomorrows_label_polls;
use super::super::logic::serve::recent::label::get_next_months_label_polls;
use super::super::logic::serve::recent::label::get_next_weeks_label_polls;
use super::super::logic::serve::recent::label::get_tomorrows_label_polls;

// use super::super::logic::serve::recent::location_and_loc_label::get_day_after_tomorrows_label_polls;
// use super::super::logic::serve::recent::location_and_loc_label::get_next_months_label_polls;
// use super::super::logic::serve::recent::location_and_loc_label::get_next_weeks_label_polls;
// use super::super::logic::serve::recent::location_and_loc_label::get_tomorrows_label_polls;

use super::super::logic::serve::recent::location::get_day_after_tomorrows_location_polls;
use super::super::logic::serve::recent::location::get_next_months_location_polls;
use super::super::logic::serve::recent::location::get_next_weeks_location_polls;
use super::super::logic::serve::recent::location::get_tomorrows_location_polls;

use super::super::logic::serve::recent::location_label::get_day_after_tomorrows_location_label_polls;
use super::super::logic::serve::recent::location_label::get_next_months_location_label_polls;
use super::super::logic::serve::recent::location_label::get_next_weeks_location_label_polls;
use super::super::logic::serve::recent::location_label::get_tomorrows_location_label_polls;

use super::super::cache::cache_reader::CacheReader;


pub struct CompleteCacheApp {

    pub cache: Box<CacheReader + Send + Sync>,

}


impl CompleteCacheApp {

    pub fn new(
        cache: Box<CacheReader + Send + Sync>
    ) -> CompleteCacheApp {
        CompleteCacheApp {
            cache
        }
    }

}


impl App for CompleteCacheApp {

    fn get_response(
        &self,
        path: &str,
        request_body: &[u8],
    ) -> Vec<u8> {
        match path {
            /*
             *
             *  POLL RANKINGS
             *
             */

            // Label Poll Rankings

            serve::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_this_months_label_rankings_by_global_id(
                        vc_month_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_this_months_label_rankings_by_cache_index(
                        vc_month_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_this_weeks_label_rankings_by_global_id(
                        vc_week_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_this_weeks_label_rankings_by_cache_index(
                        vc_week_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_todays_label_rankings_by_global_id(
                        vc_day_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_TODAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_todays_label_rankings_by_cache_index(
                        vc_day_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_last_months_label_rankings_by_global_id(
                        vc_month_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_last_months_label_rankings_by_cache_index(
                        vc_month_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_last_weeks_label_rankings_by_global_id(
                        vc_week_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_last_weeks_label_rankings_by_cache_index(
                        vc_week_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_yesterdays_label_rankings_by_global_id(
                        vc_day_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_yesterdays_label_rankings_by_cache_index(
                        vc_day_id, block_index, label_cache_index, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_day_b4_yesterdays_label_rankings_by_global_id(
                        vc_day_id, block_index, global_label_id, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_12(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, block_index, label_cache_index)
                    = read_three_ints(request_body);
                    label::get_day_b4_yesterdays_label_rankings_by_cache_index(
                        vc_day_id, block_index, label_cache_index, &self.cache)
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
                        vc_month_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_this_months_location_rankings_by_cache_index(
                        vc_month_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_this_weeks_location_rankings_by_global_id(
                        vc_week_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_this_weeks_location_rankings_by_cache_index(
                        vc_week_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_todays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_TODAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_todays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_last_months_location_rankings_by_global_id(
                        vc_month_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_last_months_location_rankings_by_cache_index(
                        vc_month_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_last_weeks_location_rankings_by_global_id(
                        vc_week_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_week_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_last_weeks_location_rankings_by_cache_index(
                        vc_week_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_yesterdays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_yesterdays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id)
                    = read_three_ints_and_long(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_global_id(
                        vc_day_id, timezone_id, block_index, global_location_id, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_POLL_RANKINGS_BY_CACHE_INDEX => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index)
                    = read_four_ints(request_body);
                    location::get_day_b4_yesterdays_location_rankings_by_cache_index(
                        vc_day_id, timezone_id, block_index, location_cache_index, &self.cache)
                }
            }

            // Location Label Poll Rankings

            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_this_months_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index,
                        global_location_id, global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_this_months_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index,
                        location_cache_index, global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_this_months_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_this_weeks_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_this_weeks_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_THIS_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_this_weeks_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_todays_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_label_id,
                        &self.cache)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_todays_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_TODAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_todays_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_last_months_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_last_months_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_MONTHS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_last_months_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_last_weeks_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_last_weeks_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_LAST_WEEKS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_last_weeks_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_yesterdays_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_label_id,
                        &self.cache)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_yesterdays_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_YESTERDAYS_LOCATION_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_yesterdays_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_GLOBAL_IDS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, global_location_id, global_label_id)
                    = read_three_ints_and_two_longs(request_body);
                    location_label::get_day_b4_yesterdays_location_label_rankings_by_global_ids(
                        vc_day_id, timezone_id, block_index, global_location_id, global_label_id,
                        &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_LOCATION_CACHE_INDEX_AND_GLOBAL_CATEGORY_ID => {
                if wrong_request_length_24(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, global_label_id)
                    = read_four_ints_and_long(request_body);
                    location_label::get_day_b4_yesterdays_location_label_rankings_by_location_cache_index_and_global_label_ids(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_DAY_B4_YESTERDAY_LOCATION_YESTERDAY_CATEGORY_POLL_RANKINGS_BY_CACHE_INDEXES => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_day_id, timezone_id, block_index, location_cache_index, location_label_cache_index)
                    = read_five_ints(request_body);
                    location_label::get_day_b4_yesterdays_location_label_rankings_by_cache_indexes(
                        vc_day_id, timezone_id, block_index, location_cache_index,
                        location_label_cache_index, &self.cache)
                }
            }

            /*
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
                    get_next_months_location_polls(
                        month_id, timezone_id, block_number, global_location_id,
                        &self.cache)
                }
            }
            serve::URL_NEXT_WEEKS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_next_weeks_location_polls(
                        week_id, timezone_id, block_number, global_location_id,
                        &self.cache)
                }
            }
            serve::URL_TOMORROWS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_tomorrows_location_polls(
                        day_id, timezone_id, block_number, global_location_id,
                        &self.cache)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_LOCATION_POLLS => {
                if wrong_request_length_20(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id) = read_three_ints_and_long(request_body);
                    get_day_after_tomorrows_location_polls(
                        day_id, timezone_id, block_number, global_location_id,
                        &self.cache)
                }
            }

            // Recent Polls by Label

            serve::URL_NEXT_MONTHS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (month_id,block_number, global_label_id) = read_two_ints_and_long(request_body);
                    get_next_months_label_polls(
                        month_id, block_number, global_label_id, &self.cache)
                }
            }
            serve::URL_NEXT_WEEKS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id,block_number, global_label_id) = read_two_ints_and_long(request_body);
                    get_next_weeks_label_polls(
                        week_id, block_number, global_label_id, &self.cache)
                }
            }
            serve::URL_TOMORROWS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id,block_number, global_label_id) = read_two_ints_and_long(request_body);
                    get_tomorrows_label_polls(
                        day_id, block_number, global_label_id, &self.cache)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_CATEGORY_POLLS => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id,block_number, global_label_id) = read_two_ints_and_long(request_body);
                    get_day_after_tomorrows_label_polls(
                        day_id, block_number, global_label_id, &self.cache)
                }
            }

            // Recent Polls by Location Label

            serve::URL_NEXT_MONTHS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (month_id, timezone_id, block_number, global_location_id, global_label_id) = read_three_ints_and_two_longs(request_body);
                    get_next_months_location_label_polls(
                        month_id, timezone_id, block_number, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_NEXT_WEEKS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (week_id, timezone_id, block_number, global_location_id, global_label_id) = read_three_ints_and_two_longs(request_body);
                    get_next_weeks_location_label_polls(
                        week_id, timezone_id, block_number, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id, global_label_id) = read_three_ints_and_two_longs(request_body);
                    get_tomorrows_location_label_polls(
                        day_id, timezone_id, block_number, global_location_id,
                        global_label_id, &self.cache)
                }
            }
            serve::URL_DAY_AFTER_TOMORROWS_LOCATION_CATEGORY_POLLS => {
                if wrong_request_length_28(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (day_id, timezone_id, block_number, global_location_id, global_label_id) = read_three_ints_and_two_longs(request_body);
                    get_day_after_tomorrows_location_label_polls(
                        day_id, timezone_id, block_number, global_location_id,
                        global_label_id, &self.cache)
                }
            }

            _ => {
                codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
            }
        }
    }

    fn get_update_response(
        &self,
        path: &str,
        request_body: &[u8],
    ) -> Vec<u8> {
        match path {
            serve::URL_THIS_MONTHS_CATEGORY_POLL_RANKINGS_BY_GLOBAL_ID => {
                if wrong_request_length_16(request_body) {
                    codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
                } else {
                    let (vc_month_id, block_index, global_label_id)
                    = read_two_ints_and_long(request_body);
                    label::get_this_months_label_rankings_by_global_id(
                        vc_month_id, block_index, global_label_id, &self.cache)
                }
            }
            _ => {
                codes::INVALID_DATA_FORMAT_RESPONSE.to_vec()
            }
        }
    }

}
