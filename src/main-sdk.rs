// main client library api

fn main() {
  let tokenKey = createKey();
  let issuerKey = createKey();
  let userKey = createKey();
  let token = createZKToken(...);
  let actionIssue = createAction("issue");
  let issueTokenProof = createCircuit( addContract(token), actionIssue, userKey );
  let proof = generateProof(issueTokenCircuit)

  let userPredicate = dest

  let actionTransfer = createAction("transfer");
  let transferTokenCircuit = createCircuit( addContract(token), actionTransfer, userKey );
  let tokenTransferCircuit = createCircuit( addContract(token), userKey );

  verifyProof

  println!("Hello World!!!!!!!!!");
}

pub fun createKey() {
  // Scalar::random(&mut OsRng::new().unwrap());
  // ...
}

pub fun createZKToken() {
  // Token::new(...)
  // ...
}

pub fun getUtxos() {
  // ...
}

pub fun buildTx() {
  // build_tx(...)
  // ...
}

pub fun createCircuit() {
  // Program::build(...)
}

pub fun verifyProof() {
  // Verifier::verify_tx(...)
}

pub fun createBlindedKey(key) {
  // Predicate::Key(VerificationKey::from_secret(key)
}

pub fun addContract() {
  // ...
}
