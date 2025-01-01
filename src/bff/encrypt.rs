#![allow(dead_code)]

use openssl::{
  base64,
  hash::MessageDigest,
  pkey::{PKey, Private},
  rsa::{Padding, Rsa},
  sign::Signer,
};

pub struct Encrypt {
  private: Rsa<Private>,
  private_pkey: PKey<Private>,
  public_pem: String,
}

impl Encrypt {
  pub fn from_file(secret_path: &str) -> Result<Self, anyhow::Error> {
    let pem_data = std::fs::read_to_string(secret_path)
      .map_err(|e| anyhow::Error::msg(format!("Failed to read PEM file, repr: {}", e)))?;

    let private = Rsa::private_key_from_pem(pem_data.as_bytes())?;
    let private_pkey = PKey::from_rsa(private.clone())?;
    let public = private.public_key_to_pem()?;

    Ok(Encrypt {
      private,
      private_pkey,
      public_pem: String::from_utf8(public)?,
    })
  }

  pub fn from_generate() -> Result<Self, anyhow::Error> {
    let private = Rsa::generate(2048)?;
    let private_pkey = PKey::from_rsa(private.clone())?;
    let public = private.public_key_to_pem()?;

    Ok(Encrypt {
      private,
      private_pkey,
      public_pem: String::from_utf8(public)?,
    })
  }

  pub fn encrypt(&self, data: &str) -> Result<String, anyhow::Error> {
    let mut encrypted = vec![0; self.private.size() as usize];
    let _ = self
      .private
      .private_encrypt(data.as_bytes(), &mut encrypted, Padding::PKCS1_PSS)?;
    Ok(base64::encode_block(&encrypted))
  }

  pub fn sign_with_private_key(&self, data: &str) -> Result<String, anyhow::Error> {
    let mut singer = Signer::new(MessageDigest::sha256(), self.private_pkey.as_ref())?;
    singer.update(&data.as_bytes())?;

    let signature = singer.sign_to_vec()?;
    Ok(base64::encode_block(&signature))
  }

  pub fn get_public(&self) -> &str {
    &self.public_pem
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic() -> Result<(), anyhow::Error> {
    let msg = "test data";

    let encrypt = Encrypt::from_generate()?;

    let cipher = encrypt.encrypt(msg)?;
    let cipher = base64::decode_block(&cipher)?;

    let public = encrypt.get_public();

    let public = Rsa::public_key_from_pem(public.as_bytes())?;

    let mut decrypted = vec![0; public.size() as usize];
    let _ = public.public_decrypt(&cipher, &mut decrypted, Padding::PKCS1_PSS);

    println!("{}", String::from_utf8(decrypted)?);

    Ok(())
  }
}
