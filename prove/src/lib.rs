pub use justcash_core::{Commit, Digest, Input, DIM};
pub use risc0_zkvm::Receipt;
use risc0_zkvm::{default_prover, ExecutorEnv};
use thiserror::Error;

const JUSTCASH_ELF: &[u8] = include_bytes!("./justcash.bin");

#[derive(Error, Debug)]
pub enum ProveError {
    #[error("input invalid")]
    InputInvalid,
    #[error("failed to build environment")]
    EnvBuildFailed,
    #[error("failed proof")]
    ProofFailed,
}

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}

pub fn prove(input: Input) -> Result<Receipt, ProveError> {
    let env = ExecutorEnv::builder()
        .write(&input)
        .map_err(|_| ProveError::InputInvalid)?
        .build()
        .map_err(|_| ProveError::EnvBuildFailed)?;

    let prover = default_prover();

    let prove_info = prover
        .prove(env, JUSTCASH_ELF)
        .map_err(|_| ProveError::ProofFailed)?;

    Ok(prove_info.receipt)
}
