use std::collections::HashMap;
use std::collections::hash_map::Entry;

use int_hash::IntBuildHasher;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::DayId;
use common::model::types::PollId;
use common::model::types::MonthId;
use common::model::types::WeekId;

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

    fn add_day_poll(
        &mut self,
        vc_day_id: DayId,
        global_poll_id: PollId,
        global_category_ids: Vec<CategoryId>,
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