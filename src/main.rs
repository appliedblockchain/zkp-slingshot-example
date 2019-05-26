#![allow(non_snake_case)]
extern crate rand_os;
extern crate zkvm;
extern crate token;
mod keys;
mod circuit;
mod tx;
mod verify;
mod utils;
use tx::generateTransaction;
use verify::verifyTransaction;
use circuit::createIssuance;
use circuit::createZKToken;

fn main() {
  let issuerKey   = keys::create();
  let userKey     = keys::create();
  let contractKey = keys::create();

  let issuerKeyBlindedKey = keys::createBlinded(issuerKey);
  let userBlindedKey      = keys::createBlinded(userKey);

  // create token
  let token = createZKToken(issuerKeyBlindedKey);
  let issueTokenContract = createIssuance( contractKey, userBlindedKey, token ); // issue token commitment

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



pub fn getUtxos() {
  // ...
}

pub fn buildTx() {
  // build_tx(...)
  // ...
}
