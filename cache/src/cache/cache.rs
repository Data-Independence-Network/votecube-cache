//use std::collections::HashMap;

//use int_hash::IntBuildHasher;
//use int_hash::IntHashMap;



use common::model::timezone::NUM_TIMEZONES;
use common::model::timezone::ALL_TIME_ZONES;
use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

use super::category_index_map::CategoryIndexMap;
use super::category_poll_rankings::CategoryPollRankings;
use super::location_category_index_map::LocationCategoryIndexMap;
use super::locations_poll_rankings::LocationsPollRankings;
use super::location_index_map::LocationIndexMap;
use super::model::CachePeriodIds;
use super::model::OneDPoll;
use super::model::ThreeDPoll;
use super::model::TwoDPoll;
use super::poll_id_byte_counts::PollIdByteCounts;
use super::polls::Polls;
use super::polls_by_category::PollsByCategory;
use super::polls_by_location::PollsByLocation;
use super::time_period_ids::TimePeriodIds;


pub struct Cache {
    /**
     * Ids of currently cached time periods, across all timezones
     */
    pub category_cache_period_ids: CachePeriodIds,
    /**
     * Ids of currently cached time periods, per timezone
     */
    pub per_timezone_cache_period_ids: [CachePeriodIds; NUM_TIMEZONES as usize],
    /**
     *  Maximum number of bytes taken by poll ids of a given current/future cache period.
     */
    pub poll_id_byte_counts: PollIdByteCounts,

    /**
     * Global time period ids across timezones
     */
    pub time_period_ids: TimePeriodIds,
    // Keeps track of when a timezone is being modified
    pub time_zone_modification_flags: [bool; NUM_TIMEZONES as usize],

    /**
     *  Random access Category Id map, needed by initial lookup from clients.
     */
    pub category_index_map: CategoryIndexMap,
    /**
     *  Random access Location + Category Id map, needed by initial lookup from clients.
     */
    pub location_category_index_map: LocationCategoryIndexMap,
    /**
     *  Random access Location Id map, needed by initial lookup from clients.
     */
    pub location_index_map: LocationIndexMap,

    /**
     * Poll rankings by Category (past & present).
     */
    pub category_poll_rankings: CategoryPollRankings,
    /**
     * Poll rankings by Location (past & present).
     */
    pub location_poll_rankings: LocationsPollRankings,

    /**
     *  Future PollIds by Category.
     */
    pub future_polls_by_category: PollsByCategory,
    /**
     *  Future PollIds by Location.
     */
    pub future_polls_by_location: PollsByLocation,

    /**
     *  Polls (with counts) for past & present time periods.
     */
    pub polls_1_d: Polls<OneDPoll>,
    pub polls_2_d: Polls<TwoDPoll>,
    pub polls_3_d: Polls<ThreeDPoll>,

}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            category_cache_period_ids: CachePeriodIds::new(),
            per_timezone_cache_period_ids: [
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new(),
                CachePeriodIds::new()
            ],
            poll_id_byte_counts: PollIdByteCounts::new(),
            time_period_ids: TimePeriodIds::new(),
            time_zone_modification_flags: [
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false,
                false
            ],

            category_index_map: CategoryIndexMap::new(),
            location_category_index_map: LocationCategoryIndexMap::new(),
            location_index_map: LocationIndexMap::new(),

            category_poll_rankings: CategoryPollRankings::new(),
            location_poll_rankings: LocationsPollRankings::new(),

            future_polls_by_category: PollsByCategory::new(),
            future_polls_by_location: PollsByLocation::new(),

            polls_1_d: Polls::new(),
            polls_2_d: Polls::new(),
            polls_3_d: Polls::new(),
        }
    }

    pub fn add_future_day_polls(
        &mut self,
        day_id: DayId,
        category_ids: Vec<CategoryId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        let tomorrows_day_id = self.time_period_ids.tomorrow[ALL_TIME_ZONES];
        if day_id == tomorrows_day_id {
            &self.future_polls_by_category.add_tomorrows_polls(category_ids, poll_ids);
        } else if day_id == tomorrows_day_id + 1 {
            &self.future_polls_by_category.add_day_after_tomorrows_polls(category_ids, poll_ids);
        } else if day_id < tomorrows_day_id {
            // TOO LATE TO ADD
            return;
        } else {
            // TOO EARLY TO ADD
        }
    }

    pub fn add_future_week_polls(
        &mut self,
        week_id: WeekId,
        category_ids: Vec<CategoryId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        let next_week_id = self.time_period_ids.next_week[ALL_TIME_ZONES];
        if week_id == next_week_id {
            &self.future_polls_by_category.add_next_weeks_polls(category_ids, poll_ids);
        } else if week_id < next_week_id {
            // TOO LATE TO ADD
            return;
        } else {
            // TOO EARLY TO ADD
        }
    }

    pub fn add_future_month_polls(
        &mut self,
        month_id: MonthId,
        category_ids: Vec<CategoryId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        let next_month_id = self.time_period_ids.next_month[ALL_TIME_ZONES];
        if month_id == next_month_id {
            &self.future_polls_by_category.add_next_months_polls(category_ids, poll_ids);
        } else if month_id < next_month_id {
            // TOO LATE TO ADD
            return;
        } else {
            // TOO EARLY TO ADD
        }
    }

}

//pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
//pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();

/*
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
