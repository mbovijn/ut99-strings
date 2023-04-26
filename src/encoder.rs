pub fn decode_byte(bytes: [u8; 2], offset: usize) -> u8 {
    if offset > 7 {
        panic!("Offset needs to be between 0 and 7 (inclusive)");
    }
    if offset == 0 {
        return bytes[1];
    }
    (bytes[0] >> offset) + (bytes[1] << (8 - offset))
}
