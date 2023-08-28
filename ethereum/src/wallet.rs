use secp256k1::{rand::rngs::OsRng, Secp256k1, PublicKey, SecretKey};
use web3::{signing::keccak256, types::{Address}};

pub fn generate_keypair() -> (SecretKey, PublicKey) {
  let secp = Secp256k1::new();
  let mut rng = OsRng::new().expect("OsRng");
  secp.generate_keypair(&mut rng)
}

pub struct EthWallet {
  pub secret_key: SecretKey,
  pub public_key: PublicKey,
  pub address: String,
}

impl EthWallet {
  pub fn new() -> Self {
    let (sk, pk) = generate_keypair();
    let address: Address = eth_wallet_address(&pk);
    EthWallet {
      public_key: pk,
      secret_key: sk,
      address: address.to_string()
    }
  }
}

pub fn eth_wallet_address(public_key: &PublicKey) -> Address {
  let hash = keccak256(&public_key.serialize());
  Address::from_slice(&hash[12..])
}

  