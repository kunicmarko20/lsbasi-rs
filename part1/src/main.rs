use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    println!("{}", Interpreter::new(input).expr());
}

struct Interpreter {
    input: String,
    position: usize,
    current_token: Option<Token>,
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        Interpreter {
            input,
            position: 0,
            current_token: None,
        }
    }

    fn next_token(&mut self) {
        if self.position >= self.text_length() - 1 {
            self.current_token = Some(Token::NONE);
            return;
        }

        let current_char: String = self.input.chars().skip(self.position).take(1).collect();

        self.position += 1;

        if let Ok(current_char_as_int) = current_char.parse::<i64>() {
            self.current_token = Some(Token::INT(current_char_as_int));
            return;
        }

        if current_char == "+" {
            self.current_token = Some(Token::OPERATION(Operation::PLUS));
            return;
        }

        panic!("Wrong Character");
    }

    fn text_length(&self) -> usize {
        self.input.len()
    }

    pub fn expr(&mut self) -> i64 {
        self.next_token();

        let left = self.current_token.clone().unwrap();
        self.next_token();

        let _operation = self.current_token.clone().unwrap();
        self.next_token();

        let right = self.current_token.clone().unwrap();
        self.next_token();

        left.into_int() + right.into_int()
    }
}

impl Token {
    pub fn into_int(self) -> i64 {
        if let Token::INT(value) = self {
            return value;
        }

        panic!("Value not integer.");
    }
}
#[derive(Debug, Copy, Clone)]
enum Token {
    INT(i64),
    OPERATION(Operation),
    NONE,
}
#[derive(Debug, Copy, Clone)]
enum Operation {
    PLUS,
}
