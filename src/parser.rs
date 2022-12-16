use crate::lexer::{Lexer, TokType, Token};
use crate::monomial::Monomial;
use std::time::Instant;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
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
        Parser { lexer: lexer }
    }

    pub fn parse_monomial(&mut self) -> Result<Monomial, String> {
        let now = Instant::now();
        self.lexer.get_next_token();
        let coefficient = 1.0;
        let mut power_list = vec![0; 3];
        loop {
            let prev_position = self.lexer.curr_pos;

            // get xvar
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
                    if self.lexer.curr_tok.token_type != TokType::Number {}
                    exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                    self.lexer.get_next_token();
                } else {
                    exponent = 1;
                }
                power_list[ind] = exponent;
            }
            else {
                if self.lexer.curr_tok.token_type == TokType::Newl {
                    break;
                }
                error!("received unexpected token {:?}", self.lexer.curr_tok.token_type);
                return Err(format!("received unexpected token {:?}", self.lexer.curr_tok.token_type));
            }
            if prev_position == self.lexer.curr_pos {
                break;
            }
        }
        let elapsed = now.elapsed();
        info!("Parsed {} in {:.5?}", self.lexer.current_line, elapsed);
        Ok(Monomial {
            coefficient,
            power_list,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn parser_monomial_degree_one() {
        let mut parser = Parser::parser_init(String::from("x\n"));
        let mut monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_degree(), 1);

        parser = Parser::parser_init(String::from("y\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_degree(), 1);

        parser = Parser::parser_init(String::from("z\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_degree(), 1);
    }

    fn parser_monomial_multivariate() {
        let mut parser = Parser::parser_init(String::from("xyz\n"));
        let mut monomial = parser.parse_monomial().unwrap();
        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_degree(), 3);

    }

    fn parser_monomial_multivariate_2() {
        let mut parser = Parser::parser_init(String::from("3.5x^2yz^5\n"));
        let mut monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 3.5);
        assert_eq!(monomial.get_degree(), 8);
        assert_eq!(monomial.get_power(0), 2);
        assert_eq!(monomial.get_power(1), 1);
        assert_eq!(monomial.get_power(2), 5);
    }
}
