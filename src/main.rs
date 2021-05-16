mod tokeniser;
mod parser;

fn main() {
    println!("Hello, world!");

    tokeniser::create_tokens(String::from("(add 2 (subtract 4 2))!"));
    return;
}

pub enum Token {
    Number(String),
    String(String),
    Name(String),
    ParenOpening,
    ParenClosing,
}
