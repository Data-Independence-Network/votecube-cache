use int_hash::IntHashMap;
use common::model::timezone::NUM_TIMEZONES;
use common::model::types::LocationId;
use super::model::LocationPollPrependLists;

/**
 *  Future period prepend data structures for adding recent polls.
 *    By:   timezoneId
 *              locationId
 *                  categoryId
 *  Contain only the prepended Poll Ids
 */
pub struct PollsByLocation {
    NEXT_MONTHS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    NEXT_WEEKS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
}

impl PollsByLocation {
    pub fn new() -> PollsByLocation {
        PollsByLocation {
            NEXT_MONTHS_POLLS_BY_LOCATION: Vec::with_capacity(NUM_TIMEZONES),
            NEXT_WEEKS_POLLS_BY_LOCATION: Vec::with_capacity(NUM_TIMEZONES),
            TOMORROWS_POLLS_BY_LOCATION: Vec::with_capacity(NUM_TIMEZONES),
            DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION: Vec::with_capacity(NUM_TIMEZONES),
        }
    }
}