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
    pub NEXT_MONTH: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub NEXT_WEEK: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub TOMORROW: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub DAY_AFTER_TOMORROW: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
}

impl PollsByLocation {
    pub fn new() -> PollsByLocation {
        PollsByLocation {
            NEXT_MONTH: Vec::with_capacity(NUM_TIMEZONES as usize),
            NEXT_WEEK: Vec::with_capacity(NUM_TIMEZONES as usize),
            TOMORROW: Vec::with_capacity(NUM_TIMEZONES as usize),
            DAY_AFTER_TOMORROW: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}