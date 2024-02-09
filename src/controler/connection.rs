#![allow(non_snake_case)]

use crate::model::beneficiaries::beneficiary::Beneficiary;
use crate::model::users::user::User;
use bincode::Decode;
use dioxus_hooks::UseSharedState;
use dotenv::var;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use base64::Engine;
use base64::engine::general_purpose;
use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use chacha20poly1305::aead::Aead;
use rand::Rng;

pub(crate) enum ConnectionUrls {
    Login,
    GetUsers,
    CreateUser,
    UpdateUser,
    DeleteUser,
    GetBeneficiaries,
    GetBeneficiary,
    SearchBeneficiary,
    CreateBeneficiary,
    UpdateBeneficiary,
    InsertPresence,
    DeletePresence,
    InsertAllergy,
    DeleteAllergy,
    GetStats,
}

impl Display for ConnectionUrls {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let base_url = var("CONNECTION_URL").unwrap();
        match self {
            ConnectionUrls::Login => write!(f, "{}/login", base_url),
            ConnectionUrls::GetUsers => write!(f, "{}/getUsers", base_url),
            ConnectionUrls::CreateUser => write!(f, "{}/createUser", base_url),
            ConnectionUrls::UpdateUser => write!(f, "{}/updateUser", base_url),
            ConnectionUrls::DeleteUser => write!(f, "{}/deleteUser", base_url),
            ConnectionUrls::GetBeneficiaries => write!(f, "{}/getBeneficiaries", base_url),
            ConnectionUrls::GetBeneficiary => write!(f, "{}/getBeneficiary", base_url),
            ConnectionUrls::SearchBeneficiary => write!(f, "{}/searchBeneficiaries", base_url),
            ConnectionUrls::CreateBeneficiary => write!(f, "{}/createBeneficiary", base_url),
            ConnectionUrls::UpdateBeneficiary => write!(f, "{}/updateBeneficiary", base_url),
            ConnectionUrls::InsertPresence => write!(f, "{}/insertPresence", base_url),
            ConnectionUrls::DeletePresence => write!(f, "{}/deletePresence", base_url),
            ConnectionUrls::InsertAllergy => write!(f, "{}/insertAllergy", base_url),
            ConnectionUrls::DeleteAllergy => write!(f, "{}/deleteAllergy", base_url),
            ConnectionUrls::GetStats => write!(f, "{}/getStats", base_url),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct Token {
    pub(crate) Token: String,
}

impl Token {
    pub(crate) fn new(token: String) -> Self {
        Self { Token: token }
    }
}

impl From<UseSharedState<User>> for Token {
    fn from(value: UseSharedState<User>) -> Self {
        Self::new(value.read().Token.clone())
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenBeneficiary {
    pub(crate) Token: String,
    pub(crate) Beneficiary: Beneficiary,
}

impl TokenBeneficiary {
    pub(crate) fn new(token: Token, beneficiary: Beneficiary) -> Self {
        Self {
            Token: token.Token,
            Beneficiary: beneficiary,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenBeneficiaryId {
    pub(crate) Token: String,
    pub(crate) Id: i32,
}

impl TokenBeneficiaryId {
    pub(crate) fn new(token: Token, id: i32) -> Self {
        Self {
            Token: token.Token,
            Id: id,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Decode)]
pub(crate) struct TokenSearch {
    pub(crate) Token: String,
    pub(crate) Search: String,
}

pub(crate) fn encrypt(plaintext: &[u8]) -> String {
    let env_key = dotenv::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key_bytes = general_purpose::STANDARD
        .decode(env_key)
        .expect("failed to decode key");
    let key = Key::from_slice(&key_bytes);

    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .expect("failed to encrypt content");

    let nonce_b64 = general_purpose::STANDARD.encode(nonce_bytes);
    let ciphertext_b64 = general_purpose::STANDARD.encode(ciphertext);

    format!("{}:{}", nonce_b64, ciphertext_b64)
}

pub(crate) fn decrypt(cipher_text: &str) -> Vec<u8> {
    let env_key = dotenv::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY must be set");
    let key_bytes = general_purpose::STANDARD
        .decode(env_key)
        .expect("failed to decode key");
    let key = Key::from_slice(&key_bytes);
    let cipher = ChaCha20Poly1305::new(key);
    let (nonce_bytes, cipher_text_bytes) =
        cipher_text.split_once(':').expect("invalid cipher text");
    let nonce = general_purpose::STANDARD
        .decode(nonce_bytes)
        .expect("failed to decode nonce");
    if nonce.len() != 12 {
        println!("Nonce: {:?} - With len : {}", nonce, nonce.len());
        return vec![];
    }
    let nonce = Nonce::from_slice(&nonce);
    let cipher_text = general_purpose::STANDARD
        .decode(cipher_text_bytes)
        .expect("failed to decode cipher text");
    if let Ok(decrypted_text) = cipher.decrypt(nonce, cipher_text.as_ref()) {
        decrypted_text
    } else {
        vec![]
    }
}
