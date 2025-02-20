//example api link: http://192.168.1.120:7878/seckey/delete/foo.test
use reqwest::{get, Error};
use std::io::stdin;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = input("Enter the key: ");
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
                &format!("{website},{username},{password}").to_string()
            }
            "delete" => &input("Enter the website of the row that you wish to delete: "),
            _ => "",
        };
        let query = format!("http://{SOCKET}/{key}/{action}/{parameters}");
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

//change this for deployment. Possibly read from .yaml config file.
const SOCKET: &str = "localhost:7878";
