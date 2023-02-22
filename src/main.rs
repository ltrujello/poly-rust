extern crate log;

use poly::parser::{Parser, ParserErr};
use std::io;
use std::io::Write;

use poly::monomial_generic::Monomial;
use poly::monomial_generic::Monomial64;
use poly::polynomial_generic::Polynomial;
use poly::polynomial_generic::Polynomial64;

fn main() {
    env_logger::init();
    println!("\x1B[36m    ______\n   //   //   ____   //   \\\\ //\n  //___//  //  //  //     \\\\/\n //       //__//  //__    //\n//                       //\x1B[0m");
    loop {
        print!("~> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input_copy: String = input.clone();
        let parser_res = Parser::parser_init(input);
        match parser_res {
            Ok(mut parser) => {
                let res = parser.start_parser();
                match res {
                    Ok(v) => {
                        println!("{}", v);
                    }
                    Err(e) => {
                        handle_parser_error(input_copy, parser.lexer.curr_pos, e);
                    }
                }
            }
            Err(e) => {
                handle_parser_error(input_copy, 0, e);
            }
        }
    }
}

pub fn handle_parser_error(offending_line: String, curr_pos: usize, parser_res: ParserErr) -> bool {
    match parser_res {
        ParserErr::ExpectedToken(msg) => {
            print_syntax_error(offending_line, curr_pos, &msg);
        }
        ParserErr::UnexpectedToken(msg) => {
            print_syntax_error(offending_line, curr_pos, &msg);
        }
        ParserErr::LexerErr(msg) => {
            print_syntax_error(offending_line, curr_pos, &msg);
        }
        ParserErr::InvalidSyntax(msg) => {
            print_syntax_error(offending_line, curr_pos, &msg);
        }
    }
    false
}

fn print_syntax_error(offending_line: String, curr_pos: usize, msg: &str) {
    print!("  {}", offending_line);
    io::stdout().flush().unwrap();
    if curr_pos > 0 {
        println!("  {: <1$}^", "", curr_pos - 1);
    } else {
        println!("  {: <1$}^", "", curr_pos);
    }
    println!("\x1B[31mSyntaxError: {}\x1B[0m", msg);
}
