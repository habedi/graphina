/// Returns a nondeterministic 64-bit seed without requiring an OS entropy source.
///
/// Each `RandomState` carries per-instance keys, so consecutive calls produce
/// different values. This is not cryptographic randomness; it only seeds
/// algorithms whose caller passed no explicit seed. Unlike `rand::random`, it
/// does not pull in `getrandom`, so it works on targets like
/// `wasm32-unknown-unknown` without a JavaScript entropy backend.
#[cfg(feature = "community")]
pub(crate) fn entropy_seed() -> u64 {
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hasher};
    RandomState::new().build_hasher().finish()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "community")]
    use super::entropy_seed;

    #[cfg(feature = "community")]
    #[test]
    fn test_entropy_seed_varies_between_calls() {
        let a = entropy_seed();
        let b = entropy_seed();
        assert_ne!(a, b, "consecutive seeds should differ");
    }
}
