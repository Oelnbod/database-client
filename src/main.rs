//example api link: http://192.168.1.120:7878/seckey/delete/foo.test
use base64;
use reqwest::{get, Error};
use std::io::stdin;
use tokio;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};
use std::str;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let aes_key = "thiskeystrmustbe32charlongtowork".to_string();
    loop {
	println!("\n**Actions availiable**
list_all
list_row
add
delete\n");
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
        println!("{:?}", result);
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
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let cipher = Aes256Gcm::new(key);

    let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes())
	.expect("err");

    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);

    let string_encrypted_data = base64::encode(encrypted_data);
    string_encrypted_data
    
}


//change this for deployment. Possibly read from .yaml config file.
const SOCKET: &str = "localhost:7878";
const KEY: &str = "seckey"; //this is the api key
