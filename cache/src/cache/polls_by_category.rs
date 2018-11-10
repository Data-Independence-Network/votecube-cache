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
        category_poll_additions: Vec<CategoryPollAddition>
    ) {
        let mut poll_map: &mut IntHashMap<CategoryId, Vec<Vec<PollId>>> = &mut sef.tomorrow;

        let mut missing_category_additions: Vec<CategoryPollAddition> = Vec::new();

        for category_poll_addition in &category_poll_additions {
            if poll_map.contains_key(&category_poll_addition.category_id) {

            } else {
                missing_category_additions.push(*category_poll_addition);
            }
        }

        let num_missing_categories = missing_category_additions.len();
        if num_missing_categories == 0 {
            return;
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

            poll_map = &mut new_poll_map;
            map_grew = true;
        }

        for category_poll_addition in &missing_category_additions {
            // TODO: work here next
            let mut first_polls_block = Vec::new();
            first_polls_block.push(global_poll_id);
        }

    }

    fn add_day_poll(
        &mut self,
        day_poll_additions: Vec<DayPollAddition>,
    ) {
        // TODO: Figure out if the poll is for tomorrow or day after tomorrow
        let for_tomorrow = false;

        let mut poll_map: &mut IntHashMap<CategoryId, Vec<Vec<PollId>>> = if for_tomorrow {
            &mut self.tomorrow
        } else {
            &mut self.day_after_tomorrow
        };

        for category_id in &global_category_ids {


            let poll_ids = match poll_map.entry(*category_id) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(Vec::new())
            };

            let num_frames = poll_ids.len();

            if num_frames == 0 {
                let mut first_polls_block = Vec::new();
                first_polls_block.push(global_poll_id);
                poll_ids.push(first_polls_block);
            } else {
                let last_polls_block: &mut Vec<PollId> = poll_ids.get_mut(num_frames - 1).unwrap();

                if last_polls_block.len() == 1024 {

                }

                last_polls_block.push(global_poll_id);
            }
        }
    }
}