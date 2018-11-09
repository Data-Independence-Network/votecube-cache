use std::mem::transmute;
use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::types::CategoryId;
use common::model::types::CategoryCacheIndex;
use common::model::types::DayId;
use common::model::types::LocationId;
use common::model::types::LocationCacheIndex;
use common::model::types::LocationCategoryCacheIndex;
use common::model::types::MonthId;
use common::model::types::TimezoneId;
use common::model::types::WeekId;

use super::super::super::super::server::codes;
use super::super::super::super::cache::cache::Cache;
use super::super::super::super::cache::model::CachePeriodIds;
use super::super::super::super::cache::model::LocationPeriodIds;
use super::super::super::super::cache::model::LocationPollRankings;
use super::super::super::super::cache::model::VoteCount;

use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES;
use super::location_and_loc_category::INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES;
use super::location_and_loc_category::get_2_byte_recent_polls;
use super::location_and_loc_category::get_3_byte_recent_polls;
use super::location_and_loc_category::get_4_byte_recent_polls;
use super::location_and_loc_category::get_5_byte_recent_polls;
use super::location_and_loc_category::get_6_byte_recent_polls;
use super::location_and_loc_category::get_7_byte_recent_polls;
use super::location_and_loc_category::get_8_byte_recent_polls;

// NOTE: max page size must fin into u16
const PAGE_SIZE: u32 = 1024;

pub fn get_todays_location_category_rankings_by_global_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.todays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.today,
        &cache.location_poll_rankings.today,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.today[timezone_id as usize],
    );
}

pub fn get_todays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.todays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.today,
        &cache.location_poll_rankings.today,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.today[timezone_id as usize],
    );
}

pub fn get_todays_location_category_rankings_by_cache_indexes(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.todays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_poll_rankings.today,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.today[timezone_id as usize],
    );
}

pub fn get_yesterdays_location_category_rankings_by_global_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.yesterday,
        &cache.location_poll_rankings.yesterday,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.yesterday[timezone_id as usize],
    );
}

pub fn get_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.yesterday,
        &cache.location_poll_rankings.yesterday,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.yesterday[timezone_id as usize],
    );
}

pub fn get_yesterdays_location_category_rankings_by_cache_indexes(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_poll_rankings.yesterday,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.yesterday[timezone_id as usize],
    );
}

pub fn get_day_b4_yesterdays_location_category_rankings_by_global_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.day_b4_yesterday,
        &cache.location_poll_rankings.day_b4_yesterday,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.day_b4_yesterday[timezone_id as usize],
    );
}

pub fn get_day_b4_yesterdays_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_category_index_map.day_b4_yesterday,
        &cache.location_poll_rankings.day_b4_yesterday,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.day_b4_yesterday[timezone_id as usize],
    );
}

pub fn get_day_b4_yesterdays_location_category_rankings_by_cache_indexes(
    vc_day_id: DayId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        timezone_id,
        &cache.location_poll_rankings.day_b4_yesterday,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.day_b4_yesterday[timezone_id as usize],
    );
}

pub fn get_this_weeks_location_category_rankings_by_global_ids(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.this_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_category_index_map.this_week,
        &cache.location_poll_rankings.this_week,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.this_week[timezone_id as usize],
    );
}

pub fn get_this_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.this_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_category_index_map.this_week,
        &cache.location_poll_rankings.this_week,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.this_week[timezone_id as usize],
    );
}

pub fn get_this_weeks_location_category_rankings_by_cache_indexes(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.this_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_poll_rankings.this_week,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.this_week[timezone_id as usize],
    );
}

pub fn get_last_weeks_location_category_rankings_by_global_ids(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.last_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_category_index_map.last_week,
        &cache.location_poll_rankings.last_week,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.last_week[timezone_id as usize],
    );
}

pub fn get_last_weeks_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.last_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_category_index_map.last_week,
        &cache.location_poll_rankings.last_week,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.last_week[timezone_id as usize],
    );
}

pub fn get_last_weeks_location_category_rankings_by_cache_indexes(
    vc_week_id: WeekId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.last_weeks_vc_week_id,
        vc_week_id,
        timezone_id,
        &cache.location_poll_rankings.last_week,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.last_week[timezone_id as usize],
    );
}

