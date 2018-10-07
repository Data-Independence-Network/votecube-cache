use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::LocationId;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

/**
 *
 *  Entering polls early - not needed for Minimum Viable Product.
 *
 *  Batching poll entries to reduce rehashing:
 *
 *    what are the chances of polls coming in for multiple
 *    categories at the same time - very high
 *
 *    what are the chances of polls coming in for the same
 *    category in the same location at the same time
 *          - not as high
 */
pub fn add_polls(
    raw_data: &[u8]
) {

}

fn add_day_poll(
    vc_day_id: DayId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
) {

}

fn add_week_poll(
    vc_week_id: WeekId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
) {

}

fn add_month_poll(
    vc_month_id: MonthId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
) {

}
