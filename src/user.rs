use pqcrypto_kyber::kyber1024::{
    keypair, encapsulate, decapsulate, PublicKey, SecretKey, Ciphertext, SharedSecret,
};
use pqcrypto_traits::kem::{
    PublicKey as PKTrait, SecretKey as SKTrait, Ciphertext as CTrait, SharedSecret as SSTrait,
};

use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce}; // GCM with 256-bit key
use aes_gcm::aead::{Aead, OsRng};
use rand::RngCore;

/// A post-quantum secure user with Kyber keypair and AES-GCM message encryption
pub struct User {
    pub name: String,
    pub public_key: PublicKey,
    private_key: SecretKey,
}

impl User {
    /// Generate a new user with a Kyber keypair
    pub fn new(name: &str) -> Self {
        let (pk, sk) = keypair();
        Self {
            name: name.to_string(),
            public_key: pk,
            private_key: sk,
        }
    }

    /// Encrypt a shared secret using the recipient's public key (Kyber encapsulation)
    pub fn send_secret(&self, recipient_pk: &PublicKey) -> (Vec<u8>, SharedSecret) {
        let (ss, ct) = encapsulate(recipient_pk);
        (ct.as_bytes().to_vec(), ss)
    }

    /// Decrypt a shared secret using this user's private key (Kyber decapsulation)
    pub fn receive_secret(&self, ciphertext: &[u8]) -> Result<SharedSecret, &'static str> {
        let ct = Ciphertext::from_bytes(ciphertext).map_err(|_| "Invalid ciphertext")?;
        Ok(decapsulate(&ct, &self.private_key))
    }

    /// Encrypt a plaintext message with AES-GCM using a Kyber-derived shared secret
    pub fn encrypt_message(
        &self,
        message: &str,
        shared_secret: &SharedSecret,
    ) -> (Vec<u8>, [u8; 12]) {
        let cipher_key = Key::<Aes256Gcm>::from_slice(&shared_secret.as_bytes()[..32]);
        let cipher = Aes256Gcm::new(cipher_key);

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, message.as_bytes())
            .expect("Encryption failed");

        (ciphertext, nonce_bytes)
    }

    /// Decrypt a message encrypted with AES-GCM using the shared secret and nonce
    pub fn decrypt_message(
        &self,
        ciphertext: &[u8],
        nonce: &[u8; 12],
        shared_secret: &SharedSecret,
    ) -> Result<String, &'static str> {
        let cipher_key = Key::<Aes256Gcm>::from_slice(&shared_secret.as_bytes()[..32]);
        let cipher = Aes256Gcm::new(cipher_key);
        let nonce = Nonce::from_slice(nonce);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Decryption failed")?;

        String::from_utf8(plaintext).map_err(|_| "UTF-8 conversion failed")
    }

    /// 🔐 Wraps key exchange + AES encryption — for use in ChatRoom
    pub fn encrypt(&self, recipient: &User, message: &str) -> Option<(Vec<u8>, Vec<u8>)> {
        let (ciphertext, shared_secret) = self.send_secret(&recipient.public_key);
        let (encrypted_message, nonce) = self.encrypt_message(message, &shared_secret);
        Some((encrypted_message, nonce.to_vec()))
    }

    /// 🔓 Wraps AES decryption + Kyber decapsulation — for use in ChatRoom
    pub fn decrypt(
        &self,
        sender: &User,
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Option<String> {
        let shared_secret = self.receive_secret(ciphertext).ok()?;
        let mut nonce_fixed = [0u8; 12];
        nonce_fixed.copy_from_slice(nonce);
        self.decrypt_message(ciphertext, &nonce_fixed, &shared_secret).ok()
    }
}
