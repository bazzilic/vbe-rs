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