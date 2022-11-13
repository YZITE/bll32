use super::*;

#[test]
fn it_works() {
    let mut s = crate::EMPTY_SET;
    let x = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    add_elem(&mut s, &x);
    assert!(is_maybe_elem(&s, &x));
}

#[test]
fn each_block() {
    for i in 0..=255u8 {
        for j in 0..=255u8 {
            let mut s = crate::EMPTY_SET;
            let x = [i, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, j];
            add_elem(&mut s, &x);
            assert!(is_maybe_elem(&s, &x));
        }
    }
}
