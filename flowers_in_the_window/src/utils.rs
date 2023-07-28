/// This code defines some utility functions.

// Standard import.
use rand::Rng;

/// Decide randomly whether an event with a given probability happens.
pub fn happens(event_probability: f64) -> bool {
    if rand::thread_rng().gen_range(0.0..1.0) < event_probability {
        return true;
    }

    return false;
}
