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
    position: i64,
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
        if &self.position >= &(&self.text_length() - 1) {
            self.current_token = Some(Token {
                token_value: TokenValue::NONE,
                token_type: TokenType::EOF,
            });
            return ();
        }

        let current_char: String = self
            .input
            .chars()
            .skip(self.position as usize)
            .take(1)
            .collect();

        self.position += 1;

        if let Ok(current_char_as_int) = current_char.parse::<i64>() {
            self.current_token = Some(Token {
                token_value: TokenValue::INT(current_char_as_int),
                token_type: TokenType::INT,
            });
            return ();
        }

        if current_char == "+" {
            self.current_token = Some(Token {
                token_value: TokenValue::OPERATION(Operation::PLUS),
                token_type: TokenType::OPERATION,
            });
            return ();
        }

        panic!("Wrong Character");
    }

    fn text_length(&self) -> i64 {
        *&self.input.len() as i64
    }

    fn eat(&mut self, token_type: TokenType) {
        let current_type = self.current_token.take().unwrap();

        if current_type.is_equal(&token_type) {
            self.next_token();
            return ();
        }

        panic!("Wrong type");
    }

    pub fn expr(&mut self) -> i64 {
        self.next_token();

        let left = self.current_token.clone().unwrap();
        self.eat(TokenType::INT);

        let _operation = self.current_token.clone().unwrap();
        self.eat(TokenType::OPERATION);

        let right = self.current_token.clone().unwrap();
        self.eat(TokenType::INT);

        left.into_int() + right.into_int()
    }
}

#[derive(Debug, Copy, Clone)]
struct Token {
    token_value: TokenValue,
    token_type: TokenType,
}

impl Token {
    pub fn is_equal(&self, token_type: &TokenType) -> bool {
        std::mem::discriminant(&self.token_type) == std::mem::discriminant(token_type)
    }

    pub fn into_int(self) -> i64 {
        if let TokenValue::INT(value) = self.token_value {
            return value;
        }

        panic!("Value not integer.");
    }
}
#[derive(Debug, Copy, Clone)]
enum TokenType {
    INT,
    OPERATION,
    EOF,
}
#[derive(Debug, Copy, Clone)]
enum TokenValue {
    INT(i64),
    OPERATION(Operation),
    NONE,
}
#[derive(Debug, Copy, Clone)]
enum Operation {
    PLUS,
}
