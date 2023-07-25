/// My attempt at Project Euler Problem 7, in Rust (obv).
/// The twist is that this time I'm using a library for is_prime().

// Standard imports.
use primes;

// Local constants.
const TARGET_PRIME_COUNT: i32 = 100001;

// Find the nth prime number.
fn find_nth_prime(n: i32) -> i32 {
    let mut count = 0;
    let mut current = 0;

    while count < n {
        current += 1;

        if primes::is_prime(current) {
            count += 1;
        }
    }

    return current as i32;
}

/// This is where the magic happens.
fn main() {
    let solution = find_nth_prime(TARGET_PRIME_COUNT);
    println!("{}", solution);
}
