use super::model::VoteCount;

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
pub struct CategoryPollRankings {
    LAST_MONTH: Vec<Vec<VoteCount>>,
    THIS_MONTH: Vec<Vec<VoteCount>>,

    LAST_WEEK: Vec<Vec<VoteCount>>,
    THIS_WEEK: Vec<Vec<VoteCount>>,

    DAY_B4_YESTERDAY: Vec<Vec<VoteCount>>,
    YESTERDAY: Vec<Vec<VoteCount>>,
    TODAY: Vec<Vec<VoteCount>>,
}

impl CategoryPollRankings {
    pub fn new() -> CategoryPollRankings {
        CategoryPollRankings {
            LAST_MONTH: Vec::new(),
            THIS_MONTH: Vec::new(),

            LAST_WEEK: Vec::new(),
            THIS_WEEK: Vec::new(),

            DAY_B4_YESTERDAY: Vec::new(),
            YESTERDAY: Vec::new(),
            TODAY: Vec::new(),
        }
    }
}
