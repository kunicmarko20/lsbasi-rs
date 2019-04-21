use super::lexer::Lexer;
use super::token::Token;
use super::ast::AST;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        Parser{current_token: (&mut lexer).get_next_token(), lexer}
    }

    pub fn eat(&mut self) {
        self.current_token = self.lexer.get_next_token();
    }

    pub fn factor(&mut self) -> AST {
        match self.current_token {
            Token::Integer(value) => {
                self.eat();
                return AST::new_leaf(Token::Integer(value));
            },
            Token::LParen => {
                self.eat();
                let node = self.expr();
                self.eat();
                return node;
            }
            _ => panic!("Factor failed")
        }

    }

    pub fn term(&mut self) -> AST {
        let mut result = self.factor();

        loop {
            if let Token::Mul = self.current_token {
                self.eat();
                result = AST::new(Token::Mul, vec![result, self.factor()]);
                continue;
            }

            if let Token::Div = self.current_token {
                self.eat();
                result = AST::new(Token::Div, vec![result, self.factor()]);
                continue;
            }

            break;
        }

        result
    }

    pub fn expr(&mut self) -> AST {
        let mut result = self.term();

        loop {
            if let Token::Plus = self.current_token {
                self.eat();
                result = AST::new(Token::Plus, vec![result, self.term()]);
                continue;
            }

            if let Token::Minus = self.current_token {
                self.eat();
                result = AST::new(Token::Minus, vec![result, self.term()]);
                continue;
            }

            break;
        }

        result
    }

    pub fn parse(&mut self) -> AST {
        self.expr()
    }
}