/// This is the entry point to the program.

// Declare local modules.
mod female;
mod gynaeceum;
mod ticket_machine;
mod utils;

/// This is where the magic happens.
fn main() {
    let mut gynaeceum = gynaeceum::Gynaeceum::new();

    gynaeceum.run_sim();
}
