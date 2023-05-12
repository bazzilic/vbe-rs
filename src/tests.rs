use crate::*;

#[test]
fn test_encode() {
    assert_eq!(encode(&22u32), vec![22u8 | 0x80]);
    assert_eq!(encode(&0x0fffu16), vec![0x7f, 0x9f]);
    assert_eq!(encode(&0x0abcu64), vec![0x3c, 0x95]);
}

#[test]
fn test_encode_decode() {
    use rand::prelude::*;

    for i in 0..32 {
        let e = encode(&i);
        let d = decode(e.iter().cloned()).unwrap();
        assert_eq!(i, d);
    }

    // fuzz test
    for _ in 0..256 {
        // generate a random u128
        let mut rng = rand::thread_rng();
        let i = rng.gen::<u128>();
        let e = encode(&i);
        let d = decode(e.iter().cloned()).unwrap();
        assert_eq!(i, d);
    }
}

#[test]
fn test_decode_iterator() {
    let mut a = encode(&225648u32);
    a.append(&mut encode(&784512u64));
    a.append(&mut encode(&6598111u128));

    let mut i = a.iter().cloned();

    assert_eq!(decode(&mut i).unwrap(), 225648);
    assert_eq!(decode(&mut i).unwrap(), 784512);
    assert_eq!(decode(&mut i).unwrap(), 6598111);
}
