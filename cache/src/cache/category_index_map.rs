use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryCacheIndex;
use common::model::types::CategoryId;

/**
 *  Random access Category Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct CategoryIndexMap {
    pub last_month: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub this_month: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub last_week: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub this_week: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub day_b4_yesterday: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub yesterday: IntHashMap<CategoryId, CategoryCacheIndex>,
    pub today: IntHashMap<CategoryId, CategoryCacheIndex>,
}

impl CategoryIndexMap {
    pub fn new() -> CategoryIndexMap {
        CategoryIndexMap {
            last_month: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            this_month: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            last_week: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            this_week: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            day_b4_yesterday: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            yesterday: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
            today: HashMap::with_capacity_and_hasher(2000, IntBuildHasher::default()),
        }
    }
}