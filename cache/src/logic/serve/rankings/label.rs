use std::mem::transmute;
use int_hash::IntHashMap;

use common::model::types::LabelId;
use common::model::types::LabelCacheIndex;
use common::model::types::DayId;
use common::model::types::MonthId;
use common::model::types::WeekId;

use super::super::super::super::cache::cache_reader::CacheReader;
use super::super::super::super::cache::model::VoteCount;
use super::super::super::super::data::byte_counts::ByteCounts;
use super::super::super::super::server::codes;

// NOTE: max page size must fin into u16
const PAGE_SIZE: u32 = 1024;

const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for label cache index (if any
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
        // space for label cache index (if any
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
        // space for label cache index (if any
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
        // space for label cache index (if any
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
        // space for label cache index (if any
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
        // space for label cache index (if any
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
        // space for label cache index (if any
        4 +
        // space for label ids & vote counts
        PAGE_SIZE * (2 + 3)
        // space for the byte counts
        + PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub fn get_todays_label_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().todays_vc_day_id,
        vc_day_id,
        &cache.get_label_index_map().today,
        &cache.get_label_poll_rankings().today,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().today[38],
    );
}

pub fn get_todays_label_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().todays_vc_day_id,
        vc_day_id,
        &cache.get_label_poll_rankings().today,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().today[38],
    );
}

pub fn get_yesterdays_label_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().yesterdays_vc_day_id,
        vc_day_id,
        &cache.get_label_index_map().yesterday,
        &cache.get_label_poll_rankings().yesterday,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().yesterday[38],
    );
}

pub fn get_yesterdays_label_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().yesterdays_vc_day_id,
        vc_day_id,
        &cache.get_label_poll_rankings().yesterday,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().yesterday[38],
    );
}

pub fn get_day_b4_yesterdays_label_rankings_by_global_id(
    vc_day_id: DayId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().day_b4_yesterdays_vc_day_id,
        vc_day_id,
        &cache.get_label_index_map().day_b4_yesterday,
        &cache.get_label_poll_rankings().day_b4_yesterday,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().day_b4_yesterday[38],
    );
}

pub fn get_day_b4_yesterdays_label_rankings_by_cache_index(
    vc_day_id: DayId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().day_b4_yesterdays_vc_day_id,
        vc_day_id,
        &cache.get_label_poll_rankings().day_b4_yesterday,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().day_b4_yesterday[38],
    );
}

pub fn get_this_weeks_label_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().this_weeks_vc_week_id,
        vc_week_id,
        &cache.get_label_index_map().this_week,
        &cache.get_label_poll_rankings().this_week,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().this_week[38],
    );
}

pub fn get_this_weeks_label_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().this_weeks_vc_week_id,
        vc_week_id,
        &cache.get_label_poll_rankings().this_week,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().this_week[38],
    );
}

pub fn get_last_weeks_label_rankings_by_global_id(
    vc_week_id: WeekId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().last_weeks_vc_week_id,
        vc_week_id,
        &cache.get_label_index_map().last_week,
        &cache.get_label_poll_rankings().last_week,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().last_week[38],
    );
}

pub fn get_last_weeks_label_rankings_by_cache_index(
    vc_week_id: WeekId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().last_weeks_vc_week_id,
        vc_week_id,
        &cache.get_label_poll_rankings().last_week,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().last_week[38],
    );
}

pub fn get_this_months_label_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().this_months_vc_month_id,
        vc_month_id,
        &cache.get_label_index_map().this_month,
        &cache.get_label_poll_rankings().this_month,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().this_month[38],
    );
}

pub fn get_this_months_label_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().this_months_vc_month_id,
        vc_month_id,
        &cache.get_label_poll_rankings().this_month,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().this_month[38],
    );
}

pub fn get_last_months_label_rankings_by_global_id(
    vc_month_id: MonthId,
    block_index: u32,
    global_label_id: LabelId,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_global_id(
        cache.get_label_cache_period_ids().last_months_vc_month_id,
        vc_month_id,
        &cache.get_label_index_map().last_month,
        &cache.get_label_poll_rankings().last_month,
        global_label_id,
        block_index,
        cache.get_poll_id_byte_counts().last_month[38],
    );
}

