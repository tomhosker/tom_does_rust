/// My attempt at Project Euler Problem 7, in Rust (obv).

// Local constants.
const TARGET_PRIME_COUNT: i32 = 100001;

/// Determine whether a given number is prime.
fn is_prime(num: i32) -> bool {
    if num == 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num%2 == 0 {
        return false;
    }

    let terminus = ((num as f64).sqrt().ceil() as i32)+1;

    for i in 3..terminus {
        if num%i == 0 {
            return false;
        }
    }

    return true;
}

// Find the nth prime number.
fn find_nth_prime(n: i32) -> i32 {
    let mut count = 0;
    let mut current = 0;

    while count < n {
        current += 1;

        if is_prime(current) {
            count += 1;
        }
    }

    return current;
}

/// This is where the magic happens.
fn main() {
    let solution: i32 = find_nth_prime(TARGET_PRIME_COUNT);
    println!("{}", solution);
}
