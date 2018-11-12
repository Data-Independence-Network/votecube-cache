use common::model::timezone::NUM_TIMEZONES;

use super::label_index_map::LabelIndexMap;
use super::label_poll_rankings::LabelPollRankings;
use super::location_label_index_map::LocationLabelIndexMap;
use super::locations_poll_rankings::LocationsPollRankings;
use super::location_index_map::LocationIndexMap;
use super::model::CachePeriodIds;
use super::model::OneDPoll;
use super::model::ThreeDPoll;
use super::model::TwoDPoll;
use super::poll_id_byte_counts::PollIdByteCounts;
use super::polls::Polls;
use super::polls_by_label::PollsByLabel;
use super::polls_by_location::PollsByLocation;
use super::time_period_ids::TimePeriodIds;


pub trait CacheReader {
    /**
     * Ids of currently cached time periods, across all timezones
     */
    fn get_label_cache_period_ids(&self) -> &CachePeriodIds;
    
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
     *  Random access Label Id map, needed by initial lookup from clients.
     */
    fn get_label_index_map(&self) -> &LabelIndexMap;
    /**
     *  Random access Location + Label Id map, needed by initial lookup from clients.
     */
    fn get_location_label_index_map(&self) -> &LocationLabelIndexMap;
    /**
     *  Random access Location Id map, needed by initial lookup from clients.
     */
    fn get_location_index_map(&self) -> &LocationIndexMap;

    /**
     * Poll rankings by Label (past & present).
     */
    fn get_label_poll_rankings(&self) -> &LabelPollRankings;
    /**
     * Poll rankings by Location (past & present).
     */
    fn get_location_poll_rankings(&self) -> &LocationsPollRankings;

    /**
     *  Future PollIds by Label.
     */
    fn get_future_polls_by_label(&self) -> &PollsByLabel;
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
