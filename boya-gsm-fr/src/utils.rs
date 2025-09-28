#[inline]
pub fn sasr(x: i16, by: u32) -> i16 {
    if x >= 0 { x >> by } else { !(-(x + 1) >> by) }
}
