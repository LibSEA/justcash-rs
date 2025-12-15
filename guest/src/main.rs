use justcash_core::{Commit, Input, DIM};
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn verify_tree(input: &Input) -> Commit {
    let sknk = [input.sk.as_bytes(), input.nk.as_bytes()].concat();
    let mut cur = *Impl::hash_bytes(&sknk);

    for n in 0..DIM {
        if input.directions[n] == 1 {
            let byts = [input.hashes[n].as_bytes(), cur.as_bytes()].concat();
            cur = *Impl::hash_bytes(&byts);
        } else {
            let byts = [cur.as_bytes(), input.hashes[n].as_bytes()].concat();
            cur = *Impl::hash_bytes(&byts);
        }
    }

    Commit {
        nullifer_hash: *Impl::hash_bytes(input.nk.as_bytes()),
        root: cur,
    }
}

fn main() {
    // read the input
    let input: Input = env::read();

    let commit = verify_tree(&input);

    // write public output to the journal
    env::commit(&commit);
}
