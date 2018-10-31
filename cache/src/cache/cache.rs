use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::timezone::NUM_TIMEZONES_WITH_GLOBAL_CATEGORY;
use common::model::types::CategoryCacheIndex;
use common::model::types::CategoryId;
use common::model::types::LocationId;
use common::model::types::PollId;

use super::model::CachePeriodIds;
use super::model::LocationPeriodIds;
use super::model::LocationPollPrependLists;
use super::model::LocationPollRankings;
use super::model::OneDPoll;
use super::model::ThreeDPoll;
use super::model::TwoDPoll;
use super::model::VoteCount;

pub struct Cache {
    
}

/**
 * Global time period ids across timezones, maintained at the same time as data is moved
 * in timezone chunks between, current and past (and future).
 *
 * Used to verify client requests, to make sure that their requests are still valid.
 */
pub static mut LAST_MONTH_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_MONTH_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut NEXT_MONTH_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut LAST_WEEK_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_WEEK_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut NEXT_WEEK_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut DAY_B4_YESTERDAY_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut YESTERDAY_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut TODAY_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut TOMORROW_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut DAY_AFTER_TOMORROW_IDS: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub unsafe fn get_last_month_id(
    timezone_index: u32
) -> u32 {
    LAST_MONTH_IDS[timezone_index as usize]
}
pub unsafe fn get_this_month_id(
    timezone_index: u32
) -> u32 {
    THIS_MONTH_IDS[timezone_index as usize]
}
pub unsafe fn get_next_month_id(
    timezone_index: u32
) -> u32 {
    NEXT_MONTH_IDS[timezone_index as usize]
}
pub unsafe fn get_last_week_id(
    timezone_index: u32
) -> u32 {
    LAST_WEEK_IDS[timezone_index as usize]
}
pub unsafe fn get_this_week_id(
    timezone_index: u32
) -> u32 {
    THIS_WEEK_IDS[timezone_index as usize]
}
pub unsafe fn get_next_week_id(
    timezone_index: u32
) -> u32 {
    NEXT_WEEK_IDS[timezone_index as usize]
}
pub unsafe fn get_day_b4_yesterday_id(
    timezone_index: u32
) -> u32 {
    DAY_B4_YESTERDAY_IDS[timezone_index as usize]
}
pub unsafe fn get_yesterday_id(
    timezone_index: u32
) -> u32 {
    YESTERDAY_IDS[timezone_index as usize]
}
pub unsafe fn get_today_id(
    timezone_index: u32
) -> u32 {
    TODAY_IDS[timezone_index as usize]
}
pub unsafe fn get_tomorrow_id(
    timezone_index: u32
) -> u32 {
    TOMORROW_IDS[timezone_index as usize]
}
pub unsafe fn get_day_after_tomorrow_id(
    timezone_index: u32
) -> u32 {
    DAY_AFTER_TOMORROW_IDS[timezone_index as usize]
}

/**
 *  Maximum number of bytes taken by poll ids of a given current of future cache period.
 */
pub static mut LAST_MONTHS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_MONTHS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut NEXT_MONTHS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub static mut LAST_WEEKS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut THIS_WEEKS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut NEXT_WEEKS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub static mut DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut YESTERDAYS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut TODAYS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut TOMORROWS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS: [u32; NUM_TIMEZONES_WITH_GLOBAL_CATEGORY] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub unsafe fn get_last_months_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    LAST_MONTHS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_this_months_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    THIS_MONTHS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_next_months_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    NEXT_MONTHS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_last_weeks_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    LAST_WEEKS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_this_weeks_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    THIS_WEEKS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_next_weeks_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    NEXT_WEEKS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_day_b4_yesterdays_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_yesterdays_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    YESTERDAYS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_todays_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    TODAYS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_tomorrows_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    TOMORROWS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}
pub unsafe fn get_day_after_tomorrows_poll_id_byte_count(
    timezone_index: u32
) -> u32 {
    DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS[timezone_index as usize]
}


/**
 * Ids of currently cached time periods, across all timezones
 */
pub static mut CATEGORY_CACHE_PERIOD_IDS: CachePeriodIds = CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    };

