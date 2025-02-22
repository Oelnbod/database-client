use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encrypt(key_str: String, plaintext: String) -> String {
    let key = Key::<Aes256Gcm>::from_slice(&key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes()).expect("err");

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    let string_encrypted_data = STANDARD.encode(encrypted_data);

    string_encrypted_data
}

pub fn decrypt(key_str: String, string_encrypted_data: String) {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let encrypted_data = STANDARD
        .decode(string_encrypted_data)
        .expect("error decoding from Base64");
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = Aes256Gcm::new(key);

    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext)
        .expect("Decryption failed");

    let plaintext = String::from_utf8(decrypted_bytes).expect("invalid UTF8 in decryption data");
    println!("{}", plaintext);
}
