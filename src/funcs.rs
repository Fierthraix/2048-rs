use std::time::{SystemTime, UNIX_EPOCH};
use display::*;
use ncurses::*;

// Terrible std_lib way to get random numbers
pub fn rand_nanos() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

// State must not be all zero
pub fn xorshift128(state: &mut [u32; 4]) -> u32 {
    let mut t: u32 = state[3];
    t ^= t << 11;
    t ^= t << 8;
    state[3] = state[2];
    state[2] = state[1];
    state[1] = state[0];
    t ^= state[0];
    t ^= state[0] >> 19;
    state[0] = t;
    t
}
