use curve25519_dalek::scalar::Scalar;
extern crate token;
extern crate zkvm;
use crate::token::{Token};
use crate::zkvm::{Commitment, Contract, Data, Predicate, Program, Value, TxEntry, Anchor};
use musig::VerificationKey;
use bulletproofs::{BulletproofGens};

fn add_contract(p: &mut Program, contract_sample_key: &Scalar) {
    let contract = Contract::new(
        Predicate::Key(VerificationKey::from_secret(contract_sample_key)),
        vec![],
        Anchor::from_raw_bytes([0u8; 32]),
    );
    p.push(contract).input().sign_tx();
}

fn main() {
    let issue_key = Scalar::from(1u64);
    let dest_key = Scalar::from(2u64);
    let contract_sample_key = Scalar::from(3u64);
    let usd = Token::new(
        Predicate::Key(VerificationKey::from_secret(&issue_key)),
        b"USD".to_vec(),
    );
    let dest = Predicate::Key(VerificationKey::from_secret(&dest_key));

    let program = Program::build(|p| {
        add_contract(p, &contract_sample_key);
        usd.issue_to(p, 10u64, dest.clone())
    });
    // build(program, vec![issue_key, contract_sample_key]).unwrap()

    // Verify tx
    let bp_gens = BulletproofGens::new(256, 1);
    // assert!(Verifier::verify_tx(&tx, &bp_gens).is_ok());
    Verifier::verify_tx(&tx, &bp_gens).is_ok()
}
