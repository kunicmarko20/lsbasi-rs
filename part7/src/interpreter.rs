use super::ast::AST;
use super::parser::Parser;
use super::token::Token;

pub struct Interpreter {
    parser: Parser,
}

impl Interpreter {
    pub fn new(parser: Parser) -> Interpreter {
        Interpreter {
            parser,
        }
    }

    fn visit_num(&self, node: &AST) -> u32 {
        if let Token::Integer(i) = node.token() {
            return *i;
        }

        panic!("Visiting number but value isn't a number.");
    }

    fn visit_binop(&self, node: &AST) -> u32 {
        let left = self.visit(&node.left());
        let right = self.visit(&node.right());

        match node.token() {
            Token::Plus => {
                return left + right;
            },
            Token::Minus => {
                return left - right;
            },
            Token::Mul => {
                return left * right;
            },
            Token::Div => {
                return left / right;
            },
            _ => panic!("Visiting operation but value isn't an operation."),
        }
    }

    fn visit(&self, node: &AST) -> u32 {
        match node.token() {
            Token::Integer(_) => {
                return self.visit_num(node);
            }
            Token::Plus | Token::Minus | Token::Mul | Token::Div => {
                return self.visit_binop(node);
            },
            _ => panic!("Unknown token."),
        }
    }

    pub fn interpret(&mut self) -> u32 {
        let tree = self.parser.parse();
        let result = self.visit(&tree);

        result
    }
}
