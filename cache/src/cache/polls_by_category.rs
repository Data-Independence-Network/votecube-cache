use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::PollId;

/**
 *  Future period prepend data structures for per category access.
 *      By:     categoryId
 *  Contain only the prepended Poll Ids
 */
pub struct PollsByCategory {
    pub next_month: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub next_week: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub day_after_tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {
        PollsByCategory {
            next_month: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            next_week: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            day_after_tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
        }
    }
}