use rand::Rng;

pub fn generate_random_id() -> u32 {
    rand::rng().random_range(1..=100)
}
