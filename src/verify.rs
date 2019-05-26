use bulletproofs::{ BulletproofGens };
use crate::zkvm::{ Verifier };

pub fn verifyTransaction(transaction: zkvm::Tx) -> bool {
  let bp_gens = BulletproofGens::new(256, 1);
  let verified = Verifier::verify_tx(&transaction, &bp_gens).is_ok();
  if verified { println!("TX Verified"); }
  return verified;
}
