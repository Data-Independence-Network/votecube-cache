use std::mem::transmute;
use int_hash::IntHashMap;

use common::model::consts;
use common::model::types::CategoryId;
use common::model::types::CategoryCacheIndex;
use common::model::types::DayId;
use common::model::types::MonthId;
use common::model::types::WeekId;

use super::super::super::super::cache::cache;
use super::super::super::super::cache::model::VoteCount;
use super::super::super::super::cache::model::CategoryPeriodPollRankings;
use super::super::super::super::data::byte_counts::ByteCounts;
use super::super::super::super::server::codes;

// NOTE: max page size must fin into u16
const PAGE_SIZE: u64 = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: u64 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: u64 =
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
    vc_day_id: DayId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.todays_vc_day_id,
        vc_day_id,
        cache::CATEGORY_TODAYS_INDEX_MAP,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_todays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.todays_vc_day_id,
        vc_day_id,
        cache::TODAYS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::TODAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdays_vc_day_id,
        vc_day_id,
        cache::CATEGORY_YESTERDAYS_INDEX_MAP,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.yesterdays_vc_day_id,
        vc_day_id,
        cache::YESTERDAYS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        cache::CATEGORY_DAY_B4_YESTERDAYS_INDEX_MAP,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        cache::DAY_B4_YESTERDAYS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::DAY_B4_YESTERDAYS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.this_weeks_vc_week_id,
        vc_week_id,
        cache::CATEGORY_THIS_WEEKS_INDEX_MAP,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.this_weeks_vc_week_id,
        vc_week_id,
        cache::THIS_WEEKS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::THIS_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.last_weeks_vc_week_id,
        vc_week_id,
        cache::CATEGORY_LAST_WEEKS_INDEX_MAP,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.last_weeks_vc_week_id,
        vc_week_id,
        cache::LAST_WEEKS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::LAST_WEEKS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.this_months_vc_month_id,
        vc_month_id,
        cache::CATEGORY_THIS_MONTHS_INDEX_MAP,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_this_months_category_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.this_months_vc_month_id,
        vc_month_id,
        cache::THIS_MONTHS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::THIS_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u64,
    global_category_id: CategoryId,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache::CATEGORY_CACHE_PERIOD_IDS.last_months_vc_month_id,
        vc_month_id,
        cache::CATEGORY_LAST_MONTHS_INDEX_MAP,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        global_category_id,
        block_index,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

pub fn get_last_months_category_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u64,
    category_cache_index: CategoryCacheIndex,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache::CATEGORY_CACHE_PERIOD_IDS.last_months_vc_month_id,
        vc_month_id,
        cache::LAST_MONTHS_CATEGORY_POLL_RANKINGS,
        category_cache_index,
        block_index,
        cache::LAST_MONTHS_POLL_ID_BYTE_COUNTS[38],
    );
}

fn get_category_rankings_by_global_id(
    current_period_id: u32,
    expected_period_id: u32,
    category_index_map: IntHashMap<CategoryId, CategoryCacheIndex>,
    given_period_category_poll_rankings: Vec<Vec<VoteCount>>,
    global_category_id: CategoryId,
    block_index: u64,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let first_record_index = PAGE_SIZE * block_index;

    match category_index_map.get(&global_category_id) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE.to_vec();
        }
        Some(categoryCacheIndex) => {
            return get_category_rankings_with_category_cache_index(
                first_record_index, *categoryCacheIndex,
                given_period_category_poll_rankings, max_poll_number_bytes);
        }
    }
}

fn get_category_rankings_by_cache_index(
    current_period_id: u32,
    expected_period_id: u32,
    vote_counts_by_category_index: Vec<Vec<VoteCount>>,
    category_cache_index: CategoryCacheIndex,
    block_index: u64,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let first_record_index = PAGE_SIZE * block_index;

    match vote_counts_by_category_index.get(category_cache_index as usize) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE.to_vec();
        }
        Some(_) => {
            return get_category_rankings(
                first_record_index, category_cache_index,
                vote_counts_by_category_index, max_poll_number_bytes);
        }
    }
}

#[inline]
fn get_category_rankings_with_category_cache_index(
    first_record_index: u64,
    category_cache_index: CategoryCacheIndex,
    vote_counts_by_category_index: Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_category = vote_counts_by_category_index[category_cache_index as usize];
    let category_cache_index_bytes: [u8; 4] = unsafe {
        std::mem::transmute(category_cache_index)
    };


    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);
            response.push(0b00000011);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_3_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);
            response.push(0b00000100);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_4_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);
            response.push(0b00000101);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_5_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);
            response.push(0b00000110);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_6_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);
            response.push(0b00000111);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_7_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);
            response.push(0b00000000);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_8_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);
            response.push(0b00000010);
            response.extend_from_slice(&category_cache_index_bytes);

            return get_2_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_category_rankings(
    first_record_index: u64,
    category_cache_index: CategoryCacheIndex,
    given_period_category_poll_rankings: Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_category = given_period_category_poll_rankings[category_cache_index as usize];

    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);
            response.push(0b00000011);
            return get_3_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);
            response.push(0b00000100);
            return get_4_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);
            response.push(0b00000101);
            return get_5_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);
            response.push(0b00000110);
            return get_6_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);
            response.push(0b00000111);
            return get_7_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);
            response.push(0b00000000);
            return get_8_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);
            response.push(0b00000010);
            return get_2_byte_recent_polls(vote_counts_for_category, first_record_index as usize, response);
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_8_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_7_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[1..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_6_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[2..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_5_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[3..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_4_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[4..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_3_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[5..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}

#[inline]
fn get_2_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[6..7]);

                let count_bytes: [u8; 4] = unsafe {
                    std::mem::transmute(voteCount.count)
                };
                if count_bytes[0] != 0 {
                    response.extend_from_slice(&count_bytes);
                    vote_counts_sizes.add4();
                } else if count_bytes[1] != 0 {
                    response.extend_from_slice(&count_bytes[1..3]);
                    vote_counts_sizes.add3();
                } else if count_bytes[2] != 0 {
                    response.extend_from_slice(&count_bytes[2..3]);
                    vote_counts_sizes.add2();
                } else {
                    response.push(count_bytes[3]);
                    vote_counts_sizes.add1();
                }
            }
        }
    }
    vote_counts_sizes.append(response);

    return response;
}