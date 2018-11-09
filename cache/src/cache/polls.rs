use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::types::PollId;

/**
* Polls HashMap by timezone and global id.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub struct Polls<T> {
    pub today: Vec<IntHashMap<PollId, T>>,
    pub yesterday: Vec<IntHashMap<PollId, T>>,
    pub day_b4_yesterday: Vec<IntHashMap<PollId, T>>,
    pub this_week: Vec<IntHashMap<PollId, T>>,
    pub last_week: Vec<IntHashMap<PollId, T>>,
    pub this_month: Vec<IntHashMap<PollId, T>>,
    pub last_month: Vec<IntHashMap<PollId, T>>,
}

impl<T> Polls<T> {
    pub fn new() -> Polls<T> {
        Polls {
            today: Vec::with_capacity(NUM_TIMEZONES as usize),
            yesterday: Vec::with_capacity(NUM_TIMEZONES as usize),
            day_b4_yesterday: Vec::with_capacity(NUM_TIMEZONES as usize),
            this_week: Vec::with_capacity(NUM_TIMEZONES as usize),
            last_week: Vec::with_capacity(NUM_TIMEZONES as usize),
            this_month: Vec::with_capacity(NUM_TIMEZONES as usize),
            last_month: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}
