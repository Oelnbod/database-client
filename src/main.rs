//example api link: http://192.168.1.120:7878/seckey/delete/foo.test
use reqwest::{get, Error};
use serde::Deserialize;
use serde_json;
use std::io::stdin;
use std::str;
use tokio;
mod encryption;
#[tokio::main]
async fn main() -> Result<(), Error> {
    let aes_key = "thiskeystrmustbe32charlongtowork".to_string();

    loop {
        let query = format!("http://{SOCKET}/{KEY}/list_all/ ");
        let result = get(query).await?.text().await?;
        let vec_result: Vec<PasswordEntry> =
            serde_json::from_str(&result).expect("Error converting json to string");

        let websites = generate_list_of_parameters(vec_result, "website");
	
	println!(
            "\n**Actions availiable**
list_row
add
delete\n"
        );
	println!("**Websites with entry**");
	pretty_list(websites);
	println!("\n");
        let action = input("Enter the desired action");

        let parameters = match action.as_str() {
            "list_row" => &input("Enter the search query"),
            "add" => {
                let website = input("Enter the website: ");
                let username = input("Enter the Username/email: ");
                let password = input("Enter the password: ");
                let password = encryption::encrypt(aes_key.clone(), password);

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
            let websites = generate_list_of_parameters(vec_result, "website");
            println!("{:#?}", websites)
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

fn generate_list_of_parameters(data: Vec<PasswordEntry>, parameter: &str) -> Vec<String> {
    let mut return_vec = Vec::new();
    for rows in data {
        match parameter {
            "id" => return_vec.push(rows.id.to_string()),
            "website" => return_vec.push(rows.website),
            "username" => return_vec.push(rows.username),
            "password" => return_vec.push(rows.password),
            _ => return_vec.push("invalid parameter requested".to_string()),
        };
    }
    return_vec
}
fn pretty_list(vec: Vec<String>) {
    for items in vec {
        println!("{}", items);
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
