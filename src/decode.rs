/// Decode a variable-length integer from a byte stream.
///
/// Since we don't know the size of the integer, we use a `u128` to store it.
///
/// # Example
/// ```rust
/// use vbe::decode;
/// let res: u128 = decode(vec![0x7f, 0x9f].into_iter()).unwrap();
/// assert_eq!(res, 0x0fffu128);
/// ```
pub fn decode(mut i: impl Iterator<Item=u8>) -> Result<u128, String> {
    let mut res = 0u128;

    let mut idx = 0usize;
    loop {
        let b = i.next().ok_or("unexpected end of input")?;
        res |= ((b & 0x7f) as u128) << (idx * 7);
        idx += 1;
        if b & 0x80 == 0x80 {
            break;
        }
    }

    Ok(res)
}