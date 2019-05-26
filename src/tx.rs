use curve25519_dalek::scalar::Scalar;
use crate::zkvm::{ Program };
use crate::utils::build_tx;

pub fn generateTransaction(
  program: Program,
  issue_key: Scalar,
  contract_key: Scalar,
) -> zkvm::Tx {
  let ( tx, tx_id, _issue_txlog ) = build_tx(program, vec![issue_key, contract_key]).unwrap();
  // println!("TX: {}", tx); // TODO
  println!("TX ID: {:?}", tx_id);
  return tx;
}
