use int_hash::IntHashMap;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::CategoryId;
use super::super::super::super::cache::cache::DayId;
use super::super::super::super::cache::cache::GLOBAL_CATEGORY_TZ_INDEX;
use super::super::super::super::cache::cache::MonthId;
use super::super::super::super::cache::cache::PollId;
use super::super::super::super::cache::cache::WeekId;
use super::super::super::super::data::prepend::GlobalNode;
use super::super::super::super::server::codes;

use super::utils::get2ByteRecentPollIds;
use super::utils::get3ByteRecentPollIds;
use super::utils::get4ByteRecentPollIds;
use super::utils::get5ByteRecentPollIds;
use super::utils::get6ByteRecentPollIds;
use super::utils::get7ByteRecentPollIds;
use super::utils::get8ByteRecentPollIds;

const noResults: Vec<u8> = Vec::new();

pub fn get_tomorrows_category_polls(
    vcDayId: DayId,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.tomorrowsVcDayId,
        vcDayId,
        cache::TOMORROWS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::TOMORROWS_POLL_ID_BYTE_COUNTS[GLOBAL_CATEGORY_TZ_INDEX]);
}

pub fn get_day_after_tomorrows_category_polls(
    vcDayId: DayId,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayAfterTomorrowsVcDayId,
        vcDayId,
        cache::DAY_AFTER_TOMORROWS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS[GLOBAL_CATEGORY_TZ_INDEX]);
}

pub fn get_next_weeks_category_polls(
    vcWeekId: WeekId,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.nextWeeksVcWeekId,
        vcWeekId,
        cache::NEXT_WEEKS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::NEXT_WEEKS_POLL_ID_BYTE_COUNTS[GLOBAL_CATEGORY_TZ_INDEX]);
}

pub fn get_next_months_category_polls(
    vcMonthId: MonthId,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_category_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.nextMonthsVcMonthId,
        vcMonthId,
        cache::NEXT_MONTHS_POLLS_BY_CATEGORY,
        blockNumber, globalCategoryId,
        cache::NEXT_MONTHS_POLL_ID_BYTE_COUNTS[GLOBAL_CATEGORY_TZ_INDEX]);
}


fn get_global_category_polls(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    globalCategoryPolls: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    // 1 based index
    blockNumber: u32,
    globalCategoryId: CategoryId,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let categoryPolls: Vec<Vec<PollId>> = match globalCategoryPolls.get(*globalCategoryId) {
        None => {
            return noResults;
        }
        Some(polls) => {
            polls
        }
    };
    let pollsBlock: Vec<PollId> = match categoryPolls.get(categoryPolls.len() - blockNumber) {
        None => {
            return noResults;
        }
        Some(block) => {
            block
        }
    };
    let mut response: Vec<u8> = Vec::with_capacity(maxPollNumberBytes * pollsBlock.len() + 1);

    match maxPollNumberBytes {
        3 => {
            response.push(0b00000011);
            return get3ByteRecentPollIds(pollsBlock, response);
        }
        4 => {
            response.push(0b00000100);
            return get4ByteRecentPollIds(pollsBlock, response);
        }
        2 => {
            response.push(0b00000010);
            return get2ByteRecentPollIds(pollsBlock, response);
        }
        5 => {
            response.push(0b00000101);
            return get5ByteRecentPollIds(pollsBlock, response);
        }
        6 => {
            response.push(0b00000110);
            return get6ByteRecentPollIds(pollsBlock, response);
        }
        7 => {
            response.push(0b00000111);
            return get7ByteRecentPollIds(pollsBlock, response);
        }
        8 => {
            response.push(0b00000000);
            return get8ByteRecentPollIds(pollsBlock, response);
        }
    }
}