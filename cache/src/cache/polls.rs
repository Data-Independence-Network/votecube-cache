use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::types::PollId;

/**
* Polls HashMap by timezone and global id.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub struct Polls<T> {
    pub TODAY: Vec<IntHashMap<PollId, T>>,
    pub YESTERDAY: Vec<IntHashMap<PollId, T>>,
    pub DAY_B4_YESTERDAY: Vec<IntHashMap<PollId, T>>,
    pub THIS_WEEK: Vec<IntHashMap<PollId, T>>,
    pub LAST_WEEK: Vec<IntHashMap<PollId, T>>,
    pub THIS_MONTH: Vec<IntHashMap<PollId, T>>,
    pub LAST_MONTH: Vec<IntHashMap<PollId, T>>,
}

impl<T> Polls<T> {
    pub fn new() -> Polls<T> {
        Polls {
            TODAY: Vec::with_capacity(NUM_TIMEZONES as usize),
            YESTERDAY: Vec::with_capacity(NUM_TIMEZONES as usize),
            DAY_B4_YESTERDAY: Vec::with_capacity(NUM_TIMEZONES as usize),
            THIS_WEEK: Vec::with_capacity(NUM_TIMEZONES as usize),
            LAST_WEEK: Vec::with_capacity(NUM_TIMEZONES as usize),
            THIS_MONTH: Vec::with_capacity(NUM_TIMEZONES as usize),
            LAST_MONTH: Vec::with_capacity(NUM_TIMEZONES as usize),
        }
    }
}
