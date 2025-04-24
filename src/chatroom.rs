use std::collections::HashMap;
use crate::user::User;

pub struct ChatRoom {
    users: HashMap<String, User>,
}

impl ChatRoom {
    pub fn new() -> Self {
        ChatRoom {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.name.clone(), user);
    }

    pub fn has_user(&self, name: &str) -> bool {
        self.users.contains_key(name)
    }

    pub fn get_users(&self) -> Vec<String> {
        self.users.keys().cloned().collect()
    }

    /// Sender encrypts message for recipient, returns (ciphertext, nonce)
    pub fn send_encrypted(&self, from: &str, to: &str, message: &str) -> Option<(Vec<u8>, Vec<u8>)> {
        let sender = self.users.get(from)?;
        let recipient = self.users.get(to)?;

        // Step 1: Sender encapsulates to recipient's public key
        let (ciphertext_ct, shared_secret) = sender.send_secret(&recipient.public_key);

        // Step 2: Sender encrypts message using shared secret
        let (ciphertext_msg, nonce) = sender.encrypt_message(message, &shared_secret);

        // Combine and return
        // ciphertext_ct = Kyber ciphertext (recipient needs this to get shared secret)
        // ciphertext_msg = actual encrypted message
        let mut full_ciphertext = ciphertext_ct;
        full_ciphertext.extend(ciphertext_msg);

        Some((full_ciphertext, nonce.to_vec()))
    }

    /// Recipient decrypts the message using Kyber decapsulation and AES-GCM
    pub fn receive_encrypted(&self, to: &str, from: &str, full_ciphertext: &[u8], nonce: &[u8]) -> Option<String> {
        let recipient = self.users.get(to)?;
        let sender = self.users.get(from)?;

        // Kyber ciphertext is always fixed size for kyber1024
        let kyber_ct_len = 1568; // Kyber1024 ciphertext size in bytes
        if full_ciphertext.len() < kyber_ct_len {
            return None;
        }

        // Step 1: Extract Kyber part and message ciphertext
        let (ct_bytes, encrypted_msg) = full_ciphertext.split_at(kyber_ct_len);

        // Step 2: Recipient decapsulates to recover shared secret
        let shared_secret = recipient.receive_secret(ct_bytes).ok()?;

        // Step 3: Decrypt the message using the shared secret
        recipient.decrypt_message(encrypted_msg, nonce.try_into().ok()?, &shared_secret).ok()
    }
}
