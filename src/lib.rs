extern crate rand;

/// Flip a coin. Returns a random boolean
pub fn flip() -> bool {
    rand::random::<bool>()
}
