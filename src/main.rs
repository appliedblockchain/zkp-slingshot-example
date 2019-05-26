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
  let mut issue_rand: OsRng = OsRng::new().unwrap();
  let issue_key = Scalar::random(&mut issue_rand);
  let mut dest_rand: OsRng = OsRng::new().unwrap();
  let dest_key = Scalar::random(&mut dest_rand);
  let mut contract_rand: OsRng = OsRng::new().unwrap();
  let contract_key = Scalar::random(&mut contract_rand);

  println!("Create keys:\n");
  println!("  issue_key:    {:?}", issue_key);
  println!("  dest_key:     {:?}", dest_key);
  println!("  contract_key: {:?}", contract_key);
  println!("\n");

  let token = Token::new(
    Predicate::Key(
      VerificationKey::from_secret(&issue_key)
    ),
    b"ABcoins".to_vec(),
  );
  // println!("Token: {:#?}", token);

  let dest = Predicate::Key(VerificationKey::from_secret(&dest_key));
  // println!("dest predicate: {:#?}", dest);

  let program = Program::build(|p| {
    add_contract(p, &contract_key);
    token.issue_to(p, 10u64, dest.clone())
  });
  // println!("program: {:#?}", program);

  let prog = program;

  // TODO, use _ ?
  let ( tx, tx_id, _issue_txlog ) = build_tx(prog, vec![issue_key, contract_key]).unwrap();
  println!("TX ID: {:?}", tx_id);

  // Verify tx
  let bp_gens = BulletproofGens::new(256, 1);
  // assert!(Verifier::verify_tx(&tx, &bp_gens).is_ok());
  let verified = Verifier::verify_tx(&tx, &bp_gens).is_ok();
  println!("TX Verified");
  println!("{:?}", verified);


  // let ( tx, tx_id, _issue_txlog ) = build_tx(program1.clone(), vec![dest_key, contract_key]).unwrap();
  // println!("TX ID: {:?}", tx_id);


}
