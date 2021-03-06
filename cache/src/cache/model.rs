use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::LabelCacheIndex;
use common::model::types::LabelId;
use common::model::types::DayId;
use common::model::types::LocationCacheIndex;
use common::model::types::LocationLabelCacheIndex;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

use super::super::logic::add::polls::add_polls_to_per_label_map;

pub struct CachePeriodIds {
    pub day_after_tomorrows_vc_day_id: DayId,
    pub day_b4_yesterdays_vc_day_id: DayId,
    pub this_months_vc_month_id: MonthId,
    pub this_weeks_vc_week_id: WeekId,
    pub last_months_vc_month_id: MonthId,
    pub last_weeks_vc_week_id: WeekId,
    pub next_months_vc_month_id: WeekId,
    pub next_weeks_vc_week_id: WeekId,
    pub todays_vc_day_id: DayId,
    pub tomorrows_vc_day_id: DayId,
    pub yesterdays_vc_day_id: DayId,
}

impl CachePeriodIds {
    pub fn new() -> CachePeriodIds {
        // FIXME: implement based on current day (day of creation)
        CachePeriodIds {
            day_after_tomorrows_vc_day_id: 0,
            day_b4_yesterdays_vc_day_id: 0,
            this_months_vc_month_id: 0,
            this_weeks_vc_week_id: 0,
            last_months_vc_month_id: 0,
            last_weeks_vc_week_id: 0,
            next_months_vc_month_id: 0,
            next_weeks_vc_week_id: 0,
            todays_vc_day_id: 0,
            tomorrows_vc_day_id: 0,
            yesterdays_vc_day_id: 0,
        }
    }
}

pub struct LabelPeriodPollRankings {
    pub max_poll_number_bytes: u8,
    pub num_polls_in_period: u32,
    pub vote_counts_by_label_index: Vec<Vec<VoteCount>>,
}

impl LabelPeriodPollRankings {
    pub fn new(
        max_poll_number_bytes: u8,
        num_polls_in_period: u32,
        num_labels_in_period: usize,
    ) -> LabelPeriodPollRankings {
        LabelPeriodPollRankings {
            max_poll_number_bytes,
            num_polls_in_period,
            vote_counts_by_label_index: Vec::with_capacity(num_labels_in_period),
        }
    }
}

/**
 * Random access data structure needed for initial lookup of a Location+Label poll rankings.
 * Contains time period specific array index of the Location
 *      and a map (by Global Id) of the label indexes for same time period
 */
pub struct LocationPeriodIds {
    pub location_label_cache_index_map: IntHashMap<LabelId, LocationLabelCacheIndex>,
    pub location_cache_index: LabelCacheIndex,
}

impl LocationPeriodIds {
    pub fn new(
        location_cache_index: LocationCacheIndex,
        num_labels: usize,
    ) -> LocationPeriodIds {
        LocationPeriodIds {
            location_cache_index,
            location_label_cache_index_map: HashMap::with_capacity_and_hasher(
                num_labels, IntBuildHasher::default()),
        }
    }
}


/**
Split by timezone:
*/

/**
 *  Vote count data structure needed for looking up Poll Rankings by Vote Count
 *  Contains ranked vote counts for a particular location
 *      and an array (by time period+location specific label index) of location+label
 *          ranked vote counts
 */
pub struct LocationPollRankings {
    pub max_poll_number_bytes: u8,
    pub location: Vec<VoteCount>,
    pub label_locations: Vec<Vec<VoteCount>>,
}

/**
 *  Ordered list of latest added polls (in a future time period).
 *     Contains time ordered polls (in order of creation) for a particular location
 *         and a map/tree (by Global Label Id) of time ordered polls for location+label
 */
pub struct LocationPollPrependLists {

    // Inner vector is a page/frame (Ex: capped @ 1024) and outer vector grows
    pub location: Vec<Vec<PollId>>,

    // PollId in frames of 1024 by LabelId
    /*
    When the update thread is running, it is possible for the poll id page request to be
    failed because the the hashmap must be re-hashed.
    */
    pub label_locations: IntHashMap<LabelId, Vec<Vec<PollId>>>,

    // True if the label_locations map is currently being rehashed
    pub label_locations_rehashing: bool,

}

impl LocationPollPrependLists {



