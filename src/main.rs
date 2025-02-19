use std::io::stdin;

fn main() {
    let key = input();
    println!("{}", key);
}

fn input() -> String {
    //reading input from the console
    let mut text = String::new();
    stdin().read_line(&mut text).expect("Error reading input from console");
    text.pop();
    text
}
