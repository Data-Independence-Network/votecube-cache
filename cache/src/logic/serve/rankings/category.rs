use std::mem::transmute;
use int_hash::IntHashMap;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::cache::CategoryCacheIndex;
use super::super::super::super::cache::cache::CategoryId;
use super::super::super::super::cache::cache::CategoryPeriodPollRankings;
use super::super::super::super::cache::cache::DayId;
use super::super::super::super::cache::cache::MonthId;
use super::super::super::super::cache::cache::VoteCount;
use super::super::super::super::cache::cache::WeekId;
use super::super::super::super::server::codes;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: usize = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for poll ids & vote counts
        PAGE_SIZE * (8 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for poll ids & vote counts
        PAGE_SIZE * (7 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for poll ids & vote counts
        PAGE_SIZE * (6 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for poll ids & vote counts
        PAGE_SIZE * (5 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        PAGE_SIZE +
        // space for poll ids & vote counts
        PAGE_SIZE * (4 + 3) +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        // space for po ids & vote counts
        PAGE_SIZE * (3 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: usize =
// space for the leading header byte
    1 +
        // space for category cache index (if any
        4 +
        // space for category ids & vote counts
        PAGE_SIZE * (2 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub fn get_todays_category_rankings_by_global_id(
    vcDayId: DayId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.todaysVcDayId,
        vcDayId,
        cache::CATEGORY_TODAYS_INDEX_MAP,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_todays_category_rankings_by_cache_index(
    vcDayId: DayId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.todaysVcDayId,
        vcDayId,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_global_id(
    vcDayId: DayId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdaysVcDayId,
        vcDayId,
        cache::CATEGORY_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vcDayId: DayId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdaysVcDayId,
        vcDayId,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vcDayId: DayId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayB4YesterdaysVcDayId,
        vcDayId,
        cache::CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vcDayId: DayId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.dayB4YesterdaysVcDayId,
        vcDayId,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_global_id(
    vcWeekId: WeekId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisWeeksVcWeekId,
        vcDayId,
        cache::CATEGORY_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vcWeekId: WeekId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisWeeksVcWeekId,
        vcDayId,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_global_id(
    vcWeekId: WeekId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastWeeksVcWeekId,
        vcDayId,
        cache::CATEGORY_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vcWeekId: WeekId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastWeeksVcWeekId,
        vcDayId,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_global_id(
    vcMonthId: MonthId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisMonthsVcMonthId,
        vcDayId,
        cache::CATEGORY_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_cache_index(
    vcMonthId: MonthId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.thisMonthsVcMonthId,
        vcDayId,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_global_id(
    vcMonthId: MonthId,
    blockIndex: u32,
    globalCategoryId: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastMonthsVcMonthId,
        vcDayId,
        cache::CATEGORY_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        globalCategoryId,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_cache_index(
    vcMonthId: MonthId,
    blockIndex: u32,
    categoryCacheIndex: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.lastMonthsVcMonthId,
        vcDayId,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        categoryCacheIndex,
        blockIndex,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

fn get_category_rankings_by_global_id(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    categoryIndexMap: IntHashMap<CategoryId, CategoryCacheIndex>,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    globalCategoryId: CategoryId,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match categoryIndexMap.get(&globalCategoryId) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE;
        }
        Some(categoryCacheIndex) => {
            return get_category_rankings_with_category_cache_index(
                firstRecordIndex, *categoryCacheIndex,
                givenPeriodCategoryPollRankings, maxPollNumberBytes);
        }
    }
}

fn get_category_rankings_by_cache_index(
    currentPeriodId: u32,
    expectedPeriodId: u32,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    categoryCacheIndex: CategoryCacheIndex,
    blockIndex: u32,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    if currentPeriodId != expectedPeriodId {
        return codes::INVALID_PERIOD_ID_RESPONSE;
    }

    let firstRecordIndex = PAGE_SIZE * blockIndex;

    match givenPeriodCategoryPollRankings.voteCountsByCategoryIndex.get(categoryCacheIndex) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE;
        }
        Some(_) => {
            return get_category_rankings(
                firstRecordIndex, categoryCacheIndex,
                givenPeriodCategoryPollRankings, maxPollNumberBytes);
        }
    }
}

#[inline]
fn get_category_rankings_with_category_cache_index(
    firstRecordIndex: usize,
    categoryCacheIndex: CategoryCacheIndex,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForCategory = givenPeriodCategoryPollRankings[categoryCacheIndex];
    let categoryCacheIndexBytes: [u8; 4] = unsafe {
        std::mem::transmute(*categoryCacheIndex);
    };


    match maxPollNumberBytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get3ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000100);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get2ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES);
            response.push(0b00000101);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES);
            response.push(0b00000110);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES);
            response.push(0b00000111);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES);
            response.push(0b00000000);
            response.extend_from_slice(&categoryCacheIndexBytes);

            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get_category_rankings(
    firstRecordIndex: usize,
    categoryCacheIndex: CategoryCacheIndex,
    givenPeriodCategoryPollRankings: Vec<Vec<VoteCount>>,
    maxPollNumberBytes: u8,
) -> Vec<u8> {
    let voteCountsForCategory = givenPeriodCategoryPollRankings[categoryCacheIndex];

    match maxPollNumberBytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES);
            response.push(0b00000011);
            return get3ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES);
            response.push(0b00000100);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES);
            response.push(0b00000010);
            return get2ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES);
            response.push(0b00000101);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES);
            response.push(0b00000110);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES);
            response.push(0b00000111);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES);
            response.push(0b00000000);
            return get4ByteRecentPolls(voteCountsForCategory, firstRecordIndex, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE;
}

#[inline]
fn get8ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get7ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[1..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get6ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[2..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get5ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[3..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[4..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get3ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[5..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}

#[inline]
fn get2ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.pollTypeAndTz);

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.tzAndPeriodPollId);
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&pollIdBytes[6..7]);

                let countBytes: [u8; 4] = unsafe {
                    std::mem::transmute(*voteCount.count);
                };
                if countBytes[0] != 0 {
                    response.extend_from_slice(&countBytes);
                    voteCountsSizes.add4();
                } else if countBytes[1] != 0 {
                    response.extend_from_slice(&countBytes[1..3]);
                    voteCountsSizes.add3();
                } else if countBytes[2] != 0 {
                    response.extend_from_slice(&countBytes[2..3]);
                    voteCountsSizes.add2();
                } else {
                    response.push(countBytes[3]);
                    voteCountsSizes.add1();
                }
            }
        }
    }
    voteCountsSizes.append(response);

    return response;
}