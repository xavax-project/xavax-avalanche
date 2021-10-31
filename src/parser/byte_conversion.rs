use std::convert::TryInto;


/* ----\\\0111100001100001011101100110000101111000_we_are_one\\\ --- NON-IMPORTANT-NOTE:
    byte_conversion.rs contains functions to take a portion of a byte-array and turn it into a
    native data-type. This is used by the Parser so we can convert raw-byte payloads back into
    their respective structs.
*/

/*---- Integers ----*/

///Get a i64 from Bytes
pub fn extract_i64(slice_portion: &[u8]) -> i64 {
    let res_bytes: [u8; 8] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    i64::from_be_bytes(res_bytes)
}
///Get a i32 from Bytes
pub fn extract_i32(slice_portion: &[u8]) -> i32 {
    let res_bytes: [u8; 4] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    i32::from_be_bytes(res_bytes)
}
///Get a i16 from Bytes
pub fn extract_i16(slice_portion: &[u8]) -> i16 {
    let res_bytes: [u8; 2] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    i16::from_be_bytes(res_bytes)
}

/*---- Unsigned Integers ----*/

///Get a u32 from Bytes
pub fn extract_u64(slice_portion: &[u8]) -> u64 {
    let res_bytes: [u8; 8] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    u64::from_be_bytes(res_bytes)
}
///Get a u32 from Bytes
pub fn extract_u32(slice_portion: &[u8]) -> u32 {
    let res_bytes: [u8; 4] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    u32::from_be_bytes(res_bytes)
}
///Get a u16 from bytes
pub fn extract_u16(slice_portion: &[u8]) -> u16 {
    let res_bytes: [u8; 2] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    u16::from_be_bytes(res_bytes)
}
///Get a u16 from bytes
pub fn extract_u8(slice_portion: &[u8]) -> u8 {
    let res_bytes: [u8; 1] = slice_portion.try_into().expect("Given slice has incorrect byte length!");
    u8::from_be_bytes(res_bytes)
}