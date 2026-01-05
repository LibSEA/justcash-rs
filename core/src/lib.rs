pub use risc0_zkvm::sha::Digest;
use serde::{Deserialize, Serialize};

pub const DIM: usize = 20;

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub hashes: [Digest; DIM],
    pub directions: [u8; DIM],
    pub sk: Digest,
    pub nk: Digest,
}

#[derive(Serialize, Deserialize)]
pub struct Commit {
    pub nullifer_hash: Digest,
    pub root: Digest,
}
