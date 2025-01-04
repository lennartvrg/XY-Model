use std::time::SystemTime;

pub const fn div_rem<const N: usize>(i: usize) -> (usize, usize) {
    (i / N, i % N)
}

pub fn unix_time() -> i32 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i32
}