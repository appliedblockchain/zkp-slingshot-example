use curve25519_dalek::scalar::Scalar;
use crate::zkvm::{ Predicate };
use musig::VerificationKey;
use rand_os::OsRng;

pub fn create() -> Scalar {
  let mut rand: OsRng = OsRng::new().unwrap();
  return Scalar::random(&mut rand);
}

pub fn createBlinded(key: Scalar) -> Predicate {
  return Predicate::Key(VerificationKey::from_secret(&key));
}
