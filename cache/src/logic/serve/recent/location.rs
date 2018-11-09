use int_hash::IntHashMap;

use common::model::types::DayId;
use common::model::types::LocationId;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::TimezoneId;
use common::model::types::WeekId;

use super::super::super::super::cache::cache::Cache;
use super::super::super::super::cache::model::LocationPollPrependLists;

use super::super::super::super::server::codes;

use super::utils::get_2_byte_recent_poll_ids;
use super::utils::get_3_byte_recent_poll_ids;
use super::utils::get_4_byte_recent_poll_ids;
use super::utils::get_5_byte_recent_poll_ids;
use super::utils::get_6_byte_recent_poll_ids;
use super::utils::get_7_byte_recent_poll_ids;
use super::utils::get_8_byte_recent_poll_ids;

const NO_RESULTS: Vec<u8> = Vec::new();

pub fn get_tomorrows_location_polls(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_location_polls(
        cache.category_cache_period_ids.tomorrows_vc_day_id,
        vc_day_id,
        timezone_id,
        cache.polls_by_location.TOMORROW,
        block_index,
        global_location_id,
        cache.poll_id_byte_counts.TOMORROW[timezone_id as usize],
    );
}

pub fn get_day_after_tomorrows_location_polls(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_location_polls(
        cache.category_cache_period_ids.day_after_tomorrows_vc_day_id,
        vc_day_id,
        timezone_id,
        cache.polls_by_location.DAY_AFTER_TOMORROW,
        block_index,
        global_location_id,
        cache.poll_id_byte_counts.DAY_AFTER_TOMORROW[timezone_id as usize],
    );
}

pub fn get_next_weeks_location_polls(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_location_polls(
        cache.category_cache_period_ids.next_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        cache.polls_by_location.NEXT_WEEK,
        block_index,
        global_location_id,
        cache.poll_id_byte_counts.NEXT_WEEK[timezone_id as usize],
    );
}

pub fn get_next_months_location_polls(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_location_polls(
        cache.category_cache_period_ids.next_months_vc_month_id,
        vc_month_id,
        timezone_id,
        cache.polls_by_location.NEXT_MONTH,
        block_index,
        global_location_id,
        cache.poll_id_byte_counts.NEXT_MONTH[timezone_id as usize],
    );
}


fn get_global_location_polls(
    current_period_id: u32,
    expected_period_id: u32,
    timezone_id: TimezoneId,
    location_polls_by_timezone: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    // 1 based index
    block_number: u32,
    global_location_id: LocationId,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let location_polls_for_timezone: IntHashMap<LocationId, LocationPollPrependLists> =
        match location_polls_by_timezone.get(timezone_id as usize) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
            }
            Some(futurePolls) => {
                *futurePolls
            }
        };

    let location_polls: LocationPollPrependLists = match location_polls_for_timezone.get(&global_location_id) {
        None => {
            return NO_RESULTS;
        }
        Some(locationPolls) => {
            *locationPolls
        }
    };

    let polls_block: Vec<PollId> = match location_polls.location.get(location_polls.location.len() - block_number as usize) {
        None => {
            return NO_RESULTS;
        }
        Some(block) => {
            *block
        }
    };
    let mut response: Vec<u8> = Vec::with_capacity(max_poll_number_bytes as usize * polls_block.len() + 1);

    match max_poll_number_bytes {
        3 => {
            response.push(0b00000011);
            return get_3_byte_recent_poll_ids(polls_block, response);
        }
        4 => {
            response.push(0b00000100);
            return get_4_byte_recent_poll_ids(polls_block, response);
        }
        5 => {
            response.push(0b00000101);
            return get_5_byte_recent_poll_ids(polls_block, response);
        }
        6 => {
            response.push(0b00000110);
            return get_6_byte_recent_poll_ids(polls_block, response);
        }
        7 => {
            response.push(0b00000111);
            return get_7_byte_recent_poll_ids(polls_block, response);
        }
        8 => {
            response.push(0b00000000);
            return get_8_byte_recent_poll_ids(polls_block, response);
        }
        2 => {
            response.push(0b00000010);
            return get_2_byte_recent_poll_ids(polls_block, response);
        }
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }

}
