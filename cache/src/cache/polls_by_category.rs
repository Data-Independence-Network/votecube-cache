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
    NEXT_MONTH: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    NEXT_WEEK: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    TOMORROW: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    DAY_AFTER_TOMORROW: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {
        let b: IntBuildHasher = IntBuildHasher::default();

        PollsByCategory {
            NEXT_MONTH: HashMap::with_capacity_and_hasher(1000000, b),
            NEXT_WEEK: HashMap::with_capacity_and_hasher(1000000, b),
            TOMORROW: HashMap::with_capacity_and_hasher(1000000, b),
            DAY_AFTER_TOMORROW: HashMap::with_capacity_and_hasher(1000000, b),
        }
    }
}