pub fn get_this_months_location_category_rankings_by_global_ids(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.this_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_category_index_map.this_month,
        &cache.location_poll_rankings.this_month,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.this_month[timezone_id as usize],
    );
}

pub fn get_this_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.this_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_category_index_map.this_month,
        &cache.location_poll_rankings.this_month,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.this_month[timezone_id as usize],
    );
}

pub fn get_this_months_location_category_rankings_by_cache_indexes(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: u32,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.this_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_poll_rankings.this_month,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.this_month[timezone_id as usize],
    );
}

pub fn get_last_months_location_category_rankings_by_global_ids(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_global_ids(
        current_period_ids.last_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_category_index_map.last_month,
        &cache.location_poll_rankings.last_month,
        global_location_id,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.last_month[timezone_id as usize],
    );
}

pub fn get_last_months_location_category_rankings_by_location_cache_index_and_global_category_ids(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_location_cache_index_and_global_category_id(
        current_period_ids.last_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_category_index_map.last_month,
        &cache.location_poll_rankings.last_month,
        location_cache_index,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.last_month[timezone_id as usize],
    );
}

pub fn get_last_months_location_category_rankings_by_cache_indexes(
    vc_month_id: MonthId,
    timezone_id: TimezoneId,
    block_index: u32,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    if timezone_id >= NUM_TIMEZONES {
        return codes::INVALID_TIMEZONE_ID_RESPONSE.to_vec();
    }

    let current_period_ids: &CachePeriodIds
    = cache.per_timezone_cache_period_ids.get(timezone_id as usize).unwrap();

    return get_location_category_rankings_by_cache_indexes(
        current_period_ids.last_months_vc_month_id,
        vc_month_id,
        timezone_id,
        &cache.location_poll_rankings.last_month,
        location_cache_index,
        location_category_cache_index,
        block_index,
        cache.poll_id_byte_counts.last_month[timezone_id as usize],
    );
}

#[inline]
fn get_location_category_rankings_by_global_ids(
    current_period_id: u32,
    expected_period_id: u32,
    timezone_id: TimezoneId,
    location_category_index_map: &IntHashMap<LocationId, LocationPeriodIds>,
    given_period_location_poll_rankings: &Vec<Vec<LocationPollRankings>>,
    global_location_id: LocationId,
    global_category_id: CategoryId,
    block_index: u32,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let location_period_ids: &LocationPeriodIds = match location_category_index_map.get(&global_location_id) {
        None => {
            return codes::INVALID_GLOBAL_LOCATION_ID_RESPONSE.to_vec();
        }
        Some(location_period_ids) => {
            location_period_ids
        }
    };

    let location_category_cache_index: LocationCategoryCacheIndex = match location_period_ids
        .location_category_cache_index_map.get(&global_category_id) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE.to_vec();
        }
        Some(category_cache_index) => {
            *category_cache_index
        }
    };


    let location_cache_index: LocationCacheIndex = location_period_ids.location_cache_index;
    let location_poll_rankings = given_period_location_poll_rankings
        .get(timezone_id as usize).unwrap().get(location_cache_index as usize).unwrap();
    let first_record_index = PAGE_SIZE * block_index;

    return get_location_category_rankings_with_cache_indexes(
         first_record_index as usize, location_cache_index,
         location_category_cache_index,
        &location_poll_rankings, max_poll_number_bytes);
}

#[inline]
fn get_location_category_rankings_by_location_cache_index_and_global_category_id(
    current_period_id: u32,
    expected_period_id: u32,
    timezone_id: TimezoneId,
    location_category_index_map: &IntHashMap<LocationId, LocationPeriodIds>,
    given_period_location_poll_rankings: &Vec<Vec<LocationPollRankings>>,
    location_cache_index: LocationCacheIndex,
    global_category_id: CategoryId,
    block_index: u32,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let location_poll_rankings: &LocationPollRankings
    = match given_period_location_poll_rankings.get(timezone_id as usize)
    .unwrap().get(location_cache_index as usize) {
        None => {
            return codes::INVALID_LOCATION_CACHE_INDEX_RESPONSE.to_vec();
        }
        Some(location_poll_rankings) => {
            location_poll_rankings
        }
    };

    let location_period_ids: &LocationPeriodIds
    = location_category_index_map.get(&(location_cache_index as u64)).unwrap();

    let location_category_cache_index: u32 = match location_period_ids
        .location_category_cache_index_map.get(&global_category_id) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE.to_vec();
        }
        Some(category_cache_index) => {
            *category_cache_index
        }
    };

