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
    pub last_month: Vec<Vec<VoteCount>>,
    pub this_month: Vec<Vec<VoteCount>>,

    pub last_week: Vec<Vec<VoteCount>>,
    pub this_week: Vec<Vec<VoteCount>>,

    pub day_b4_yesterday: Vec<Vec<VoteCount>>,
    pub yesterday: Vec<Vec<VoteCount>>,
    pub today: Vec<Vec<VoteCount>>,
}

impl CategoryPollRankings {
    pub fn new() -> CategoryPollRankings {
        CategoryPollRankings {
            last_month: Vec::new(),
            this_month: Vec::new(),

            last_week: Vec::new(),
            this_week: Vec::new(),

            day_b4_yesterday: Vec::new(),
            yesterday: Vec::new(),
            today: Vec::new(),
        }
    }
}
