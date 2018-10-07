use common::model::types::PollId;

#[inline]
pub fn get_2_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[6..7]);
    }

    return response;
}

#[inline]
pub fn get_3_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[5..7]);
    }

    return response;
}

#[inline]
pub fn get_4_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[4..7]);
    }

    return response;
}

#[inline]
pub fn get_5_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[3..7]);
    }

    return response;
}

#[inline]
pub fn get_6_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[2..7]);
    }

    return response;
}

#[inline]
pub fn get_7_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes[1..7]);
    }

    return response;
}

#[inline]
pub fn get_8_byte_recent_poll_ids(
    poll_ids: Vec<PollId>,
    mut response: Vec<u8>,
) -> Vec<u8> {
    for pollId in poll_ids {
        let poll_id_bytes: [u8; 8] = unsafe {
            // Poll Id in the period of a given time zone
            std::mem::transmute(*pollId);
        };
        // TODO: ALWAYS verify Big fs Little Endianness
        response.extend_from_slice(&poll_id_bytes);
    }

    return response;
}
