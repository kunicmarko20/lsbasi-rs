use super::token::{Token, Operation};

pub struct Interpreter {
    input: String,
    position: usize,
}

impl Interpreter {
    pub fn new(input: String) -> Self {
        Interpreter {
            input,
            position: 0,
        }
    }

    pub fn expr(&mut self) -> Result<i64, String> {
        let left = self.next_token()?;
        let operation = self.next_token()?;
        let right = self.next_token()?;

        if let Token::OPERATION(operation) = operation {
            match operation {
                Operation::ADDITION => return Ok(left.into_int()? + right.into_int()?),
                Operation::SUBTRACTION => return Ok(left.into_int()? - right.into_int()?),
                Operation::MULTIPLICATION => return Ok(left.into_int()? * right.into_int()?),
                Operation::DIVISION => return Ok(left.into_int()? / right.into_int()?),
            }
        }

        return Err(String::from("Expected operation between numbers."));
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.has_more_characters() {
            self.decrement();
            return Ok(Token::NONE);
        }

        let current_character = self.current_character();

        self.increment();

        if let Ok(current_number) = current_character.parse::<i64>() {
            return Ok(self.extract_current_number(current_number));
        }

        match current_character.as_ref() {
            "+" => {
                return Ok(Token::OPERATION(Operation::ADDITION));
            },
            "-" => {
                return Ok(Token::OPERATION(Operation::SUBTRACTION));
            },
            "*" => {
                return Ok(Token::OPERATION(Operation::MULTIPLICATION));
            },
            "/" => {
                return Ok(Token::OPERATION(Operation::DIVISION));
            },
            " " => return self.next_token(),
            _ => return Err(format!("Wrong Character: {}", current_character))
        }
    }

    fn has_more_characters(&self) -> bool {
        self.position >= self.input.len()
    }

    fn decrement(&mut self) {
        self.position -= 1;
    }

    fn increment(&mut self) {
        self.position += 1;
    }

    fn current_character(&self) -> String {
        self.input.chars().skip(self.position).take(1).collect()
    }

    fn extract_current_number(&mut self, current_number: i64) -> Token {
        let token = Token::INT(current_number);

        let next_token = self.next_token();

        if let Err(_) = next_token {
            self.decrement();
            return token;
        }

        if let Token::INT(value) = next_token.unwrap() {
            return Token::INT(format!("{}{}", current_number, value).parse::<i64>().unwrap());
        }

        self.decrement();

        token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn addition() {
        assert_eq!(Interpreter::new(String::from("1+2")).expr(), Ok(3));
    }

    #[test]
    fn subtraction() {
        assert_eq!(Interpreter::new(String::from("5-2")).expr(), Ok(3));
    }

    #[test]
    fn multiplication() {
        assert_eq!(Interpreter::new(String::from("5*2")).expr(), Ok(10));
    }

    #[test]
    fn division() {
        assert_eq!(Interpreter::new(String::from("10/2")).expr(), Ok(5));
        assert_eq!(Interpreter::new(String::from("33/2")).expr(), Ok(16));
    }

    #[test]
    fn letter_instead_of_integer() {
        assert_eq!(Interpreter::new(String::from("1+b")).expr(), Err(String::from("Wrong Character: b")));
    }

    #[test]
    fn operation_instead_of_integer() {
        assert_eq!(Interpreter::new(String::from("1++")).expr(), Err(String::from("Value not integer.")));
    }

    #[test]
    fn unknown_operation() {
        assert_eq!(Interpreter::new(String::from("1%3")).expr(), Err(String::from("Wrong Character: %")));
    }

    #[test]
    fn bigger_numbers_are_allowed() {
        assert_eq!(Interpreter::new(String::from("55+31")).expr(), Ok(86));
        assert_eq!(Interpreter::new(String::from("155-55")).expr(), Ok(100));
        assert_eq!(Interpreter::new(String::from("1221+116")).expr(), Ok(1337));
    }

    #[test]
    fn operator_is_required() {
        assert_eq!(Interpreter::new(String::from("123")).expr(), Err(String::from("Expected operation between numbers.")));
    }

}
