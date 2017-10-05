use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use board::Board;
use std::io;

// Terrible std_lib way to get random numbers
pub fn rand_nanos() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
}

// State must be non-zero
fn xorshift32(state: u32) -> u32 {
    let mut x = state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
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

pub fn print_screen(b: &Board) {
    // Clear the screen
    /*
    Command::new("clear").spawn().expect(
        "Error clearing screen",
    );
    */

    // Print score
    println!("{}", b.score);

    // Print board
    b.print();
}

pub fn get_key_input(b: &mut Board) {
    let mut not_valid = true;
    while not_valid {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect(
            "Error reading line",
        );
        not_valid = false;
        match guess.as_ref() {
            "w\n" => b.up(),
            "a\n" => b.left(),
            "s\n" => b.down(),
            "d\n" => b.right(),
            _ => not_valid = true,
        }
    }
    /*
    Command::new("clear").spawn().expect(
        "Error clearing screen",
    );
    */
    for i in 0..500 {
        println!("");
    }
}

#[test]
fn test_xorshift128() {
    let mut seed = [rand_nanos(), rand_nanos(), rand_nanos(), rand_nanos()];
    println!("seed is {:?}", seed);

    println!("Some random numbers:");
    for _ in 0..10 {
        println!("{} -- seed is {:?}", xorshift128(&mut seed) % 16, seed);
    }

}
