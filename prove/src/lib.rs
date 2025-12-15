pub use justcash_core::Input;
pub use risc0_zkvm::Receipt;
use risc0_zkvm::{default_prover, ExecutorEnv};

const JUSTCASH_ELF: &[u8] = include_bytes!("./justcash.bin");

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();
}

pub fn prove(input: Input) -> Receipt {
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover.prove(env, JUSTCASH_ELF).unwrap();

    prove_info.receipt
}
