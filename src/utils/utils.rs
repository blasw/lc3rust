//sign aware bit extending input size varies, output size is always 16 bit
pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    if (x >> (bit_count - 1)) & 1 != 0 {
        //negative
        x |= 0xFFFF << bit_count;
    }
    x
}