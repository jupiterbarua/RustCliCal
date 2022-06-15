use std::io;
mod parsemath;
use parsemath::ast;
use parsemath::parser::{ParseError, Parser};


fn evalute(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_praser = Parser::new(&expr)?;
    let ast = math_praser.parse()?;
    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Enter your arithmatic expression below");

    loop {
        let mut input = String::new();
        match  io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evalute(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => println!("Please enter valid expression"),
                }
            }
            Err(error) => println!("Error {}", error)
        }
    }
}
