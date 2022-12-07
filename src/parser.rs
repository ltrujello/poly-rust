use crate::monomial::Monomial;
use crate::lexer::{
    Lexer,
    Token,
    TokType,
};

pub struct Parser {
    lexer: Lexer
}

impl Parser{
    pub fn parser_init(current_line: String) -> Self {
        let lexer = Lexer {
            current_line: current_line.clone(),
            line_size: current_line.len(),
            curr_pos: 0,
            curr_tok: Token {
                token_type: TokType::End,
                token_content: String::from(""),
            },
        };
        Parser {
            lexer : lexer
        }
    }


    pub fn parse_monomial(&mut self) -> Monomial {
        self.lexer.get_next_token();

        let coefficient = 1.0;
        let mut power_list = vec![0; 3];
        while true {
            // get xvar
            let prev_position = self.lexer.curr_pos;

            if self.lexer.curr_tok.token_type == TokType::Xvar {
                let mut ind: usize = 0;
                if self.lexer.curr_tok.token_content == "x" {
                    ind = 0;
                }

                if self.lexer.curr_tok.token_content == "y" {
                    ind = 1;
                }
                
                if self.lexer.curr_tok.token_content == "z" {
                    ind = 2;
                }

                self.lexer.get_next_token();
                let exponent: i32;
                // get caret
                if self.lexer.curr_tok.token_type == TokType::Caret {
                    self.lexer.get_next_token();
                    // get number
                    if self.lexer.curr_tok.token_type != TokType::Number {
                    }
                    exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                    self.lexer.get_next_token();
                }
                else {
                    exponent = 1;
                }
                power_list[ind] = exponent;
            }
            if prev_position != self.lexer.curr_pos {
                break;
            }
        }
        Monomial {
            coefficient,
            power_list,
        }
    }

}
