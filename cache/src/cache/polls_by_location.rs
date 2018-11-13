use std::time::Duration;
use std::thread;

use int_hash::IntHashMap;
use common::model::timezone::NUM_TIMEZONES;
use common::model::types::LabelId;
use common::model::types::LocationId;
use common::model::types::PollId;
use common::model::types::TimezoneId;
use super::model::LocationPollPrependLists;

/**
    The timeout needed for any existing requests to finish
    before rehashing starts.  Assuming very fast requests.
 */
const NUM_MILLIS_TO_SLEEP_BEFORE_HASHMAP_REHASH: u64 = 10;

/**
 *  Future period prepend data structures for adding recent polls.
 *    By:   timezoneId
 *              location_id
 *                  labelId
 *  Contain only the prepended Poll Ids
 */
pub struct PollsByLocation {
    pub next_month: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub next_month_rehashing: Vec<bool>,
    pub next_week: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub next_week_rehashing: Vec<bool>,
    pub tomorrow: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub tomorrow_rehashing: Vec<bool>,
    pub day_after_tomorrow: Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
    pub day_after_tomorrow_rehashing: Vec<bool>,
}

pub struct PollsForLocation {
    pub label_ids: Vec<LabelId>,
    pub location_id: LocationId,
    pub polls: Vec<PollId>,
    pub polls_per_label: Vec<Vec<LabelId>>,
}

pub struct PollsForLocations {
    pub location_ids: Vec<LocationId>,
    pub polls_for_locations: Vec<PollsForLocation>,
}

/*
 A poll addition server may be sending polls to be added to the same time period (Day/Week/Month)
 but across multiple timezones.  This brings up a question, how do you add a poll to a particular
 period in edge case timezone that is close to the Pacific Ocean?  If we stick with the timezone
 boundaries then PST/PDT won't be able to add polls to tomorrow after 8PM.  By the same token,
 the biggest timezone (by population) in the world won't be able to add polls for the day after
 tomorrow until 8AM of two days before.   Currently this appears to be an acceptable scenario,
 going with it.

 Note, this will impact voting as well, since there must be time to prepare the voting count
 data structures after the lock-down.  Sticking with a global time-frame makes recording category
 counts easy but adds additional load at the end of each day to prepare all of the data structures
 (globally) at the same time.

 So, alternatively addition of polls to a particular timezone might be allowed until "reasonable"
 limits" for that timezone.  That is a more complex approach and may not be needed for MVP.

 So, with the "global" approach, add lock-down could finish 1 hour before the end of the day in UTC.
 This means that UTC-8 timezone must be done with the polls @3PM.  On the flip side the UTC+8
 timezone cannot enter polls until 9AM (due to 1 hour lock down).

 NOTE, this affects how voting is done, and locking people in UTC-8 to 3PM on voting is not
 acceptable.

 So an alternative approach may be to keep daily per-category stats with an overlap.  So,
 tomorrows poll ids would be there for few extra hours and would make it there a few hours early.
 Technically they could be there for 2 calendar days with a 12 hour overlap each way (12 before and
 12 after).

 Same would be true for "global location polls".  Every timezone would keep their polls according
 to time boundaries of that timezone.  The continent-wide polls would have boundaries that are
 timezone-central.  Of course, it would be better for locations that span multiple timezones to
 have polls
 */
pub struct PollsForLocationsInTimezones {
    pub timezone_ids: Vec<TimezoneId>,
    pub polls_for_locations: Vec<PollsForLocations>,
}

impl PollsByLocation {
    pub fn new() -> PollsByLocation {
        PollsByLocation {
            next_month: Vec::with_capacity(NUM_TIMEZONES as usize),
            next_month_rehashing: Vec::with_capacity(NUM_TIMEZONES as usize),
            next_week: Vec::with_capacity(NUM_TIMEZONES as usize),
            next_week_rehashing: Vec::with_capacity(NUM_TIMEZONES as usize),
            tomorrow: Vec::with_capacity(NUM_TIMEZONES as usize),
            tomorrow_rehashing: Vec::with_capacity(NUM_TIMEZONES as usize),
            day_after_tomorrow: Vec::with_capacity(NUM_TIMEZONES as usize),
            day_after_tomorrow_rehashing: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }

    pub fn add_tomorrows_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
//        add_polls_to_per_label_map(&mut self.tomorrow, &mut self.tomorrow_rehash,
//                                   label_ids, poll_ids);
    }

    pub fn add_day_after_tomorrows_polls(
        &mut self,
        polls_for_location: PollsForLocation,
    ) {
        add_polls_to_time_period(polls_for_location)
//        add_polls_to_per_label_map(&mut self.day_after_tomorrow, &mut self.day_after_tomorrow_rehash,
//                                   label_ids, poll_ids);
    }

    fn add_polls_to_time_period(
        time_period: &mut Vec<IntHashMap<LocationId, LocationPollPrependLists>>,
        rehash_vec: &mut Vec<bool>,
        polls_for_locations: PollsForLocations,
    ) {
        let mut missing_location_ids: Vec<LabelId> = Vec::new();
        let mut missing_poll_ids: Vec<Vec<PollId>> = Vec::new();

        let mut i = 0;
        for location_id in polls_for_locations.location_ids {
            let label_id = label_ids[i];
            if poll_map.contains_key(&label_id) {
                insert_polls_ids_to_per_label_map(poll_map, poll_ids_to_add, label_id, false);
            } else {
                missing_location_ids.push(label_id);
                missing_poll_ids.push(poll_ids_to_add);
            }

            i += 1;
        }

        let num_missing_labels = missing_location_ids.len();
        if num_missing_labels == 0 {
            return;
        }

        let spare_poll_map_capacity = poll_map.capacity() - poll_map.len();
        if spare_poll_map_capacity < num_missing_labels {
            std::thread::sleep(Duration::from_millis(NUM_MILLIS_TO_SLEEP_BEFORE_HASHMAP_REHASH));
            *rehash = true;
        }

        let mut i = 0;

        for poll_ids_to_add in missing_poll_ids {
            let label_id = missing_location_ids[i];

            insert_polls_ids_to_per_label_map(poll_map, poll_ids_to_add, label_id, true);

            i += 1;
        }
        *rehash = false;
    }

    pub fn add_next_weeks_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
//        add_polls_to_per_label_map(&mut self.next_week, &mut self.next_week_rehash,
//                                   label_ids, poll_ids);
    }

    pub fn add_next_months_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
//        add_polls_to_per_label_map(&mut self.next_month, &mut self.next_month_rehash,
//                                   label_ids, poll_ids);
    }

}