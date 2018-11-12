use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::LocationId;

use super::model::LocationPeriodIds;

/**
 *  Random access Location + Label Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct LocationLabelIndexMap {
    pub last_month: IntHashMap<LocationId, LocationPeriodIds>,
    pub this_month: IntHashMap<LocationId, LocationPeriodIds>,
    pub last_week: IntHashMap<LocationId, LocationPeriodIds>,
    pub this_week: IntHashMap<LocationId, LocationPeriodIds>,
    pub day_b4_yesterday: IntHashMap<LocationId, LocationPeriodIds>,
    pub yesterday: IntHashMap<LocationId, LocationPeriodIds>,
    pub today: IntHashMap<LocationId, LocationPeriodIds>,
}

impl LocationLabelIndexMap {
    pub fn new() -> LocationLabelIndexMap {
        LocationLabelIndexMap {
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