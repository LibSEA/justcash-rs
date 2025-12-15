pub use justcash_core::Commit;
use risc0_zkvm::{Receipt, sha::Digest};
use thiserror::Error;
use hex::FromHex;

const JUSTCASH_ID: &str = "f08b1a04038ddf3a3f28f03ead1ee57c3671418122660379a3125dbd5bd3f9c0";

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("verification failed")]
    VerficationFailed,
    #[error("decode of commit failed")]
    InvalidCommit,
}

pub fn verify(receipt: Receipt) -> Result<Commit, VerifyError> {
    let digest = Digest::from_hex(JUSTCASH_ID).expect("invalid hash");
    receipt
        .verify(digest)
        .map_err(|_| VerifyError::VerficationFailed)?;

    let c: Commit = receipt
        .journal
        .decode()
        .map_err(|_| VerifyError::InvalidCommit)?;

    Ok(c)
}
