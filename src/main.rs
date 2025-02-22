//example api link: http://192.168.1.120:7878/seckey/delete/foo.test
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use reqwest::{get, Error};
use serde::Deserialize;
use serde_json;
use std::io::stdin;
use std::str;
use tokio;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let aes_key = "thiskeystrmustbe32charlongtowork".to_string();
    loop {
        println!(
            "\n**Actions availiable**
list_all
list_row
add
delete\n"
        );
        let action = input("Enter the desired action");

        let parameters = match action.as_str() {
            "list_all" => "",
            "list_row" => &input("Enter the search query"),
            "add" => {
                let website = input("Enter the website: ");
                let username = input("Enter the Username/email: ");
                let password = input("Enter the password: ");
                let password = encrypt(aes_key.clone(), password);

                &format!("{website},{username},{password}").to_string()
            }
            "delete" => &input("Enter the website of the row that you wish to delete: "),
            _ => "",
        };

        let query = format!("http://{SOCKET}/{KEY}/{action}/{parameters}");
        println!("query: {}", query);

        let result = get(query).await?.text().await?;

        //this is validating if the response is JSON
        if serde_json::from_str::<serde_json::Value>(&result).is_ok() == true {
            let vec_result: Vec<PasswordEntry> =
                serde_json::from_str(&result).expect("Error converting json to string");
            println!("{:?}", vec_result.len());
	    generate_list_of_parameters(vec_result, "website");
        } else {
            println!("{:?}", result);
        }
    }
}
fn input(question: &str) -> String {
    println!("{}", question);
    //reading input from the console
    let mut text = String::new();
    stdin()
        .read_line(&mut text)
        .expect("Error reading input from console");
    text.pop();
    text
}
fn encrypt(key_str: String, plaintext: String) -> String {
    let key = Key::<Aes256Gcm>::from_slice(&key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes()).expect("err");

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    let string_encrypted_data = STANDARD.encode(encrypted_data);

    string_encrypted_data
}

fn decrypt(key_str: String, string_encrypted_data: String) {
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
fn generate_list_of_parameters(data: Vec<PasswordEntry>, parameter: &str) {
    for rows in data {
	match parameter {
	    "id" => println!("{}", rows.id),
	    "website" => println!("{}", rows.website),
	    "username" => println!("{}", rows.username),
	    "password" => println!("{}", rows.password),
	    _ => println!("invalid parameter requested"),
	}
	
    }
}

#[derive(Deserialize, Debug)]
struct PasswordEntry {
    id: u32,
    website: String,
    username: String,
    password: String,
}

//change this for deployment. Possibly read from .yaml config file.
const SOCKET: &str = "localhost:7878";
const KEY: &str = "seckey"; //this is the api key
