pub const fn div_rem<const N: usize>(i: usize) -> (usize, usize) {
    (i / N, i % N)
}
