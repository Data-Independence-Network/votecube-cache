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
pub struct CategoriesPollRankings {
    LAST_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,
    THIS_MONTHS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,

    LAST_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,
    THIS_WEEKS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,

    DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,
    YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,
    TODAYS_CATEGORY_POLL_RANKINGS: Vec<Vec<VoteCount>>,
}

impl CategoriesPollRankings {
    pub fn new() -> CategoryPollRankings {
        CategoryPollRankings {
            LAST_MONTHS_CATEGORY_POLL_RANKINGS: Vec::new(),
            THIS_MONTHS_CATEGORY_POLL_RANKINGS: Vec::new(),

            LAST_WEEKS_CATEGORY_POLL_RANKINGS: Vec::new(),
            THIS_WEEKS_CATEGORY_POLL_RANKINGS: Vec::new(),

            DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec::new(),
            YESTERDAYS_CATEGORY_POLL_RANKINGS: Vec::new(),
            TODAYS_CATEGORY_POLL_RANKINGS: Vec::new(),
        }
    }
}
