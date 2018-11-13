use std::time::Duration;
use std::thread;

use int_hash::IntHashMap;

use common::model::types::LabelId;
//use common::model::types::DayId;
//use common::model::types::LocationId;
//use common::model::types::MonthId;
use common::model::types::PollId;
//use common::model::types::WeekId;

/**
    The timeout needed for any existing requests to finish
    before rehashing starts.  Assuming very fast requests.
 */
const NUM_MILLIS_TO_SLEEP_BEFORE_HASHMAP_REHASH: u64 = 10;

//use super::super::super::cache::cache::Cache;

/*
 *
 *  Entering polls early - not needed for Minimum Viable Product.
 *
 *  Batching poll entries to reduce rehashing:
 *
 *    what are the chances of polls coming in for multiple
 *    labels at the same time - very high
 *
 *    what are the chances of polls coming in for the same
 *    label in the same location at the same time
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
//    global_label_ids: Vec<LabelId>,
//    cache: &mut Cache,
//) {
    // TODO: Figure out if the poll is for tomorrow or day after tomorrow

//    let for_tomorrow = false;
//
//    let poll_map = if for_tomorrow {
//        &mut cache.polls_by_label.tomorrow
//    } else {
//        &mut cache.polls_by_label.day_after_tomorrow
//    };
//
//    for label_id in &global_label_ids {
//        let label_poll_ids = match poll_map.get(label_id) {
//            Some(poll_ids) => {
//                poll_ids
//            },
//            None => {
//                let mut poll_ids = Vec::new();
//                let first_polls_block = Vec::new();
//                poll_ids.push(first_polls_block);
//                poll_map.insert(*label_id, poll_ids);
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
//    global_label_ids: Vec<LabelId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_day_after_tomorrows_polls(
//    vc_day_id: DayId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_label_ids: Vec<LabelId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_next_weeks_polls(
//    vc_week_id: WeekId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_label_ids: Vec<LabelId>,
//    cache: &mut Cache,
//) {
//
//}
//
//fn add_next_month_polls(
//    vc_month_id: MonthId,
//    global_poll_id: PollId,
//    global_location_id: LocationId,
//    global_label_ids: Vec<LabelId>,
//    cache: &mut Cache,
//) {
//
//}



pub fn add_polls_to_per_label_map(
    poll_map: &mut IntHashMap<LabelId, Vec<Vec<PollId>>>,
    rehash: &mut bool,
    // Vec of label ids with PollIds to be added
    label_ids: Vec<LabelId>,
    // Vec of Vec<PollId>s, in the same order as the Vec<LabelId> above.  Each nested Vec
    // contains the PollIds for that given LabelId
    poll_ids: Vec<Vec<PollId>>,
) {
    let mut missing_label_ids: Vec<LabelId> = Vec::new();
    let mut missing_poll_ids: Vec<Vec<PollId>> = Vec::new();

    let mut i = 0;
    for poll_ids_to_add in poll_ids {
        let label_id = label_ids[i];
        if poll_map.contains_key(&label_id) {
            insert_polls_ids_to_per_label_map(poll_map, poll_ids_to_add, label_id, false);
        } else {
            missing_label_ids.push(label_id);
            missing_poll_ids.push(poll_ids_to_add);
        }

        i += 1;
    }

    let num_missing_labels = missing_label_ids.len();
    if num_missing_labels == 0 {
        return;
    }

    let spare_poll_map_capacity = poll_map.capacity() - poll_map.len();
    if spare_poll_map_capacity < num_missing_labels {
        std::thread::sleep(Duration::from_millis(NUM_MILLIS_TO_SLEEP_BEFORE_HASHMAP_REHASH));
        *rehash = true;
    }

    let mut i = 0;

    for poll_ids_to_add in missing_poll_ids {
        let label_id = missing_label_ids[i];

        insert_polls_ids_to_per_label_map(poll_map, poll_ids_to_add, label_id, true);

        i += 1;
    }
    *rehash = false;
}

fn insert_polls_ids_to_per_label_map(
    poll_map: &mut IntHashMap<LabelId, Vec<Vec<PollId>>>,
    poll_ids_to_add: Vec<PollId>,
    label_id: LabelId,
    new_vec: bool,
) {
    if new_vec {
        poll_map.insert(label_id, Vec::new());
    }

    let poll_id_frames = poll_map.get_mut(&label_id).unwrap();

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
