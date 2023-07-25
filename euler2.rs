/// My attempt at Project Euler Problem 2, in Rust (obv).

// Local constants.
const MAX_FIB: i32 = 4000000;

/// Get the nth term of the Fibonacci sequence beginning 1, 2...
fn fib(index: i32) -> i32 {
    if index == 0 {
        return 1;
    }
    if index == 1 {
        return 2;
    }

    let result = fib(index-1)+fib(index-2);

    return result;
}

/// Get the sum of the even-valued terms of the Fibonacci sequence under a given
/// maximum.
fn get_sum_of_even_valued_fib_terms_under_max(max: i32) -> i32 {
    let mut index = 0;
    let mut current_term = fib(index);
    let mut result = 0;

    while current_term < max {
        if current_term%2 == 0 {
            result += current_term;
        }

        index += 1;
        current_term = fib(index);
    }

    return result
}

/// This is where the magic happens.
fn main() {
    let solution: i32 = get_sum_of_even_valued_fib_terms_under_max(MAX_FIB);
    println!("{}", solution);
}
