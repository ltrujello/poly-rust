mod lexer;
mod monomial;
mod parser;
mod polynomial;

pub use crate::lexer::{tokenize, Lexer, TokType, Token};
pub use crate::monomial::Monomial;
pub use crate::parser::Parser;
pub use crate::polynomial::Polynomial;
use std::io;

fn main() {
    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    let mut buffer = String::from("xyz\n");
    let mut lexer = Lexer {
        current_line: buffer.clone(),
        line_size: buffer.len(),
        curr_pos: 0,
        curr_tok: Token {
            token_type: TokType::End,
            token_content: String::from(""),
        },
    };

    tokenize(lexer)
    // let mut parser = Parser::parser_init(String::from("x^3\n"));
    // let monomial = parser.parse_monomial();
    // println!("{}", monomial);
}
