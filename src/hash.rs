use xxhash_rust::const_xxh64::xxh64;

pub fn hash(input: &[u8], seed: u32) -> u64 {
    xxh64(input, seed as u64)
}
