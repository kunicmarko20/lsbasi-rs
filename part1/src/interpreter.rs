use super::token::*;

pub struct Interpreter {
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

    fn next_token(&mut self) -> Result<(), String> {
        if self.position >= self.input.len() {
            self.current_token = Some(Token::NONE);
            return Ok(());
        }

        let current_char: String = self.input.chars().skip(self.position).take(1).collect();

        self.position += 1;

        if let Ok(current_char_as_int) = current_char.parse::<i64>() {
            self.current_token = Some(Token::INT(current_char_as_int));
            return Ok(());
        }

        if current_char == "+" {
            self.current_token = Some(Token::OPERATION(Operation::PLUS));
            return Ok(());
        }

        Err(format!("Wrong Character: {}", current_char))
    }

    pub fn expr(&mut self) -> Result<i64, String> {
        self.next_token()?;

        let left = self.current_token.unwrap();
        self.next_token()?;

        if let Token::OPERATION(_) = self.current_token.unwrap() {
            self.next_token()?;
        } else {
            return Err(String::from("Expected operation at second position."));
        }

        let right = self.current_token.unwrap();
        self.next_token()?;

        Ok(left.into_int()? + right.into_int()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Interpreter::new(String::from("1+2")).expr(), Ok(3));
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
        assert_eq!(Interpreter::new(String::from("1-3")).expr(), Err(String::from("Wrong Character: -")));
    }

    #[test]
    #[should_panic]
    fn int_where_operator_should_be_fails() {
        Interpreter::new(String::from("123")).expr().unwrap();
    }

}
