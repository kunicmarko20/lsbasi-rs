use lexer::Lexer;
use parser::Parser;
use interpreter::Interpreter;
use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut input = String::new();

        let _ = io::stdout().write(b"spi> ");
        let _ = io::stdout().flush();

        io::stdin().read_line(&mut input).unwrap();

        let text = String::from(input.trim());
        let lexer = Lexer::new(text);
        let parser = Parser::new(lexer);

        let mut interpreter = Interpreter::new(parser);
        let result = interpreter.interpret();
        println!("{}", result);
    }
}

mod ast;
mod token;
mod lexer;
mod parser;
mod interpreter;
