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
    CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
    CATEGORY_TODAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex>,
}

impl CategoryIndexMap {
    pub fn new() -> CategoryIndexMap {
        let b: IntBuildHasher = IntBuildHasher::default();

        CategoryIndexMap {
            CATEGORY_LAST_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_THIS_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_LAST_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_THIS_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            CATEGORY_TODAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
        }
    }
}