pub fn get_last_months_label_rankings_by_cache_index(
    vc_month_id: MonthId,
    block_index: u32,
    label_cache_index: LabelCacheIndex,
    cache: &Box<CacheReader + Send + Sync>,
) -> Vec<u8> {
    return get_label_rankings_by_cache_index(
        cache.get_label_cache_period_ids().last_months_vc_month_id,
        vc_month_id,
        &cache.get_label_poll_rankings().last_month,
        label_cache_index,
        block_index,
        cache.get_poll_id_byte_counts().last_month[38],
    );
}

fn get_label_rankings_by_global_id(
    current_period_id: u32,
    expected_period_id: u32,
    label_index_map: &IntHashMap<LabelId, LabelCacheIndex>,
    given_period_label_poll_rankings: &Vec<Vec<VoteCount>>,
    global_label_id: LabelId,
    block_index: u32,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let first_record_index = PAGE_SIZE * block_index;

    match label_index_map.get(&global_label_id) {
        None => {
            return codes::INVALID_GLOBAL_CATEGORY_ID_RESPONSE.to_vec();
        }
        Some(label_cache_index) => {
            return get_label_rankings_with_label_cache_index(
                first_record_index, *label_cache_index,
                given_period_label_poll_rankings, max_poll_number_bytes);
        }
    }
}

fn get_label_rankings_by_cache_index(
    current_period_id: u32,
    expected_period_id: u32,
    vote_counts_by_label_index: &Vec<Vec<VoteCount>>,
    label_cache_index: LabelCacheIndex,
    block_index: u32,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    if current_period_id != expected_period_id {
        return codes::INVALID_PERIOD_ID_RESPONSE.to_vec();
    }

    let first_record_index = PAGE_SIZE * block_index;

    match vote_counts_by_label_index.get(label_cache_index as usize) {
        None => {
            return codes::INVALID_CATEGORY_CACHE_INDEX_RESPONSE.to_vec();
        }
        Some(_) => {
            return get_label_rankings(
                first_record_index, label_cache_index,
                vote_counts_by_label_index, max_poll_number_bytes);
        }
    }
}

#[inline]
fn get_label_rankings_with_label_cache_index(
    first_record_index: u32,
    label_cache_index: LabelCacheIndex,
    vote_counts_by_label_index: &Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_label: &Vec<VoteCount> = vote_counts_by_label_index.get(label_cache_index as usize).unwrap();
    let label_cache_index_bytes: [u8; 4] = unsafe {
        transmute(label_cache_index)
    };


    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);
            response.push(0b00000011);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_3_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);
            response.push(0b00000100);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_4_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);
            response.push(0b00000101);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_5_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);
            response.push(0b00000110);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_6_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);
            response.push(0b00000111);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_7_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);
            response.push(0b00000000);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_8_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);
            response.push(0b00000010);
            response.extend_from_slice(&label_cache_index_bytes);

            return get_2_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        _ => {
            panic!("Unexpected number of bytes {}", max_poll_number_bytes)
        }
    }
//            return codes::INVALID_CATEGORY_RESPONSE.to_vec();
}

#[inline]
fn get_label_rankings(
    first_record_index: u32,
    label_cache_index: LabelCacheIndex,
    given_period_label_poll_rankings: &Vec<Vec<VoteCount>>,
    max_poll_number_bytes: u8,
) -> Vec<u8> {
    let vote_counts_for_label: &Vec<VoteCount> = given_period_label_poll_rankings.get(label_cache_index as usize).unwrap();

    match max_poll_number_bytes {
        3 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES as usize);
            response.push(0b00000011);
            return get_3_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        4 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES as usize);
            response.push(0b00000100);
            return get_4_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        5 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES as usize);
            response.push(0b00000101);
            return get_5_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        6 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES as usize);
            response.push(0b00000110);
            return get_6_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        7 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES as usize);
            response.push(0b00000111);
            return get_7_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        8 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES as usize);
            response.push(0b00000000);
            return get_8_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
        }
        2 => {
            let mut response: Vec<u8> = Vec::with_capacity(INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES as usize);
            response.push(0b00000010);
            return get_2_byte_recent_polls(vote_counts_for_label, first_record_index as usize, response);
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[1..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[2..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[3..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[4..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[5..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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
            Some(vote_count) => {
                response.push(vote_count.poll_type_and_tz);

                let poll_id_bytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    transmute(vote_count.poll_id)
                };
                // TODO: ALWAYS verify Big fs Little Endianness
                response.extend_from_slice(&poll_id_bytes[6..7]);

                let count_bytes: [u8; 4] = unsafe {
                    transmute(vote_count.count)
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