use std::io;
use interpreter::Interpreter;

fn main() {
    println!("Enter expression:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");

    let mut interpreter = Interpreter::new(input.trim().to_string());

    match interpreter.expr() {
        Ok(value) => println!("{}", value),
        Err(error) => {
            println!("{} Try again.", error);
            main();
        }
    }
}

mod interpreter;
mod token;
