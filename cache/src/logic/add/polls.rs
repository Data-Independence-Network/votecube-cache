use int_hash::IntHashMap;

use common::model::types::DayId;
use common::model::types::LocationId;
use common::model::types::PollId;
use common::model::types::WeekId;

use super::super::super::cache::cache;
use super::super::super::cache::cache::CategoryId;
use super::super::super::cache::cache::DayId;
use super::super::super::cache::cache::LocationId;
use super::super::super::cache::cache::LocationPollPrependLists;
use super::super::super::cache::cache::MonthId;
use super::super::super::cache::cache::PollId;
use super::super::super::cache::cache::TimezoneId;
use super::super::super::cache::cache::WeekId;



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
    rawData: &[u8]
) {

}

fn add_day_poll(
    vcDayId: DayId,
    globalPollId: PollId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
) {

}

fn add_week_poll(
    vcWeekId: WeekId,
    globalPollId: PollId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
) {

}

fn add_month_poll(
    vcMonthId: MonthId,
    globalPollId: PollId,
    globalLocationId: LocationId,
    globalCategoryIds: Vec<u64>,
) {

}
