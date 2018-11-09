use int_hash::IntHashMap;

use common::model::consts::GLOBAL_TZ_INDEX;

use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

use super::super::super::super::cache::cache::Cache;
use super::super::super::super::server::codes;

use super::utils::get_2_byte_recent_poll_ids;
use super::utils::get_3_byte_recent_poll_ids;
use super::utils::get_4_byte_recent_poll_ids;
use super::utils::get_5_byte_recent_poll_ids;
use super::utils::get_6_byte_recent_poll_ids;
use super::utils::get_7_byte_recent_poll_ids;
use super::utils::get_8_byte_recent_poll_ids;

pub fn get_tomorrows_category_polls(
    vc_day_id: DayId,
    // 1 based index
    block_number: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_category_polls(
        cache.category_cache_period_ids.tomorrows_vc_day_id,
        vc_day_id,
        &cache.polls_by_category.TOMORROW,
        block_number, global_category_id,
        cache.poll_id_byte_counts.TOMORROW[GLOBAL_TZ_INDEX as usize]);
}

pub fn get_day_after_tomorrows_category_polls(
    vc_day_id: DayId,
    // 1 based index
    block_number: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_category_polls(
        cache.category_cache_period_ids.day_after_tomorrows_vc_day_id,
        vc_day_id,
        &cache.polls_by_category.DAY_AFTER_TOMORROW,
        block_number, global_category_id,
        cache.poll_id_byte_counts.DAY_AFTER_TOMORROW[GLOBAL_TZ_INDEX as usize]);
}

pub fn get_next_weeks_category_polls(
    vc_week_id: WeekId,
    // 1 based index
    block_number: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_category_polls(
        cache.category_cache_period_ids.next_weeks_vc_week_id,
        vc_week_id,
        &cache.polls_by_category.NEXT_WEEK,
        block_number, global_category_id,
        cache.poll_id_byte_counts.NEXT_WEEK[GLOBAL_TZ_INDEX as usize]);
}

pub fn get_next_months_category_polls(
    vc_month_id: MonthId,
    // 1 based index
    block_number: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_global_category_polls(
        cache.category_cache_period_ids.next_months_vc_month_id,
        vc_month_id,
        &cache.polls_by_category.NEXT_MONTH,
        block_number, global_category_id,
        cache.poll_id_byte_counts.NEXT_MONTH[GLOBAL_TZ_INDEX as usize]);
}


fn get_global_category_polls(
    current_period_id: u32,
    expected_period_id: u32,
    global_category_polls: &IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    // 1 based index
    block_number: u32,
    global_category_id: CategoryId,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let category_polls: &Vec<Vec<PollId>> = match global_category_polls.get(&global_category_id) {
        None => {
            return Vec::new();
        }
        Some(polls) => {
            polls
        }
    };
    let polls_block: &Vec<PollId> = match category_polls.get(category_polls.len() - (block_number as usize)) {
        None => {
            return Vec::new();
        }
        Some(block) => {
            block
        }
    };
    let mut response: Vec<u8> = Vec::with_capacity(max_poll_number_bytes as usize * polls_block.len()  + 1);

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