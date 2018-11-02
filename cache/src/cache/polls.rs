use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::PollId;

pub struct Polls1D {
    TODAY: Vec<IntHashMap<PollId, OneDPoll>>,
    YESTERDAY: Vec<IntHashMap<PollId, OneDPoll>>,
    DAY_B4_YESTERDAY: Vec<IntHashMap<PollId, OneDPoll>>,
    THIS_WEEK: Vec<IntHashMap<PollId, OneDPoll>>,
    LAST_WEEK: Vec<IntHashMap<PollId, OneDPoll>>,
    THIS_MONTH: Vec<IntHashMap<PollId, OneDPoll>>,
    LAST_MONTH: Vec<IntHashMap<PollId, OneDPoll>>,
}

impl Polls1D {
    pub fn new() -> Polls1D {
        Polls1D {
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
