use int_hash::IntHashMap;

use common::model::types::CategoryId;
//use common::model::types::DayId;
//use common::model::types::LocationId;
//use common::model::types::MonthId;
use common::model::types::PollId;
//use common::model::types::WeekId;


//use super::super::super::cache::cache::Cache;

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
//pub fn add_polls(
//    raw_data: &[u8],
//    cache: &mut Cache,
//) {
//
//}

//fn add_day_poll(
//    vc_day_id: DayId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_category_ids: Vec<CategoryId>,
//    cache: &mut Cache,
//) {
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

//}


//
//
//fn add_tomorrows_polls(
//    vc_day_id: DayId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_category_ids: Vec<CategoryId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_day_after_tomorrows_polls(
//    vc_day_id: DayId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_category_ids: Vec<CategoryId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_next_weeks_polls(
//    vc_week_id: WeekId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_category_ids: Vec<CategoryId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_next_month_polls(
//    vc_month_id: MonthId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_category_ids: Vec<CategoryId>,
//    cache: &mut Cache,
//) {
//
//}



pub fn add_polls_to_per_category_map(
    poll_map: &mut IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    rehash: &mut bool,
    category_ids: Vec<CategoryId>,
    poll_ids: Vec<Vec<PollId>>,
) {
    let mut missing_category_ids: Vec<CategoryId> = Vec::new();
    let mut missing_poll_ids: Vec<Vec<PollId>> = Vec::new();

    let mut i = 0;
    for poll_ids_to_add in poll_ids {
        let category_id = category_ids[i];
        if poll_map.contains_key(&category_id) {
            insert_polls_ids_to_per_category_map(poll_map, poll_ids_to_add, category_id, false);
        } else {
            missing_category_ids.push(category_id);
            missing_poll_ids.push(poll_ids_to_add);
        }

        i += 1;
    }

    let num_missing_categories = missing_category_ids.len();
    if num_missing_categories == 0 {
        return;
    }

    let spare_poll_map_capacity = poll_map.capacity() - poll_map.len();
    if spare_poll_map_capacity < num_missing_categories {
        *rehash = true;
    }

    let mut i = 0;

    for poll_ids_to_add in missing_poll_ids {
        let category_id = missing_category_ids[i];

        insert_polls_ids_to_per_category_map(poll_map, poll_ids_to_add, category_id, true);

        i += 1;
    }
    *rehash = false;
}

fn insert_polls_ids_to_per_category_map(
    poll_map: &mut IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    poll_ids_to_add: Vec<PollId>,
    category_id: CategoryId,
    new_vec: bool,
) {
    if new_vec {
        poll_map.insert(category_id, Vec::new());
    }

    let poll_id_frames = poll_map.get_mut(&category_id).unwrap();

    let num_polls = poll_ids_to_add.len() * 1;
    if num_polls <= 1024 {
        poll_id_frames.push(poll_ids_to_add);
        return;
    }

    let mut num_frames = num_polls / 1024;
    if num_polls % 1024 != 0 {
        num_frames += 1;
    }
    let last_frame_index = num_frames - 1;
    for frame_index in 0..last_frame_index {
        let frame = poll_ids_to_add[frame_index * 1024..frame_index + 1 * 1024]
            .iter().cloned().collect();
        poll_id_frames.push(frame);
    }
    let last_frame = poll_ids_to_add[last_frame_index * 1024..poll_ids_to_add.len()]
        .iter().cloned().collect();
    poll_id_frames.push(last_frame);
}
