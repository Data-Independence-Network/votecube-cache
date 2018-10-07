use int_hash::IntHashMap;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::CategoryId;
use super::super::super::super::cache::cache::DayId;
use super::super::super::super::cache::cache::LocationId;
use super::super::super::super::cache::cache::LocationPollPrependLists;
use super::super::super::super::cache::cache::MonthId;
use super::super::super::super::cache::cache::PollId;
use super::super::super::super::cache::cache::TimezoneId;
use super::super::super::super::cache::cache::WeekId;

use super::super::super::super::server::codes;

use super::utils::get2ByteRecentPollIds;
use super::utils::get3ByteRecentPollIds;
use super::utils::get4ByteRecentPollIds;
use super::utils::get5ByteRecentPollIds;
use super::utils::get6ByteRecentPollIds;
use super::utils::get7ByteRecentPollIds;
use super::utils::get8ByteRecentPollIds;

pub fn get_tomorrows_location_category_polls(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_location_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.tomorrowsVcDayId,
        vcDayId,
        timezoneId,
        cache::TOMORROWS_POLLS_BY_LOCATION,
        blockIndex,
        globalLocationId,
        globalCategoryId,
        cache::TOMORROWS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_day_after_tomorrows_category_location_polls(
    vcDayId: DayId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_location_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayAfterTomorrowsVcDayId,
        vcDayId,
        timezoneId,
        cache::DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION,
        blockIndex,
        globalLocationId,
        globalCategoryId,
        cache::DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_next_weeks_category_location_polls(
    vcWeekId: WeekId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_location_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.nextWeeksVcWeekId,
        vcDayId,
        timezoneId,
        cache::NEXT_WEEKS_POLLS_BY_LOCATION,
        blockIndex,
        globalLocationId,
        globalCategoryId,
        cache::NEXT_WEEKS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}

pub fn get_next_months_category_location_polls(
    vcMonthId: MonthId,
    timezoneId: TimezoneId,
    blockIndex: u32,
    globalLocationId: LocationId,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_global_location_polls(
        cache::CATEGORY_CACHE_PERIOD_IDS.nextMonthsVcMonthId,
        vcDayId,
        timezoneId,
        cache::NEXT_MONTHS_POLLS_BY_LOCATION,
        blockIndex,
        globalLocationId,
        globalCategoryId,
        cache::NEXT_MONTHS_POLL_ID_BYTE_COUNTS[timezoneId],
    );
}


fn get_global_location_category_polls(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    timezoneId: TimezoneId,
    locationPollsByTimezone: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    // 1 based index
    blockNumber: u32,
    globalLocationId: LocationId,
    globalCategoryId: CategoryId,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let locationPollsForTimezone: IntHashMap<LocationId, LocationPollPrependLists> =
        match locationPollsByTimezone.get(timezoneId) {
            None => {
                return codes::INVALID_TIMEZONE_ID_RESPONSE;
            }
            Some(futurePolls) => {
                futurePolls
            }
        };

    let locationPolls: LocationPollPrependLists = match locationPollsByTimezone.get(*globalLocationId) {
        None => {
            return noResults;
        }
        Some(locationPolls) => {
            locationPolls
        }
    };

    let locationCategoryPolls: LocationPollPrependLists = match locationPolls.categoryLocations.get(*globalCategoryId) {
        None => {
            return noResults;
        }
        Some(locationCategoryPolls) => {
            locationCategoryPolls
        }
    };

    let pollsBlock: Vec<PollId> = match locationCategoryPolls.get(locationPolls.len() - blockNumber) {
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
