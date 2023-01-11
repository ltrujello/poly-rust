use crate::parser::{Parser, ParserErr};
use std::io;
use std::io::Write;

pub fn run_cli() {
    println!("\x1B[36m    ______\n   //   //   ____   //   \\\\ //\n  //___//  //  //  //     \\\\/\n //       //__//  //__    //\n//                       //\x1B[0m");

    loop {
        print!("~> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);

        let mut parser = Parser::parser_init(buffer);
        let res = parser.start_parser();
        match res {
            Ok(v) => {
                println!("{}", v.expr());
            }
            Err(e) => {
                let offending_line: String = parser.lexer.current_line.iter().collect();
                handle_parser_error(offending_line, parser.lexer.curr_pos, e);
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
    println!("  {: <1$}^", "", curr_pos);
    println!("\x1B[31mSyntaxError: {}\x1B[0m", msg);
}
