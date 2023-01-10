use crate::parser::Parser;
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
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => {
                println!("{}", v.expr());
            }
            Err(e) => (),
        }
    }
}
