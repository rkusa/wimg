use crate::Image;
use xxhash_rust::const_xxh64::xxh64;

pub fn hash(img: &Image) -> u64 {
    xxh64(img.as_ref(), 0)
}
