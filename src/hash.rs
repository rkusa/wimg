use crate::Image;
use sha2::{Digest, Sha256};

pub fn hash(img: &Image) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&img);
    let mut out = [0u8; 32];
    hasher.finalize_into((&mut out[..]).into());
    out
}
