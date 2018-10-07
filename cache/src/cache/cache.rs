use int_hash::IntHashMap;

use common::model::consts::NUM_TIMEZONES;

/**
 * Global time period ids across timezones, maintained at the same time as data is moved
 * in timezone chunks between, current and past (and future).
 *
 * Used to verify client requests, to make sure that their requests are still valid.
 */
pub static mut lastMonthIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut thisMonthIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut nextMonthIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut lastWeekIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut thisWeekIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut nextWeekIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut dayB4YesterdayIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut yesterdayIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut tomorrowIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
pub static mut dayAfterTomorrowIds: [u32; NUM_TIMEZONES] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

/**
 *  Maximum number of bytes taken by poll ids of a given current of future cache period.
 */
pub static mut LAST_MONTHS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut THIS_MONTHS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut NEXT_MONTHS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);

pub static mut LAST_WEEKS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut THIS_WEEKS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut NEXT_WEEKS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);

pub static mut DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut YESTERDAYS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut TODAYS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut TOMORROWS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);
pub static mut DAY_AFTER_TOMORROWS_POLL_ID_BYTE_COUNTS: Vec<u8>
= Vec::with_capacity(NUM_TIMEZONES_WITH_GLOBAL_CATEGORY);

/**
 * Ids of currently cached time periods, across all timezones
 */
pub static mut CATEGORY_CACHE_PERIOD_IDS: CachePeriodIds = CachePeriodIds::new();

/**
 * Ids of currently cached time periods, per timezone
 */
pub static mut PER_TIMEZONE__CACHE_PERIOD_IDS: Vec<CachePeriodIds> = Vec::with_capacity(NUM_TIMEZONES);

//pub static mut LOCATION_TIMEZONE_MAP: LsbShiftTree<usize> = LsbShiftTree::new();
//pub static mut LOCATIONS_BY_TIMEZONE: Vec<u32> = Vec::new();

// Keeps track of when a timezone is being modified
pub static mut TIMEZONE_MODIFICATION_FLAGS: Vec<boolean> = Vec::with_capacity(NUM_TIMEZONES);

/**
 *  Future period prepend data structures for adding recent polls.
 *    By:   timezoneId
 *              locationId
 *                  categoryId
 *  Contain only the prepended Poll Ids
 */
pub static mut NEXT_MONTHS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut NEXT_WEEKS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_AFTER_TOMORROWS_POLLS_BY_LOCATION: Vec<IntHashMap<LocationId, LocationPollPrependLists>> = Vec::with_capacity(NUM_TIMEZONES);

/**
 *  Future period prepend data structures for per category access.
 *      By:     categoryId
 *  Contain only the prepended Poll Ids
 */
pub static mut NEXT_MONTHS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = IntHashMap::with_capacity(1000000);
pub static mut NEXT_WEEKS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = IntHashMap::with_capacity(1000000);
pub static mut TOMORROWS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = IntHashMap::with_capacity(1000000);
pub static mut DAY_AFTER_TOMORROWS_POLLS_BY_CATEGORY: IntHashMap<CategoryId, Vec<Vec<PollId>>> = IntHashMap::with_capacity(1000000);

/**
 *  Random access Category and Location Id maps, needed by initial lookup from clients.  The
 *  stored index is then used to access the VoteCount nested arrays.
 */
pub static mut CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);
pub static mut CATEGORY_TODAYS_INDEX_MAP: IntHashMap<CategoryId, CategoryCacheIndex> = IntHashMap::with_capacity(2000);

pub static mut LOCATION_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);

pub static mut LOCATION_CATEGORY_LAST_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_THIS_MONTHS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_LAST_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_THIS_WEEKS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_YESTERDAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);
pub static mut LOCATION_CATEGORY_TODAYS_INDEX_MAP: IntHashMap<LocationId, LocationPeriodIds> = IntHashMap::with_capacity(2000);

/**
 *  The location/based poll rankings nested arrays by:
 *      Timezone Id
 *          Location Id
 *  Internally each LocationPollRanking contains another array by:
 *      Category Id
 *
 *  Location and Location+Category Ids are initially looked up via the Random Access maps.
 *  Subsequently, the client knows the time period specific ids and uses them for direct access.
 */
pub static mut LAST_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTHS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

pub static mut LAST_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEKS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

pub static mut DAY_B4_YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAYS_LOCATION_POLL_RANKINGS: Vec<Vec<LocationPollRankings>> = Vec::with_capacity(NUM_TIMEZONES);

/**
 * Poll rankings by Category.
 * Q: Global category lookups are meant to cross timezone boundaries but how to maintain that?
 *
 * 1)  Maintain only per-location/per-category rankings
 *
 * 2)  Dynamically add and remove polls from category rankings as the go in and out of scope for each
 * day (probably too hard at the moment).
 *
 * 3)  Maintain only previous period rankings (doable now) - Implementing
 *
 * 3a)  Actually, today's category rankings can be made available after UTC-8 (West Coast) passes
 * its poll add deadline (10pm) for the next day.  At that point there are still 9-10 hours left
 * in the day in Japan (depending on daylight savings).
 */
pub static mut LAST_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut LAST_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut THIS_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();

pub static mut DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();
pub static mut TODAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>> = Vec::new();


/**
 * Random access current poll maps, needed for count and sum increments by the voting servers.
 *    Indexed by global PollIds
 *
 *  TODO: many not be needed, assuming that timezone is known at the time of
 *  count and sum increments
 */
//pub static mut TODAY_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
//pub static mut TODAY_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
//pub static mut TODAY_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_WEEK_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_WEEK_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_WEEK_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_MONTH_1_D_POLL_MAP: IntHashMap<u64, OneDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_MONTH_2_D_POLL_MAP: IntHashMap<u64, TwoDPoll> = IntHashMap::with_capacity(2000);
//pub static mut THIS_MONTH_3_D_POLL_MAP: IntHashMap<u64, ThreeDPoll> = IntHashMap::with_capacity(2000);

/**
* Polls HashMap by timezone and global id.
*  The actual poll counts are stored here.  They are accessed by the clients when they need
*  sums and counts for a particular poll.
*/
pub static mut TODAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut TODAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut YESTERDAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut DAY_B4_YESTERDAY_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_WEEK_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_1_D_POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_WEEK_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_1_D__POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut THIS_MONTH_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_1_D__POLLS: Vec<IntHashMap<PollId, OneDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_2_D_POLLS: Vec<IntHashMap<PollId, TwoDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
pub static mut LAST_MONTH_3_D_POLLS: Vec<IntHashMap<PollId, ThreeDPoll>> = Vec::with_capacity(NUM_TIMEZONES);
