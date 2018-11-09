use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::LocationId;

use super::model::LocationPeriodIds;

/**
 *  Random access Location + Category Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct LocationCategoryIndexMap {
    pub LAST_MONTH: IntHashMap<LocationId, LocationPeriodIds>,
    pub THIS_MONTH: IntHashMap<LocationId, LocationPeriodIds>,
    pub LAST_WEEK: IntHashMap<LocationId, LocationPeriodIds>,
    pub THIS_WEEK: IntHashMap<LocationId, LocationPeriodIds>,
    pub DAY_B4_YESTERDAY: IntHashMap<LocationId, LocationPeriodIds>,
    pub YESTERDAY: IntHashMap<LocationId, LocationPeriodIds>,
    pub TODAY: IntHashMap<LocationId, LocationPeriodIds>,
}

impl LocationCategoryIndexMap {
    pub fn new() -> LocationCategoryIndexMap {
        LocationCategoryIndexMap {
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