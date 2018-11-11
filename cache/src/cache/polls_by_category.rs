use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::collections::hash_map::RandomState;

use evmap;
use evmap::ReadHandle;
use evmap::WriteHandle;

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
    ExistingMap,
}

/**
 *  Future period prepend data structures for per category access.
 *      By:     categoryId
 *  Contain only the prepended Poll Ids in an Vector of equal size blocks (1024 each), hence
 *  a Vec<Vec<PollId>>
 *
    let x: [Box<[u8]>; 3] = [
    Box::new([1, 2, 3]),
    Box::new([4]),
    Box::new([5, 6])
];
 *  Vec appears to be "thread safe enough", since during reallocation the ptr gets updated
 *  before the capacity is reset.  So it's already a simple pointer swap and given that there
 *  is only one thread doing the updates the cap reset non-automicity is of no consequence.
 *
 *  The next issue realloc of array which is:
 *         Allocation of new memory
 *         Copy of memory
 *         Freeing of old memory
 *
 *  This does require more memory writes and will likely cause fragmentation of memory.  A
 *  trade-off would be to have an extra dimension with arrays 2 dimensional array of vectors
 *  (because vector is the final interface anyway).  The array dimensions being 1024*1024
 *  this would require 8KB per category.  Alternatively ALL vectors could be stored in a
 *  flat array, per period.  So given that no more than 1B polls are created per period
 *  we could have 8MB + 5 periods - only 40MB worth of pointers with a flat structure.
 *  Though that does not work because each category get's its own Vector frames and these
 *  frames are not shared between categories.  But given that the data itself would not be
 *  accessed directly, we can afford a 2d array there 1MB*1KB*frame, thus allowing for
 *  a trillion of frames across all categories, per time period, at the same flat cost
 *  of 40MB worth of pointers.
 *  Alternatively the global (per period) storage array can store the per-category Vec<Vec>s.
 *  This would allow for the flexibility of Vec<Vec> with the global storage and &Vec<Vec>s
 *  in the evmaps of categories (which solves the copying problem of evmap) - current solution.
 *  We can even hedge the bet by instead of using an array using a Vec with initial capacity
 *  of 1M.  Then if it does grow it's not as much of an issue and fragmentation is also not
 *  that bad, given the 8MB worth of memory freed in a single block (and it's unlikely one-time
 *  occurrence).
 *  We do have to maintain per time period underlying data Vecs because their lifecycles are
 *  maintained at difference cadence.

pub struct PollsByCategory {
    pub next_month_data: Vec<Vec<Vec<PollId>>>,
    pub next_month_r: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,
    pub next_month_w: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,

    pub next_week_data: Vec<Vec<Vec<PollId>>>,
    pub next_week_r: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,
    pub next_week_w: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,

    pub tomorrow_data: Vec<Vec<Vec<PollId>>>,
    pub tomorrow_r: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,
    pub tomorrow_w: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,

    pub day_after_tomorrow_data: Vec<Vec<Vec<PollId>>>,
    pub day_after_tomorrow_r: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,
    pub day_after_tomorrow_w: ReadHandle<CategoryId, &'static Vec<Vec<PollId>>, (), RandomState>,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {

        let (next_month_r, next_month_w) = evmap::Options::default()
            .with_capacity(512000)
            .with_hasher(IntBuildHasher::default())
            .construct();

        PollsByCategory {
            next_month_data: Vec::with_capacity(1024000),
            next_month_r,
            next_month_w,
            next_week: Vec::with_capacity(1024000),
            next_week_r,
            next_week_w,
            tomorrow: Vec::with_capacity(1024000),
            tomorrow_r,
            tomorrow_w,
            tomorrow_r: Vec::with_capacity(1024000),
            tomorrow_w: Vec::with_capacity(1024000),
            day_after_tomorrow: Vec::with_capacity(1024000),
            day_after_tomorrow_r,
            day_after_tomorrow_w,
        }
    }

 *  Ultimately, since the maps get rebuilt periodically we can just build in spare capacity
 *  And resize them 2x the size of the structure for the previous period of that type
 *  And in the unlikely scenario of a re-hash we can check for that during access and
 *  just return a "temporarily unavailable" error


    ReadHandle<K, V, (), RandomState>,
    WriteHandle<K, V, (), RandomState>,
 */
pub struct PollsByCategory {
    pub next_month: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub next_month_rehash: bool,
    pub next_week: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub next_week_rehash: bool,
    pub tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub tomorrow_rehash: bool,
    pub day_after_tomorrow: IntHashMap<CategoryId, Vec<Vec<PollId>>>,
    pub day_after_tomorrow_rehash: bool,
}

impl PollsByCategory {
    pub fn new() -> PollsByCategory {
        evmap::new();
        PollsByCategory {
            next_month: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            next_month_rehash: false,
            next_week: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            next_week_rehash: false,
            tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            tomorrow_rehash: false,
            day_after_tomorrow: HashMap::with_capacity_and_hasher(1000000, IntBuildHasher::default()),
            day_after_tomorrow_rehash: false,
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

            for (category_id, poll_ids) in poll_map.iter_mut() {
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
                poll_ids.push(poll_ids_to_add);
            } else {
                let mut num_frames = num_polls / 1024;
                if num_polls % 1024 != 0 {
                    num_frames += 1;
                }
                let last_frame_index = num_frames - 1;
                for frame_index in 0..last_frame_index {
                    let frame = poll_ids_to_add[frame_index * 1024..frame_index + 1 * 1024]
                        .iter().cloned().collect();
                    poll_ids.push(frame);
                }
                let last_frame = poll_ids_to_add[last_frame_index * 1024..poll_ids_to_add.len()]
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
