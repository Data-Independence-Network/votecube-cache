use std::collections::HashMap;
use std::collections::hash_map::Entry;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::PollId;
use common::model::types::MonthId;
use common::model::types::WeekId;

pub struct DayPollAddition {
    pub vc_day_id: DayId,
    pub global_poll_id: PollId,
    pub global_category_ids: Vec<CategoryId>,
}

pub struct CategoryPollAddition {
    pub category_id: CategoryId,
    pub poll_ids: Vec<PollId>,
}

pub enum PollIdAdditionResult {
    NewMap(IntHashMap<CategoryId, Vec<Vec<PollId>>>),
    ExistingMap
}

/**
 *  Future period prepend data structures for per category access.
 *      By:     categoryId
 *  Contain only the prepended Poll Ids in an Vector of equal size blocks (1024 each), hence
 *  a Vec<Vec<PollId>>
 */
pub struct PollsByCategory {
    pub next_month: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub next_week: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub day_after_tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {
        PollsByCategory {
            next_month: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            next_week: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            day_after_tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
        }
    }

    fn add_tomorrow_poll(
        &mut self,
        mut poll_map: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
        mut category_poll_additions: Vec<CategoryPollAddition>,
    ) -> PollIdAdditionResult {
        let mut result = PollIdAdditionResult::ExistingMap;

        let mut missing_category_additions: Vec<&CategoryPollAddition> = Vec::new();

        for category_poll_addition in &category_poll_additions {
            if poll_map.contains_key(&category_poll_addition.category_id) {} else {
                missing_category_additions.push(category_poll_addition);
            }
        }

        let num_missing_categories = missing_category_additions.len();
        if num_missing_categories == 0 {
            return result;
        }

        let mut map_grew = false;

        let spare_poll_map_capacity = poll_map.capacity() - poll_map.len();
        if spare_poll_map_capacity < num_missing_categories {
            let mut new_poll_map_capacity = poll_map.capacity() + num_missing_categories;
            new_poll_map_capacity += new_poll_map_capacity / 2;

            let mut new_poll_map: IntHashMap<CategoryId, Vec<Vec<PollId>>>
            = HashMap::with_capacity_and_hasher(new_poll_map_capacity, IntBuildHasher::default());

            for (category_id, poll_ids) in poll_map.iter() {
                new_poll_map.insert(*category_id, *poll_ids);
            }

            poll_map = new_poll_map;
            map_grew = true;
        }

        for category_poll_addition in &missing_category_additions {
            let mut poll_ids = Vec::new();

            let poll_ids_to_add = category_poll_addition.poll_ids;
            let num_polls = poll_ids_to_add.len();
            if num_polls <= 1024 {
                poll_ids.push(poll_ids_to_add );
            } else {
                let mut num_frames = num_polls / 1024;
                if num_polls % 1024 != 0 {
                    num_frames += 1;
                }
                let last_frame_index = num_frames - 1;
                for frame_index in 0..last_frame_index {
                    let frame = poll_ids_to_add [frame_index * 1024..frame_index + 1 * 1024]
                        .iter().cloned().collect();
                    poll_ids.push(frame);
                }
                let last_frame = poll_ids_to_add [last_frame_index * 1024..poll_ids_to_add.len()]
                    .iter().cloned().collect();
                poll_ids.push(last_frame);
            }
            poll_map.insert(category_poll_addition.category_id, poll_ids);
        }

        if map_grew {
            return PollIdAdditionResult::NewMap(poll_map);
        }

        return result;
    }

}
