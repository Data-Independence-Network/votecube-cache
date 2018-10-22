use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

#[inline]
fn wrong_request_length_12(request_body: &[u8]) -> bool {
    request_body.len() != 12
}

#[inline]
fn wrong_request_length_16(request_body: &[u8]) -> bool {
    request_body.len() != 16
}

#[inline]
fn wrong_request_length_20(request_body: &[u8]) -> bool {
    request_body.len() != 20
}

#[inline]
fn wrong_request_length_24(request_body: &[u8]) -> bool {
    request_body.len() != 24
}

#[inline]
fn wrong_request_length_28(request_body: &[u8]) -> bool {
    request_body.len() != 28
}

#[inline]
fn read_two_ints_and_long(request_body: &[u8]) -> (u32, u32, u64) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_three_ints(request_body: &[u8]) -> (u32, u32, u32) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_three_ints_and_long(request_body: &[u8]) -> (u32, u32, u32, u64) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_four_ints(request_body: &[u8]) -> (u32, u32, u32, u32) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_three_ints_and_two_longs(request_body: &[u8]) -> (u32, u32, u32, u64, u64) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u64::<BigEndian>().unwrap(),
        request_data_reader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_four_ints_and_long(request_body: &[u8]) -> (u32, u32, u32, u32, u64) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u64::<BigEndian>().unwrap()
    );
}

#[inline]
fn read_five_ints(request_body: &[u8]) -> (u32, u32, u32, u32, u32) {
    let mut request_data_reader = Cursor::new(request_body);
    return (
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap(),
        request_data_reader.read_u32::<BigEndian>().unwrap()
    );
}