//    let location_poll_rankings = given_period_location_poll_rankings
// .get(timezone_id as usize).unwrap().get(location_cache_index as usize).unwrap();
    let first_record_index = PAGE_SIZE * block_index;

    return get_location_category_rankings_with_category_cache_index(
        first_record_index as usize, location_category_cache_index,
        &location_poll_rankings, max_poll_number_bytes);
}

#[inline]
fn get_location_category_rankings_by_cache_indexes(
    current_period_id: u32,
    expected_period_id: u32,
    timezone_id: TimezoneId,
    given_period_location_poll_rankings: &Vec<Vec<LocationPollRankings>>,
    location_cache_index: LocationCacheIndex,
    location_category_cache_index: LocationCategoryCacheIndex,
    block_index: u32,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let location_poll_rankings: &LocationPollRankings
    = match given_period_location_poll_rankings.get(timezone_id as usize)
    .unwrap().get(location_cache_index as usize) {
        None => {
            return codes::INVALID_LOCATION_CACHE_INDEX_RESPONSE.to_vec();
        }
        Some(location_poll_rankings) => {
            location_poll_rankings
        }
    };

    let location_category_vote_counts: &Vec<VoteCount>
    = match location_poll_rankings.category_locations.get(location_category_cache_index as usize) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE.to_vec();
        }
        Some(location_category_vote_counts) => {
            location_category_vote_counts
        }
    };

    let first_record_index = PAGE_SIZE * block_index;

    return get_location_category_rankings(
        first_record_index as usize, location_category_vote_counts,
        max_poll_number_bytes);
}

#[inline]
fn get_location_category_rankings_with_cache_indexes(
    first_record_index: usize,
    location_cache_index: LocationCacheIndex,
    category_cache_index: CategoryCacheIndex,
    location_poll_rankings: &LocationPollRankings,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_location
    = location_poll_rankings.category_locations.get(category_cache_index as usize).unwrap();
    let location_cache_index_bytes: [u8; 4] = unsafe {
        transmute(location_cache_index)
    };
    let category_cache_index_bytes: [u8; 4] = unsafe {
        transmute(category_cache_index)
    };

    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);;
            response.push(0b00000011);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_3_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);;
            response.push(0b00000100);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_4_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);;
            response.push(0b00000101);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_5_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);;
            response.push(0b00000110);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_6_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);;
            response.push(0b00000111);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_7_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);;
            response.push(0b00000000);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_8_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);;
            response.push(0b00000010);
            response.extend_from_slice(&location_cache_index_bytes);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_2_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_location_category_rankings_with_category_cache_index(
    first_record_index: usize,
    category_cache_index: CategoryCacheIndex,
    location_poll_rankings: &LocationPollRankings,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_location
    = location_poll_rankings.category_locations.get(category_cache_index as usize).unwrap();
    let category_cache_index_bytes: [u8; 4] = unsafe {
        transmute(category_cache_index)
    };

    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);;
            response.push(0b00000011);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_3_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);;
            response.push(0b00000100);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_4_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);;
            response.push(0b00000101);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_5_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);;
            response.push(0b00000110);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_6_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);;
            response.push(0b00000111);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_7_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);;
            response.push(0b00000000);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_8_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);;
            response.push(0b00000010);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_2_byte_recent_polls(vote_counts_for_location, first_record_index as usize, response);
        }
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_location_category_rankings(
    first_record_index: usize,
    location_category_vote_counts: &Vec<VoteCount>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);;
            response.push(0b00000011);

            return get_3_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);;
            response.push(0b00000100);

            return get_4_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);;
            response.push(0b00000101);

            return get_5_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);;
            response.push(0b00000110);

            return get_6_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);;
            response.push(0b00000111);

            return get_7_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);;
            response.push(0b00000000);

            return get_8_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);;
            response.push(0b00000010);

            return get_2_byte_recent_polls(location_category_vote_counts, first_record_index as usize, response);
        }
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}
