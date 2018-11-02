use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::LocationId;
use common::model::types::PollId;

use super::model::LocationPeriodIds;

/**
 *  Random access Location Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct LocationIndexMap {
    LOCATION_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
}

impl LocationIndexMap {
    pub fn new() -> LocationIndexMap {
        let b: IntBuildHasher = IntBuildHasher::default();

        LocationIndexMap {
            LOCATION_LAST_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_THIS_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_LAST_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_THIS_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_TODAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
        }
    }
}