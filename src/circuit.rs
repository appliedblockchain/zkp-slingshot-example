use curve25519_dalek::scalar::Scalar;
use crate::zkvm::{ Program, Predicate };
use crate::token::{ Token };
use crate::utils::add_contract;

pub fn createIssuance(
  contract_key: Scalar,
  dest_key: Predicate,
  token: Token,
) -> Program {
  let program = Program::build(|p| {
    add_contract(p, &contract_key);
    token.issue_to(p, 10u64, dest_key.clone())
  });
  // println!("Issuance Circuit (Program):\n{:#?}", program);
  return program
}

pub fn createZKToken(issuerKeyBlindedKey: Predicate) -> Token {
  return Token::new(
    issuerKeyBlindedKey,
    b"ABcoins".to_vec(),
  );
}
