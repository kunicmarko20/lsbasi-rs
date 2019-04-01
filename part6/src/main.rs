use std::io;
use interpreter::Interpreter;
use lexer::Lexer;

fn main() {
    println!("Enter expression:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let mut interpreter = Interpreter::new(
        Lexer::new(
            input.trim().to_string()
        )
    );

    println!("{}", interpreter.expr());
}

mod token;
mod lexer;
mod interpreter;
