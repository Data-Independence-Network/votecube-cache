use common::model::timezone::NUM_TIMEZONES;

use super::model::LocationPollRankings;

/**
 *  The location/based poll rankings nested arrays by:
 *      Timezone Id
 *          Location Id
 *  Internally each LocationPollRanking contains another array by:
 *      Label Id
 *
 *  Location and Location+Label Ids are initially looked up via the Random Access maps.
 *  Subsequently, the client knows the time period specific ids and uses them for direct access.
 */
pub struct LocationsPollRankings {
    pub last_month: Vec<Vec<LocationPollRankings>>,
    pub this_month: Vec<Vec<LocationPollRankings>>,

    pub last_week: Vec<Vec<LocationPollRankings>>,
    pub this_week: Vec<Vec<LocationPollRankings>>,

    pub day_b4_yesterday: Vec<Vec<LocationPollRankings>>,
    pub yesterday: Vec<Vec<LocationPollRankings>>,
    pub today: Vec<Vec<LocationPollRankings>>,
}

impl LocationsPollRankings {
    pub fn new() -> LocationsPollRankings {
        LocationsPollRankings {
            last_month: Vec::with_capacity(NUM_TIMEZONES as usize),
            this_month: Vec::with_capacity(NUM_TIMEZONES as usize),

            last_week: Vec::with_capacity(NUM_TIMEZONES as usize),
            this_week: Vec::with_capacity(NUM_TIMEZONES as usize),

            day_b4_yesterday: Vec::with_capacity(NUM_TIMEZONES as usize),
            yesterday: Vec::with_capacity(NUM_TIMEZONES as usize),
            today: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}
