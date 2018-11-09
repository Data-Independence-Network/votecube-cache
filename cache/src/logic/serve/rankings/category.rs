use std::mem::transmute;
use int_hash::IntHashMap;

use common::model::types::CategoryId;
use common::model::types::CategoryCacheIndex;
use common::model::types::DayId;
use common::model::types::MonthId;
use common::model::types::WeekId;

use super::super::super::super::cache::cache::Cache;
use super::super::super::super::cache::model::VoteCount;
use super::super::super::super::data::byte_counts::ByteCounts;
use super::super::super::super::server::codes;

// NOTE: max page size must fin into u16
const PAGE_SIZE: u32 = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: u32 =
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

const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: u32 =
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
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.todays_vc_day_id,
        vc_day_id,
        &cache.category_index_map.TODAY,
        &cache.category_poll_rankings.TODAY,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.TODAY[38],
    );
}

pub fn get_todays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.todays_vc_day_id,
        vc_day_id,
        &cache.category_poll_rankings.TODAY,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.TODAY[38],
    );
}

pub fn get_yesterdays_category_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.yesterdays_vc_day_id,
        vc_day_id,
        &cache.category_index_map.YESTERDAY,
        &cache.category_poll_rankings.YESTERDAY,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.YESTERDAY[38],
    );
}

pub fn get_yesterdays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.yesterdays_vc_day_id,
        vc_day_id,
        &cache.category_poll_rankings.YESTERDAY,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.YESTERDAY[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        &cache.category_index_map.DAY_B4_YESTERDAY,
        &cache.category_poll_rankings.DAY_B4_YESTERDAY,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.DAY_B4_YESTERDAY[38],
    );
}

pub fn get_day_b4_yesterdays_category_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.day_b4_yesterdays_vc_day_id,
        vc_day_id,
        &cache.category_poll_rankings.DAY_B4_YESTERDAY,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.DAY_B4_YESTERDAY[38],
    );
}

pub fn get_this_weeks_category_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.this_weeks_vc_week_id,
        vc_week_id,
        &cache.category_index_map.THIS_WEEK,
        &cache.category_poll_rankings.THIS_WEEK,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.THIS_WEEK[38],
    );
}

pub fn get_this_weeks_category_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.this_weeks_vc_week_id,
        vc_week_id,
        &cache.category_poll_rankings.THIS_WEEK,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.THIS_WEEK[38],
    );
}

pub fn get_last_weeks_category_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.last_weeks_vc_week_id,
        vc_week_id,
        &cache.category_index_map.LAST_WEEK,
        &cache.category_poll_rankings.LAST_WEEK,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.LAST_WEEK[38],
    );
}

pub fn get_last_weeks_category_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.last_weeks_vc_week_id,
        vc_week_id,
        &cache.category_poll_rankings.LAST_WEEK,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.LAST_WEEK[38],
    );
}

pub fn get_this_months_category_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.this_months_vc_month_id,
        vc_month_id,
        &cache.category_index_map.THIS_MONTH,
        &cache.category_poll_rankings.THIS_MONTH,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.THIS_MONTH[38],
    );
}

pub fn get_this_months_category_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.this_months_vc_month_id,
        vc_month_id,
        &cache.category_poll_rankings.THIS_MONTH,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.THIS_MONTH[38],
    );
}

pub fn get_last_months_category_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u32,
    global_category_id: CategoryId,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_global_id(
        cache.category_cache_period_ids.last_months_vc_month_id,
        vc_month_id,
        &cache.category_index_map.LAST_MONTH,
        &cache.category_poll_rankings.LAST_MONTH,
        global_category_id,
        block_index,
        cache.poll_id_byte_counts.LAST_MONTH[38],
    );
}

pub fn get_last_months_category_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u32,
    category_cache_index: CategoryCacheIndex,
    cache: &Cache,
) -> Vec<u8> {
    return get_category_rankings_by_cache_index(
        cache.category_cache_period_ids.last_months_vc_month_id,
        vc_month_id,
        &cache.category_poll_rankings.LAST_MONTH,
        category_cache_index,
        block_index,
        cache.poll_id_byte_counts.LAST_MONTH[38],
    );
}

fn get_category_rankings_by_global_id(
    current_period_id: u32,
    expected_period_id: u32,
    category_index_map: &IntHashMap<CategoryId, CategoryCacheIndex>,
    given_period_category_poll_rankings: &Vec<Vec<VoteCount>>,
    global_category_id: CategoryId,
    block_index: u32,
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
    vote_counts_by_category_index: &Vec<Vec<VoteCount>>,
    category_cache_index: CategoryCacheIndex,
    block_index: u32,
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
    first_record_index: u32,
    category_cache_index: CategoryCacheIndex,
    vote_counts_by_category_index: &Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_category: &Vec<VoteCount> = vote_counts_by_category_index.get(category_cache_index as usize).unwrap();
    let category_cache_index_bytes: [u8; 4] = unsafe {
        transmute(category_cache_index)
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
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_category_rankings(
    first_record_index: u32,
    category_cache_index: CategoryCacheIndex,
    given_period_category_poll_rankings: &Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_category: &Vec<VoteCount> = given_period_category_poll_rankings.get(category_cache_index as usize).unwrap();

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
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_8_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_7_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[1..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_6_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[2..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_5_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[3..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_4_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[4..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_3_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[5..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}

#[inline]
fn get_2_byte_recent_polls(
    poll_rankings: &Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);

    for _ in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                response.push(voteCount.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(voteCount.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[6..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(voteCount.count)
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
    vote_counts_sizes.append(&mut response);

    return response;
}