    pub fn add_tomorrows_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        add_polls_to_per_label_map(&mut self.tomorrow, &mut self.tomorrow_rehash,
                                   label_ids, poll_ids);
    }

    pub fn add_day_after_tomorrows_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        add_polls_to_per_label_map(&mut self.day_after_tomorrow, &mut self.day_after_tomorrow_rehash,
                                   label_ids, poll_ids);
    }

    pub fn add_next_weeks_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        add_polls_to_per_label_map(&mut self.next_week, &mut self.next_week_rehash,
                                   label_ids, poll_ids);
    }

    pub fn add_next_months_polls(
        &mut self,
        label_ids: Vec<LabelId>,
        poll_ids: Vec<Vec<PollId>>,
    ) {
        add_polls_to_per_label_map(&mut self.next_month, &mut self.next_month_rehash,
                                   label_ids, poll_ids);
    }

}


/**
 * Transmission details - for future poll time ordered lists a single header with the number of
 * bytes per id is acceptable.  This is because the global poll ids will have close ids (due to
 * creation order) and can be assumed to take up a roughly equal amount of bits for storage.
 * A page level byte counter can be used to pre-compute it (at insertion time).
 *
 * Note for current/past periods same type counter can be used for per timezone/period, computed
 * at creation of the period.
 */



/*
 * With 64bit Dimension Direction Sums:
 *
 * At least upper 3 bytes in sums will be free, we can use this space for
 * additional threshold counts and flags.  Also the total sum of free
 * bytes will be at least 6 to 18.  This could be used to store additional
 * information about the poll.
 *
 * For example, the positional configuration of a 3D poll can be encoded
 * into a number of configurations.  Lets assume that it would take 2 bytes
 * (64K configurations).  In the
 *
 * With 32 bit sums, they will loose precision after about 300M polls (given
 * that vote could take up to 30 spaces (2*3*5), so may need overflow bytes
 * to keep track of overflow and additional computation is needed:
 *
 * let newVal = oldVal + 24
 * if(newVal < oldVal) {
 *  overflow += 1;
 * }
 *
 * Note for pipe compression having 8u + u32 is actually faster, because
 * only 5 bytes need to be checked and serialized vs 8
 */

/**
 * Count of votes contains:
 *   PollType + Timezone - unified in a byte
 *   Id of the poll for that Timezone+period
 *   count of votes
 *   TODO: revisit poll count size if and when needed (perhaps adding an overflow bit)
 */
pub struct VoteCount {
    /**
    First 5 bits are for timezone, last 2 for for Type
    */
    pub poll_type_and_tz: u8,
    pub poll_id: PollId,
    pub count: u32,
}

/*
 * Poll sums and counts for a 3 dimensional poll.
 */
pub struct ThreeDPoll {
    pub dim_1_dir_1_over: u8,
    pub dim_1_dir_2_over: u8,
    pub dim_2_dir_1_over: u8,
    pub dim_2_dir_2_over: u8,
    pub dim_3_dir_1_over: u8,
    pub dim_3_dir_2_over: u8,
    pub dim_1_dir_1_sum: u32,
    pub dim_1_dir_2_sum: u32,
    pub dim_2_dir_1_sum: u32,
    pub dim_2_dir_2_sum: u32,
    pub dim_3_dir_1_sum: u32,
    pub dim_3_dir_2_sum: u32,
    pub vote_count: VoteCount,
}

/*
 * Poll sums and counts for a 2 dimensional poll.
 */
pub struct TwoDPoll {
    pub dim_1_dir_1_over: u8,
    pub dim_1_dir_2_over: u8,
    pub dim_2_dir_1_over: u8,
    pub dim_2_dir_2_over: u8,
    pub dim_1_dir_1_sum: u32,
    pub dim_1_dir_2_sum: u32,
    pub dim_2_dir_1_sum: u32,
    pub dim_2_dir_2_sum: u32,
    pub vote_count: VoteCount,
}

/*
 * Poll sums and counts for a 1 dimensional poll.
 */
pub struct OneDPoll {
    pub dim_1_dir_1_over: u8,
    pub dim_1_dir_2_over: u8,
    pub dim_1_dir_1_sum: u32,
    pub dim_1_dir_2_sum: u32,
    pub vote_count: VoteCount,
}
