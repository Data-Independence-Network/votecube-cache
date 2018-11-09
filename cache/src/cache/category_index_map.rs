use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryCacheIndex;
use common::model::types::CategoryId;
use common::model::types::LocationId;
use common::model::types::PollId;

/**
 *  Random access Category Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct CategoryIndexMap {
    pub LAST_MONTH: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub THIS_MONTH: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub LAST_WEEK: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub THIS_WEEK: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub DAY_B4_YESTERDAY: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub YESTERDAY: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub TODAY: IntHashMap<CategoryId, CategoryCacheIndex>,
}

impl CategoryIndexMap {
    pub fn new() -> CategoryIndexMap {
        CategoryIndexMap {
            LAST_MONTH: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            THIS_MONTH: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            LAST_WEEK: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            THIS_WEEK: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            DAY_B4_YESTERDAY: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            YESTERDAY: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            TODAY: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
        }
    }
}