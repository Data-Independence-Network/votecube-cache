use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::LabelCacheIndex;
use common::model::types::LabelId;

/**
 *  Random access Label Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct LabelIndexMap {
    pub last_month: IntHashMap<LabelId, LabelCacheIndex>,
    pub this_month: IntHashMap<LabelId, LabelCacheIndex>,
    pub last_week: IntHashMap<LabelId, LabelCacheIndex>,
    pub this_week: IntHashMap<LabelId, LabelCacheIndex>,
    pub day_b4_yesterday: IntHashMap<LabelId, LabelCacheIndex>,
    pub yesterday: IntHashMap<LabelId, LabelCacheIndex>,
    pub today: IntHashMap<LabelId, LabelCacheIndex>,
}

impl LabelIndexMap {
    pub fn new() -> LabelIndexMap {
        LabelIndexMap {
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