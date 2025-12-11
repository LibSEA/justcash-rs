pub use justcash_core::Commit;
use justcash_methods::JUSTCASH_ID;
use risc0_zkvm::Receipt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("verification failed")]
    VerficationFailed,
    #[error("decode of commit failed")]
    InvalidCommit,
}

pub fn verify<T>(receipt: Receipt) -> Result<Commit, VerifyError> {
    receipt
        .verify(JUSTCASH_ID)
        .map_err(|_| VerifyError::VerficationFailed)?;

    let c: Commit = receipt
        .journal
        .decode()
        .map_err(|_| VerifyError::InvalidCommit)?;

    Ok(c)
}
