fn main() {
    println!("Hello, world!");

    tokenizer(String::from("(add 2 (subtract 4 2))!"))
}

enum Token {
    Number(String),
    // String(String),
    Name(String),
    ParenOpening,
    ParenClosing,
}

fn tokenizer(input: String) {
    let mut character_iterable = input.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    while let Some(char) = character_iterable.next() {
        match char {
            char if char.is_whitespace() => { println!("whitespace")}
            '(' => { tokens.push(Token::ParenOpening) }
            ')' => { tokens.push(Token::ParenClosing)}
            '0'..='9' => {
                let mut value = String::new();
                value.push(char);

                while let Some('0'..='9') = character_iterable.peek() {
                    value.push(character_iterable.next().unwrap())
                }

                tokens.push(Token::Number(value))
            }
            'a'..='z' => {
                let mut value = String::new();
                value.push(char);

                while let Some('a'..='z') = character_iterable.peek() {
                    value.push(character_iterable.next().unwrap())
                }

                tokens.push(Token::Name(value))
            }
            '"' => {
                let mut value = String::new();

                while match character_iterable.peek() {
                    Some(&'"') | None => false,
                    _ => true,
                } {
                    value.push(character_iterable.next().unwrap());
                }

                 character_iterable.next().unwrap();
            }
            _ => { panic!()}
        }
    }
}