//example api link: http://192.168.1.120:7878/seckey/delete/foo.test
use std::io::stdin;

fn main() {
    let key = input("Enter the key: ");
    let action = input("Enter the desired action");
    let parameters = input("Enter the desired parameters");

    let query = format!("{SOCKET}/{key}/{action}/{parameters}");
    println!("query: {}", query);
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
