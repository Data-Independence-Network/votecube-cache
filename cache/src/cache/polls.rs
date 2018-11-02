use int_hash::IntHashMap;

use common::model::timezone::NUM_TIMEZONES;
use common::model::types::PollId;

/**
* Polls HashMap by timezone and global id.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub struct Polls<T> {
    TODAY: Vec<IntHashMap<PollId, T>>,
    YESTERDAY: Vec<IntHashMap<PollId, T>>,
    DAY_B4_YESTERDAY: Vec<IntHashMap<PollId, T>>,
    THIS_WEEK: Vec<IntHashMap<PollId, T>>,
    LAST_WEEK: Vec<IntHashMap<PollId, T>>,
    THIS_MONTH: Vec<IntHashMap<PollId, T>>,
    LAST_MONTH: Vec<IntHashMap<PollId, T>>,
}

impl<T> Polls<T> {
    pub fn new() -> Polls<T> {
        Polls {
            TODAY: Vec::with_capacity(NUM_TIMEZONES),
            YESTERDAY: Vec::with_capacity(NUM_TIMEZONES),
            DAY_B4_YESTERDAY: Vec::with_capacity(NUM_TIMEZONES),
            THIS_WEEK: Vec::with_capacity(NUM_TIMEZONES),
            LAST_WEEK: Vec::with_capacity(NUM_TIMEZONES),
            THIS_MONTH: Vec::with_capacity(NUM_TIMEZONES),
            LAST_MONTH: Vec::with_capacity(NUM_TIMEZONES),
        }
    }
}
