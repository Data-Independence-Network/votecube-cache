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
    pub NEXT_MONTH: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub NEXT_WEEK: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub TOMORROW: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub DAY_AFTER_TOMORROW: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {
        let b: IntBuildHasher = IntBuildHasher::default();

        PollsByCategory {
            NEXT_MONTH: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            NEXT_WEEK: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            TOMORROW: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            DAY_AFTER_TOMORROW: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
        }
    }
}