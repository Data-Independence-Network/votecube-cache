use int_hash::IntHashMap;
use common::model::timezone::NUM_TIMEZONES;
use common::model::types::LocationId;
use super::model::LocationPollPrependLists;

/**
 *  Future period prepend data structures for adding recent polls.
 *    By:   timezoneId
 *              locationId
 *                  labelId
 *  Contain only the prepended Poll Ids
 */
pub struct PollsByLocation {
    pub next_month: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub next_week: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub tomorrow: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub day_after_tomorrow: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
}

impl PollsByLocation {
    pub fn new() -> PollsByLocation {
        PollsByLocation {
            next_month: Vec::with_capacity(NUM_TIMEZONES as usize),
            next_week: Vec::with_capacity(NUM_TIMEZONES as usize),
            tomorrow: Vec::with_capacity(NUM_TIMEZONES as usize),
            day_after_tomorrow: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}