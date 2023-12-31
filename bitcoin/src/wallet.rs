use secp256k1::{rand::rngs::OsRng, PublicKey, Secp256k1, SecretKey};
use crypto::{digest::Digest, sha2::Sha256, ripemd160::Ripemd160};

#[derive(Debug)]
pub struct BtcWallet {
  pub secret_key: SecretKey,
  pub public_key: PublicKey,
  pub address: String
}

impl BtcWallet {
  pub fn new() -> Self {
    let (sk, pk) = generate_keypair();
    let address = wallet_address(&pk);
    BtcWallet {
      public_key: pk,
      secret_key: sk,
      address: address
    }
  }
}

pub fn generate_keypair() -> (SecretKey, PublicKey) {
  let secp = Secp256k1::new();
  let mut rng = OsRng::new().expect("OsRng");
  secp.generate_keypair(&mut rng)
}

pub fn wallet_address(public_key: &PublicKey) -> String {
  let public_key = public_key.serialize();
  let address = base58check(&public_key);
  bs58::encode(address).into_string()
}

pub fn ripemd160(input: &[u8]) -> Vec<u8> {
  let mut ripemder = Ripemd160::new();
  let mut hash = vec![0; ripemder.output_bytes()];
  ripemder.input(&input); 
  ripemder.result(&mut hash);
  hash
}

pub fn sha256(input: &[u8]) -> Vec<u8> {
  let mut hasher = Sha256::new();
  let mut hash = vec![0; hasher.output_bytes()];
  hasher.input(&input); 
  hasher.result(&mut hash);
  hash
}

pub fn hash160(input: &[u8]) -> Vec<u8> {
  let mut res = sha256(&input);
  res = ripemd160(&res);
  res
}

fn double_sha256(bytes : &Vec<u8>) -> Vec<u8> {
  let mut hasher = Sha256::new();
  let mut hash = vec![0; hasher.output_bytes()];
  hasher.input(&bytes);
  hasher.result(&mut hash);
  hasher.reset();
  hasher.input(&hash);
  hasher.result(&mut hash);
  hash
}

pub fn base58check(public_key: &[u8]) -> Vec<u8> {
  let mut address = Vec::new();
  address.extend(vec![0x00]);
  let hash_pk = hash160(&public_key);
  address.extend(hash_pk);
  let double_sha = double_sha256(&address);
  let checksum = hex::encode(&double_sha);
  address.extend(checksum[0..4].bytes());
  address
}
