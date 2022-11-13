#![no_std]

#[cfg(test)]
mod tests;

pub type Set = [u8; 7936];
pub const EMPTY_SET: Set = [0u8; 7936];

#[inline]
fn item_block(key: u8) -> core::ops::Range<usize> {
    let start = usize::from(key) * 31;
    start..(start+31)
}

#[inline]
pub fn add_elem(s: &mut Set, item: &[u8; 32]) {
    s[item_block(item[0])].iter_mut().zip(item[1..32].iter()).for_each(|(i, &j)| *i |= j);
}

#[inline]
pub fn is_maybe_elem(s: &Set, item: &[u8; 32]) -> bool {
    s[item_block(item[0])].iter().zip(item[1..32].iter()).all(|(&i, &j)| (!j | i) == 0xffu8)
}

#[inline]
pub fn intersect_with(s: &mut Set, s2: &Set) {
    s.iter_mut().zip(s2.iter()).for_each(|(i, &j)| *i &= j);
}

#[inline]
pub fn union_with(s: &mut Set, s2: &Set) {
    s.iter_mut().zip(s2.iter()).for_each(|(i, &j)| *i |= j);
}
