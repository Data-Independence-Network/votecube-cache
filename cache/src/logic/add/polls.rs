use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::LocationId;
use common::model::types::MonthId;
use common::model::types::PollId;
use common::model::types::WeekId;

use super::super::super::cache::cache::Cache;

/*
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
    raw_data: &[u8],
    cache: &mut Cache,
) {

}

fn add_day_poll(
    vc_day_id: DayId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
    cache: &mut Cache,
) {
    // TODO: Figure out if the poll is for tomorrow or day after tomorrow

//    let for_tomorrow = false;
//
//    let poll_map = if for_tomorrow {
//        &mut cache.polls_by_category.tomorrow
//    } else {
//        &mut cache.polls_by_category.day_after_tomorrow
//    };
//
//    for category_id in &global_category_ids {
//        let category_poll_ids = match poll_map.get(category_id) {
//            Some(poll_ids) => {
//                poll_ids
//            },
//            None => {
//                let mut poll_ids = Vec::new();
//                let first_polls_block = Vec::new();
//                poll_ids.push(first_polls_block);
//                poll_map.insert(*category_id, poll_ids);
//
//                &poll_ids
//            }
//        };
//    }

}

fn add_week_poll(
    vc_week_id: WeekId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
    cache: &mut Cache,
) {

}

fn add_month_poll(
    vc_month_id: MonthId,
    global_poll_id: PollId,
    global_location_id: LocationId,
    global_category_ids: Vec<CategoryId>,
    cache: &mut Cache,
) {

}
