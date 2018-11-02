use std::collections::HashMap;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::LocationId;
use common::model::types::PollId;

use super::model::LocationPeriodIds;

/**
 *  Random access Location + Category Id map, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub struct LocationCategoryIndexMap {
    LOCATION_CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
    LOCATION_CATEGORY_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds>,
}

impl LocationCategoryIndexMap {
    pub fn new() -> LocationCategoryIndexMap {
        let b: IntBuildHasher = IntBuildHasher::default();

        LocationCategoryIndexMap {
            LOCATION_CATEGORY_LAST_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_THIS_MONTHS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_LAST_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_THIS_WEEKS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_YESTERDAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
            LOCATION_CATEGORY_TODAYS_INDEX_MAP: HashMap::with_capacity_and_hasher(2000, b),
        }
    }
}