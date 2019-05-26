#![allow(non_snake_case)]
extern crate rand_os;
extern crate zkvm;
extern crate token;
use rand_os::OsRng;
use curve25519_dalek::scalar::Scalar;
use musig::VerificationKey;
use bulletproofs::{BulletproofGens};
use crate::zkvm::{
  Predicate, Program, Verifier
};
use crate::token::{Token};
mod utils;
use utils::add_contract;
use utils::build_tx;

fn main() {
  let issuerKey   = createKey();
  let userKey     = createKey();
  let contractKey = createKey();

  let issuerKeyBlindedKey = createBlindedKey(issuerKey);
  let userBlindedKey      = createBlindedKey(userKey);

  // create token
  let token = createZKToken(issuerKeyBlindedKey);
  let issueTokenContract = createIssuanceCircuit( contractKey, userBlindedKey, token ); // issue token commitment

  let tx = generateTransaction(issueTokenContract, issuerKey, contractKey);

  let verified = verifyTransaction(tx);
  println!("{:?}", verified);

  // transfer token to another user - TODO
  // let user2Key = createKey();
  // let user2BlindedKey = createBlindedKey(user2Key);
  // let transferTokenContract = createTransferCircuit( userBlindedKey, user2BlindedKey, token );
  // let txTransfer = generateTransferTransaction(transferTokenContract, userBlindedKey);
  // let verifiedTransfer = verifyTransaction(tx);
  // println!("{:?}", verifiedTransfer);
}

pub fn createKey() -> Scalar {
  let mut rand: OsRng = OsRng::new().unwrap();
  return Scalar::random(&mut rand);
}

pub fn createZKToken(issuerKeyBlindedKey: Predicate) -> Token {
  return Token::new(
    issuerKeyBlindedKey,
    b"ABcoins".to_vec(),
  );
}

pub fn getUtxos() {
  // ...
}

pub fn buildTx() {
  // build_tx(...)
  // ...
}

pub fn createIssuanceCircuit(
  contract_key: Scalar,
  dest_key: Predicate,
  token: Token,
) -> Program {
  return Program::build(|p| {
    add_contract(p, &contract_key);
    token.issue_to(p, 10u64, dest_key.clone())
  });
}

pub fn generateTransaction(
  program: Program,
  issue_key: Scalar,
  contract_key: Scalar,
) -> zkvm::Tx {
  let ( tx, tx_id, _issue_txlog ) = build_tx(program, vec![issue_key, contract_key]).unwrap();
  println!("TX ID: {:?}", tx_id);
  return tx;
}

pub fn verifyTransaction(transaction: zkvm::Tx) -> bool {
  let bp_gens = BulletproofGens::new(256, 1);
  let verified = Verifier::verify_tx(&transaction, &bp_gens).is_ok();
  println!("TX Verified");
  println!("{:?}", verified);
  return verified;
}

pub fn createBlindedKey(key: Scalar) -> Predicate {
  return Predicate::Key(VerificationKey::from_secret(&key));
}