pub unsafe fn get_category_cache_period_ids(
    timezone_index: u32
) -> &'static CachePeriodIds {
    &CATEGORY_CACHE_PERIOD_IDS
}

/**
 * Ids of currently cached time periods, per timezone
 */
pub static mut PER_TIMEZONE__CACHE_PERIOD_IDS: [CachePeriodIds; NUM_TIMEZONES] = [
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
    CachePeriodIds {
        day_after_tomorrows_vc_day_id: 0,
        day_b4_yesterdays_vc_day_id: 0,
        this_months_vc_month_id: 0,
        this_weeks_vc_week_id: 0,
        last_months_vc_month_id: 0,
        last_weeks_vc_week_id: 0,
        next_months_vc_month_id: 0,
        next_weeks_vc_week_id: 0,
        todays_vc_day_id: 0,
        tomorrows_vc_day_id: 0,
        yesterdays_vc_day_id: 0,
    },
];

pub unsafe fn get_timezone_cache_period_ids(
    timezone_index: u32
) -> &'static CachePeriodIds {
    &PER_TIMEZONE__CACHE_PERIOD_IDS[timezone_index]
}

//pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
//pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();

// Keeps track of when a timezone is being modified
pub static mut TIMEZONE_MODIFICATION_FLAGS: Vec<bool> = Vec::with_capacity(NUM_TIMEZONES);

/**
 *  Future period prepend data structures for adding recent polls.
 *    By:   timezoneId
 *              locationId
 *                  categoryId
 *  Contain only the prepended Poll Ids
 */
pub static mut NEXT_MONTHS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut NEXT_WEEKS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);

pub static b: IntBuildHasher = IntBuildHasher::default();

/**
 *  Future period prepend data structures for per category access.
 *      By:     categoryId
 *  Contain only the prepended Poll Ids
 */
pub static mut NEXT_MONTHS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = HashMap::with_capacity_and_hasher(1000000, b);
pub static mut NEXT_WEEKS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = HashMap::with_capacity_and_hasher(1000000, b);
pub static mut TOMORROWS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = HashMap::with_capacity_and_hasher(1000000, b);
pub static mut DAY_AFTER_TOMORROWS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = HashMap::with_capacity_and_hasher(1000000, b);

/**
 *  Random access Category and Location Id maps, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub static mut CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut CATEGORY_TODAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = HashMap::with_capacity_and_hasher(2000, b);

pub static mut LOCATION_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);

pub static mut LOCATION_CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);
pub static mut LOCATION_CATEGORY_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = HashMap::with_capacity_and_hasher(2000, b);

/**
 *  The location/based poll rankings nested arrays by:
 *      Timezone Id
 *          Location Id
 *  Internally each LocationPollRanking contains another array by:
 *      Category Id
 *
 *  Location and Location+Category Ids are initially looked up via the Random Access maps.
 *  Subsequently, the client knows the time period specific ids and uses them for direct access.
 */
pub static mut LAST_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

pub static mut LAST_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

pub static mut DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

/**
 * Poll rankings by Category.
 * Q: Global category lookups are meant to cross timezone boundaries but how to maintain that?
 *
 * 1)  Maintain only per-location/per-category rankings
 *
 * 2)  Dynamically add and remove polls from category rankings as the go in and out of scope for each
 * day (probably too hard at the moment).
 *
 * 3)  Maintain only previous period rankings (doable now) - Implementing
 *
 * 3a)  Actually, today's category rankings can be made available after UTC-8 (West Coast) passes
 * its poll add deadline (10pm) for the next day.  At that point there are still 9-10 hours left
 * in the day in Japan (depending on daylight savings).
 */
pub static mut LAST_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut LAST_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut TODAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();


/**
 * Random access current poll maps, needed for count and sum increments by the voting servers.
 *    Indexed by global PollIds
 *
 *  TODO: many not be needed, assuming that timezone is known at the time of
 *  count and sum increments
 */
//pub static mut TODAY_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut TODAY_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut TODAY_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_WEEK_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = HashMap::with_capacity_and_hasher(2000, b);
//pub static mut THIS_MONTH_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = HashMap::with_capacity_and_hasher(2000, b);

/**
* Polls HashMap by timezone and global id.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub static mut TODAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_1_D__POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_1_D__POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
