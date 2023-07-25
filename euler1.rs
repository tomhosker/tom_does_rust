/// My attempt at Project Euler Problem 1, in Rust (obv).

// Local constants.
const FACTOR0: i32 = 3;
const FACTOR1: i32 = 5;
const LAST_MULTIPLE: i32 = 1000;

/// Ronseal.
fn get_sum_of_multiples() -> i32 {
    let mut result = 0;

    for i in 1..LAST_MULTIPLE {
        if i%FACTOR0 == 0 || i%FACTOR1 == 0 {
            result += i;
        }
    }

    return result;
}

/// This is where the magic happens.
fn main() {
    let sum_of_multiples = get_sum_of_multiples();

    println!("{}", sum_of_multiples);
}
