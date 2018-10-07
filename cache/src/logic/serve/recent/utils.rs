use common::model::types::PollId;

#[inline]
pub fn get2ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[6..7]);
    }

    return response;
}

#[inline]
pub fn get3ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[5..7]);
    }

    return response;
}

#[inline]
pub fn get4ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[4..7]);
    }

    return response;
}

#[inline]
pub fn get5ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[3..7]);
    }

    return response;
}

#[inline]
pub fn get6ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[2..7]);
    }

    return response;
}

#[inline]
pub fn get7ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes[1..7]);
    }

    return response;
}

#[inline]
pub fn get8ByteRecentPollIds(
    pollIds: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in pollIds {
        let pollIdBytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&pollIdBytes);
    }

    return response;
}
