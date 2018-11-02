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
    LAST_MONTH: Vec<Vec<LocationPollRankings>>,
    THIS_MONTH: Vec<Vec<LocationPollRankings>>,

    LAST_WEEK: Vec<Vec<LocationPollRankings>>,
    THIS_WEEK: Vec<Vec<LocationPollRankings>>,

    DAY_B4_YESTERDAY: Vec<Vec<LocationPollRankings>>,
    YESTERDAY: Vec<Vec<LocationPollRankings>>,
    TODAY: Vec<Vec<LocationPollRankings>>,
}

impl LocationsPollRankings {
    pub fn new() -> LocationsPollRankings {
        LocationsPollRankings {
            LAST_MONTH: Vec::with_capacity(NUM_TIMEZONES),
            THIS_MONTH: Vec::with_capacity(NUM_TIMEZONES),

            LAST_WEEK: Vec::with_capacity(NUM_TIMEZONES),
            THIS_WEEK: Vec::with_capacity(NUM_TIMEZONES),

            DAY_B4_YESTERDAY: Vec::with_capacity(NUM_TIMEZONES),
            YESTERDAY: Vec::with_capacity(NUM_TIMEZONES),
            TODAY: Vec::with_capacity(NUM_TIMEZONES),
        }
    }
}
