use curve25519_dalek::scalar::Scalar;
use crate::zkvm::{
  Anchor, Contract, Predicate, Program, Prover, Signature, Tx,  TxHeader, TxID, TxLog, VMError
};
use merlin::Transcript;
use musig::VerificationKey;
use bulletproofs::{BulletproofGens, PedersenGens};

pub fn add_contract(p: &mut Program, secret_key: &Scalar) {
  let contract = Contract::new(
    Predicate::Key(VerificationKey::from_secret(secret_key)),
    vec![],
    Anchor::from_raw_bytes([0u8; 32]),
  );
  p.push(contract).input().sign_tx();
}

// Helper functions
pub fn build_tx(program: Program, keys: Vec<Scalar>) -> Result<(Tx, TxID, TxLog), VMError> {
  let bp_gens = BulletproofGens::new(256, 1);
  let header = TxHeader {
    version: 0u64,
    mintime_ms: 0u64,
    maxtime_ms: 0u64,
  };
  // TBD: figure out better + more robust signing mechanism
  let gens = PedersenGens::default();
  let utx = Prover::build_tx(program, header, &bp_gens)?;

  // find all the secret scalars for the pubkeys used in the VM
  let privkeys: Vec<Scalar> = utx
    .signing_instructions
    .iter()
    .filter_map(|(pubkey, _msg)| {
      for k in keys.iter() {
        if (k * gens.B).compress() == *pubkey.as_compressed() {
          return Some(*k);
        }
      }
      None
    })
    .collect();

  let mut signtx_transcript = Transcript::new(b"ZkVM.signtx");
  signtx_transcript.append_message(b"txid", &utx.txid.0);
  let sig = Signature::sign_multi(
    privkeys,
    utx.signing_instructions.clone(),
    &mut signtx_transcript,
  )
  .unwrap();

  let txid = utx.txid;
  let txlog = utx.txlog.clone();
  let tx = utx.sign(sig);
  Ok((tx, txid, txlog))
}
