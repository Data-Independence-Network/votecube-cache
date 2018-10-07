use std::mem::transmute;

use super::super::super::super::server::codes;
use super::super::super::super::cache::model::VoteCount;
use super::super::super::super::data::byte_counts::ByteCounts;

// NOTE: max page size must fin into u16
const PAGE_SIZE: usize = 1024;

pub const INITIAL_RESPONSE_VECTOR_SIZE_8_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_7_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_6_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_5_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_4_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_3_POLL_BYTES: usize =
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

pub const INITIAL_RESPONSE_VECTOR_SIZE_2_POLL_BYTES: usize =
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
pub fn get8ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

                let pollIdBytes: [u8; 8] = unsafe {
                    // Poll Id in the period of a given time zone
                    std::mem::transmute(*voteCount.pollId);
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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get7ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get6ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get5ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get4ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get3ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}

#[inline]
pub fn get2ByteRecentPolls(
    pollRankings: Vec<VoteCount>,
    startingIndex: usize,
    mut response: Vec<u8>,
) -> Vec<u8> {
    let mut iterator = pollRankings.iter().skip(startingIndex);
    let mut voteCountsSizes = ByteCounts::new(PAGE_SIZE);
    let mut pollTypes = ByteCounts::new(PAGE_SIZE);

    for x in 0...PAGE_SIZE {
        match iterator.next() {
            None => break,
            Some(voteCount) => {
                // Get the poll type
                match voteCount.pollTypeAndTz & 0b00000011 {
                    codes::POLL_TYPE_1D => {
                        pollTypes.add1();
                    }
                    codes::POLL_TYPE_2D => {
                        pollTypes.add2();
                    }
                    codes::POLL_TYPE_3D => {
                        pollTypes.add3();
                    }
                }

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
    byteCounts.appendData(response);
    pollTypes.append(response);

    return response;
}