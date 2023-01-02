#[macro_use]
extern crate log;

mod lexer;
mod monomial;
mod parser;
mod polynomial;

pub use crate::lexer::{tokenize, Lexer, TokType, Token};
pub use crate::monomial::Monomial;
pub use crate::parser::Parser;
pub use crate::polynomial::Polynomial;

fn main() {
    env_logger::init();
    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    // let mut buffer = String::from("x^3y^2z^50\n\n");
    // let mut lexer = Lexer {
    //     current_line: buffer.clone(),
    //     line_size: buffer.len(),
    //     curr_pos: 0,
    //     curr_tok: Token {
    //         token_type: TokType::End,
    //         token_content: String::from(""),
    //     },
    // };
    // tokenize(lexer)

    // let polynomial = Polynomial::from("2xyz + yzx + zxy + xy").unwrap();
    // println!("{:?}", polynomial);
    // println!("{}", polynomial.expr());

    // let monomial = Monomial::from("2xyz").unwrap();
    // println!("{}", monomial.expr())

    let mut parser = Parser::parser_init(String::from("x^3 + (x + 5)*(x - 7)"));
    let polynomial = parser.start_parser();
    match polynomial {
        Ok(v) => println!("{}", v.expr()),
        Err(e) => println!("{:?}", e),
    }
}
