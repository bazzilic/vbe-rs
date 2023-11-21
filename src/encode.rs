use num_traits::{PrimInt, Unsigned};
use std::mem::size_of;

/// Encode an integer into a variable-length byte array.
///
/// # Example
/// ```rust
/// use vbe::encode;
/// let res: Vec<u8> = encode(&22u32);
/// assert_eq!(res, vec![22u8 | 0x80]);
/// ```
pub fn encode<T>(i: &T) -> Vec<u8>
where
    T: PrimInt + Unsigned + Copy,
{
    let mut j = *i;

    let mut res = Vec::with_capacity((size_of::<T>() * 8) / 7 + 1);

    loop {
        let b = j & T::from(0x7f).unwrap();
        j = j >> 7;

        res.push(b.to_u8().unwrap());

        if j == T::zero() {
            break;
        }
    }

    let last = res.last_mut().unwrap();
    *last |= 0x80;

    res
}
