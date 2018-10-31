use std::mem::transmute;

use common::model::consts;

use super::super::super::super::server::codes;
use super::super::super::super::cache::model::VoteCount;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: u32 = 1024;

pub const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (8 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (7 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (6 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (5 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (4 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (3 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

pub const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: u32 =
// space for the leading header byte
    1 +
        // space for location cache index (if any
        4 +
        // space for category cache index (if any
        4 +
        // space for location ids & vote counts
        PAGE_SIZE * (2 + 3) +
        // space for poll type
        PAGE_SIZE / 4 +
        // space for the byte counts
        PAGE_SIZE / 4 +
        // space for trailing size bytes
        2;

#[inline]
pub fn get_8_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_7_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_6_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_5_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_4_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_3_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}

#[inline]
pub fn get_2_byte_recent_polls(
    poll_rankings: Vec<VoteCount>,
    starting_index: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = poll_rankings.iter().skip(starting_index);
    let mut vote_counts_sizes = ByteCounts::new(PAGE_SIZE as usize);
    let mut poll_types = ByteCounts::new(PAGE_SIZE as usize);

    for x in 0..PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.poll_type_and_tz & 0b00000011 {
                    consts::POLL_TYPE_1D => {
                        poll_types.add1();
                    }
                    consts::POLL_TYPE_2D => {
                        poll_types.add2();
                    }
                    consts::POLL_TYPE_3D => {
                        poll_types.add3();
                    }
                    _ => {
                        panic!("Unexpected Poll Type {}", voteCount.poll_type_and_tz & 0b00000011)
                    }
                }

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
    vote_counts_sizes.append_data(response);
    poll_types.append(response);

    return response;
}