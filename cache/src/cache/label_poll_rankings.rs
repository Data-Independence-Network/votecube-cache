use super::model::VoteCount;

/**
 * Poll rankings by:
 *      Label Cache Id
 *
 * Q: Global label lookups are meant to cross timezone boundaries but how to maintain that?
 *
 * 1)  Maintain only per-location/per-label rankings
 *
 * 2)  Dynamically add and remove polls from label rankings as the go in and out of scope for each
 * day (probably too hard at the moment).
 *
 * 3)  Maintain only previous period rankings (doable now) - Implementing
 *
 * 3a)  Actually, today's label rankings can be made available after UTC-8 (West Coast) passes
 * its poll add deadline (9pm) for the next day.  At that point there are still 10-11 hours left
 * in the next day in Japan (depending on daylight savings).
 */
pub struct LabelPollRankings {
    pub last_month: Vec<Vec<VoteCount>>,
    pub this_month: Vec<Vec<VoteCount>>,

    pub last_week: Vec<Vec<VoteCount>>,
    pub this_week: Vec<Vec<VoteCount>>,

    pub day_b4_yesterday: Vec<Vec<VoteCount>>,
    pub yesterday: Vec<Vec<VoteCount>>,
    pub today: Vec<Vec<VoteCount>>,
}

impl LabelPollRankings {
    pub fn new() -> LabelPollRankings {
        LabelPollRankings {
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
