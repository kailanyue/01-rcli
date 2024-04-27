use crate::{process_genpass, Base64Format, TextSignFormat};
use anyhow::{Ok, Result};
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    Engine,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{collections::HashMap, io::Read};

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};

// 1.文本签名的接口
pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

// 2.文本验证的接口
pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

// 3.文本加密的接口
pub trait TextEncrypt {
    fn encrypt(&self, format: Base64Format, reader: &mut dyn Read) -> Result<String>;
}

pub trait TextDecrypt {
    fn decrypt(&self, format: Base64Format, reader: &mut dyn Read) -> Result<String>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub struct ChaCha20 {
    key: [u8; 32],
    nonce: [u8; 12],
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes().to_vec())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        Ok(hash.as_bytes() == sig)
    }
}

impl TextSigner for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl TextEncrypt for ChaCha20 {
    fn encrypt(
        &self,
        format: Base64Format,
        reader: &mut dyn Read,
    ) -> Result<String, anyhow::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.key));
        let ciphertext = cipher.encrypt(Nonce::from_slice(&self.nonce), buf.as_ref());

        match ciphertext.is_ok() {
            true => match format {
                Base64Format::Standard => Ok(STANDARD.encode(ciphertext.unwrap())),
                Base64Format::UrlSafe => Ok(URL_SAFE_NO_PAD.encode(ciphertext.unwrap())),
            },
            false => Err(anyhow::anyhow!("encryptor failed")),
        }
    }
}

impl TextDecrypt for ChaCha20 {
    fn decrypt(
        &self,
        format: Base64Format,
        reader: &mut dyn Read,
    ) -> Result<String, anyhow::Error> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        let ciphertext_decode = match format {
            Base64Format::Standard => STANDARD.decode(&buf),
            Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(&buf),
        };

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.key));

        match ciphertext_decode.is_ok() {
            true => {
                let ciphertext = cipher.decrypt(
                    Nonce::from_slice(&self.nonce),
                    ciphertext_decode.unwrap().as_ref(),
                );
                match ciphertext.is_ok() {
                    true => Ok(String::from_utf8_lossy(&ciphertext.unwrap()).to_string()),
                    false => Err(anyhow::anyhow!("decrypt failed")),
                }
            }
            false => Err(anyhow::anyhow!("input decode failed")),
        }
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    // impl AsRef<[u8]> 表示能够通过 as_ref() 方法得到 &[u8]
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        // convert &[u8] to &[u8; 32]
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(
            32,
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
            "true".to_string(),
        )?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}

impl Ed25519Signer {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(key);
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }

    fn generate() -> Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).into();

        let mut map = HashMap::new();
        map.insert("ed25519.sk", sk.to_bytes().to_vec());
        map.insert("ed25519.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

impl ChaCha20 {
    pub fn new(key: [u8; 32], nonce: [u8; 12]) -> Self {
        Self { key, nonce }
    }

    pub fn try_new(key: impl AsRef<[u8]>, nonce: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let nonce = nonce.as_ref();
        let key = (&key[..32]).try_into()?;
        let nonce = (&nonce[..12]).try_into()?;
        Ok(Self::new(key, nonce))
    }
}

pub fn process_text_sign(
    reader: &mut dyn Read,
    key: &[u8],
    format: TextSignFormat,
) -> Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Signer::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_text_verify(
    reader: &mut dyn Read,
    key: &[u8],
    sig: &[u8],
    format: TextSignFormat,
) -> Result<bool> {
    let verifier: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verifier.verify(reader, sig)
}

pub fn process_text_key_generate(format: TextSignFormat) -> Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

pub fn process_text_encrypt(
    reader: &mut dyn Read,
    key: &[u8],
    nonce: &[u8],
    format: Base64Format,
) -> Result<String> {
    let chacha20 = ChaCha20::try_new(key, nonce)?;
    Ok(chacha20.encrypt(format, reader)?)
}

pub fn process_text_decrypt(
    reader: &mut dyn Read,
    key: &[u8],
    nonce: &[u8],
    format: Base64Format,
) -> Result<String> {
    let chacha20 = ChaCha20::try_new(key, nonce)?;
    Ok(chacha20.decrypt(format, reader)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{
        engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
        Engine,
    };

    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");

    use chacha20poly1305::{
        aead::{Aead, KeyInit},
        ChaCha20Poly1305, Key, Nonce,
    };

    #[test]
    fn test_process_text_sign() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = process_text_sign(&mut reader, KEY, format)?;
        let ret = process_text_verify(&mut reader1, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    fn test_process_text_verify() -> Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = URL_SAFE_NO_PAD.decode(sig)?;
        let ret = process_text_verify(&mut reader, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }
    #[test]
    fn test_process_text_chacha20() -> Result<()> {
        let mut reader = "hello world!".as_bytes();
        let format = Base64Format::Standard;
        let key: &[u8] = include_bytes!("../../fixtures/chacha20_key.txt");
        let nonce: &[u8] = include_bytes!("../../fixtures/chacha20_nonce.txt");

        let encrypt_str = process_text_encrypt(&mut reader, key, nonce, format)?;

        let mut encrypt_str = encrypt_str.as_bytes();
        let decrypt_str = process_text_decrypt(&mut encrypt_str, key, nonce, format)?;

        assert_eq!(decrypt_str, "hello world!");
        Ok(())
    }

    #[test]
    fn test_chacha20() {
        // let key = Key::from_slice(b"an example very very secret key.");
        let key = Key::from_slice(&KEY[0..32]);
        let cipher = ChaCha20Poly1305::new(key);
        let nonce = Nonce::from_slice(&KEY[0..12]);

        let ciphertext = cipher.encrypt(nonce, b"hello world!".as_ref()).unwrap();
        println!("ciphertext: {:?}", String::from_utf8_lossy(&ciphertext));

        let ciphertext_base64 = STANDARD.encode(&ciphertext);
        println!("ciphertext_base64: {}", ciphertext_base64);

        let ciphertext_decode: Vec<u8> = STANDARD.decode(ciphertext_base64).unwrap();
        println!(
            "ciphertext_decode: {}",
            String::from_utf8_lossy(&ciphertext_decode)
        );

        let plaintext = cipher.decrypt(nonce, ciphertext_decode.as_ref()).unwrap();
        println!("plaintext: {:?}", String::from_utf8_lossy(&plaintext));
        assert_eq!(&plaintext, b"hello world!");
    }
}
