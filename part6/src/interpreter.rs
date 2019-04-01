use super::lexer::Lexer;
use super::token::Token;

pub struct Interpreter {
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    pub fn new(mut lexer: Lexer) -> Self {
        Interpreter{current_token: (&mut lexer).get_next_token(), lexer}
    }

    pub fn from_input<T: Into<String>>(text: T) -> Self {
        Interpreter::new(Lexer::new(text))
    }

    pub fn eat(&mut self) {
        self.current_token = self.lexer.get_next_token();
    }

    pub fn factor(&mut self) -> u32 {
        if let Token::Integer(value) = self.current_token {
            self.eat();
            return value;
        }

        if let Token::LParen = self.current_token {
            self.eat();
            let result = self.expr();
            self.eat();
            return result;
        }

        panic!("Factor failed");
    }

    pub fn term(&mut self) -> u32 {
        let mut result = self.factor();

        loop {
            if let Token::Mul = self.current_token {
                self.eat();
                result *= self.factor();
                continue;
            }

            if let Token::Div = self.current_token {
                self.eat();
                result /= self.factor();
                continue;
            }

            break;
        }

        result
    }

    pub fn expr(&mut self) -> u32 {
        let mut result = self.term();

        loop {
            dbg!(&result);
            if let Token::Plus = self.current_token {
                self.eat();
                result += self.term();
                continue;
            }

            if let Token::Minus = self.current_token {
                self.eat();
                result -= self.term();
                continue;
            }

            break;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single() {
        assert_eq!(Interpreter::from_input("3").expr(), 3);
    }

    #[test]
    fn plus_and_mul() {
        assert_eq!(Interpreter::from_input("2+7*4").expr(), 30);
    }

    #[test]
    fn minus_and_div() {
        assert_eq!(Interpreter::from_input("7-8/4").expr(), 5);
    }

    #[test]
    fn plus_mul_minus_div() {
        assert_eq!(Interpreter::from_input("14 + 2 * 3 - 6 / 2").expr(), 17);
    }

    #[test]
    fn with_parentheses() {
        assert_eq!(Interpreter::from_input("7 + 3 * (10 / (12 / (3 + 1) - 1))").expr(), 22);
        assert_eq!(Interpreter::from_input("7 + 3 * (10 / (12 / (3 + 1) - 1)) / (2 + 3) - 5 - 3 + (8)").expr(), 10);
        assert_eq!(Interpreter::from_input("7 + (((3 + 2)))").expr(), 12);
    }
}
