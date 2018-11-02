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
pub struct LocationPollRankings {
    LAST_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,
    THIS_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,

    LAST_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,
    THIS_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,

    DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,
    YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,
    TODAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>>,
}

impl LocationPollRankings {
    pub fn new() -> LocationPollRankings {
        LocationPollRankings {
            LAST_MONTHS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),
            THIS_MONTHS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),

            LAST_WEEKS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),
            THIS_WEEKS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),

            DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),
            YESTERDAYS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),
            TODAYS_LOCATION_POLL_RANKINGS: Vec::with_capacity(NUM_TIMEZONES),
        }
    }
}
