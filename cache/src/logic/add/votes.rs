
use common::model::types::DayId;
use common::model::types::PollId;
use common::model::types::MonthId;
use common::model::types::WeekId;

pub fn add_votes(
    dataStream: &[u8]
) {

}

fn add_day_poll_vote(
    vcDayId: DayId,
    globalPollId: PollId,
    voteCount: u32
) {

}

fn add_week_poll_vote(
    vcWeekId: WeekId,
    globalPollId: PollId,
    voteCount: u32
) {

}

fn add_month_poll_vote(
    vcMonthId: MonthId,
    globalPollId: PollId,
    voteCount: u32
) {

}
