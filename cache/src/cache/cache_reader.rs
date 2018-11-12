use common::model::timezone::NUM_TIMEZONES;

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


pub trait CacheReader {
    /**
     * Ids of currently cached time periods, across all timezones
     */
    fn get_category_cache_period_ids(&self) -> &CachePeriodIds;
    
    /**
     * Ids of currently cached time periods, per timezone
     */
    fn get_per_timezone_cache_period_ids(&self) -> &[CachePeriodIds; NUM_TIMEZONES as usize];
    /**
     *  Maximum number of bytes taken by poll ids of a given current/future cache period.
     */
    fn get_poll_id_byte_counts(&self) -> &PollIdByteCounts;

    /**
     * Global time period ids across timezones
     */
    fn get_time_period_ids(&self) -> &TimePeriodIds;

    // Keeps track of when a timezone is being modified
    fn get_time_zone_modification_flags(&self) -> &[bool; NUM_TIMEZONES as usize];

    /**
     *  Random access Category Id map, needed by initial lookup from clients.
     */
    fn get_category_index_map(&self) -> &CategoryIndexMap;
    /**
     *  Random access Location + Category Id map, needed by initial lookup from clients.
     */
    fn get_location_category_index_map(&self) -> &LocationCategoryIndexMap;
    /**
     *  Random access Location Id map, needed by initial lookup from clients.
     */
    fn get_location_index_map(&self) -> &LocationIndexMap;

    /**
     * Poll rankings by Category (past & present).
     */
    fn get_category_poll_rankings(&self) -> &CategoryPollRankings;
    /**
     * Poll rankings by Location (past & present).
     */
    fn get_location_poll_rankings(&self) -> &LocationsPollRankings;

    /**
     *  Future PollIds by Category.
     */
    fn get_future_polls_by_category(&self) -> &PollsByCategory;
    /**
     *  Future PollIds by Location.
     */
    fn get_future_polls_by_location(&self) -> &PollsByLocation;

    /**
     *  Polls (with counts) for past & present time periods.
     */
    fn get_polls_1_d(&self) -> &Polls<OneDPoll>;
    fn get_polls_2_d(&self) -> &Polls<TwoDPoll>;
    fn get_polls_3_d(&self) -> &Polls<ThreeDPoll>;

}
