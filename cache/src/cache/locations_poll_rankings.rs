use common::model::timezone::NUM_TIMEZONES;

use super::model::LocationPollRankings;

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
pub struct LocationsPollRankings {
    pub LAST_MONTH: Vec<Vec<LocationPollRankings>>,
    pub THIS_MONTH: Vec<Vec<LocationPollRankings>>,

    pub LAST_WEEK: Vec<Vec<LocationPollRankings>>,
    pub THIS_WEEK: Vec<Vec<LocationPollRankings>>,

    pub DAY_B4_YESTERDAY: Vec<Vec<LocationPollRankings>>,
    pub YESTERDAY: Vec<Vec<LocationPollRankings>>,
    pub TODAY: Vec<Vec<LocationPollRankings>>,
}

impl LocationsPollRankings {
    pub fn new() -> LocationsPollRankings {
        LocationsPollRankings {
            LAST_MONTH: Vec::with_capacity(NUM_TIMEZONES as usize),
            THIS_MONTH: Vec::with_capacity(NUM_TIMEZONES as usize),

            LAST_WEEK: Vec::with_capacity(NUM_TIMEZONES as usize),
            THIS_WEEK: Vec::with_capacity(NUM_TIMEZONES as usize),

            DAY_B4_YESTERDAY: Vec::with_capacity(NUM_TIMEZONES as usize),
            YESTERDAY: Vec::with_capacity(NUM_TIMEZONES as usize),
            TODAY: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}
