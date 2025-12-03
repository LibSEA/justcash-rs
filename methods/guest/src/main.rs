use jc_core::{Commit, Input, DIM};
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn verify_tree(input: &Input) {
}

fn main() {
    // read the input
    let input: Input = env::read();

    // write public output to the journal
    env::commit(&Commit {
        nullifer_hash: *Impl::hash_bytes("test".as_bytes()),
        root: *Impl::hash_bytes("test".as_bytes()),
    });
}
