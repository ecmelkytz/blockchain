use secp256k1::{Secp256k1, Message, SecretKey, Signature};

#[derive(Debug, Clone)]
pub struct Transaction {
  pub sender: String,
  pub receiver: String,
  pub amount: i32,
}

impl Transaction {
  pub fn calculate_hash(&self) -> Message {
    let format = format!("{:?}{:?}{:?}", self.sender, self.receiver, self.amount);
    let message = Message::from_slice(&[0xab; 32]).expect(&format);
    return message;
  }

  pub fn sign(&mut self, secret_key: SecretKey) -> Signature {
    let secp = Secp256k1::new();
    let sign = secp.sign(&self.calculate_hash(), &secret_key);
    sign
  }
}
