use crate::lexer::{Lexer, TokType};
use crate::monomial::Monomial;
use crate::polynomial::Polynomial;
use std::time::Instant;

pub struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub fn parser_init(current_line: String) -> Self {
        let lexer = Lexer::lexer_init(current_line);
        let mut parser = Parser { lexer: lexer };
        parser.lexer.get_next_token();
        parser
    }

    pub fn parse_polynomial(&mut self) -> Result<Polynomial, String> {
        let now = Instant::now();
        let mut polynomial = Polynomial::new();
        loop {
            match self.lexer.curr_tok.token_type {
                TokType::Number | TokType::Xvar => {
                    let monomial_res = self.parse_monomial()?;
                    polynomial += monomial_res;
                }
                TokType::Plus | TokType::Minus => self.lexer.get_next_token(),
                _ => break,
            }
        }
        let elapsed = now.elapsed();
        info!("Parsed {} in {:.5?}", self.lexer.current_line, elapsed);

        Ok(polynomial)
    }

    pub fn parse_monomial(&mut self) -> Result<Monomial, String> {
        let now = Instant::now();
        let mut coefficient = 1.0;
        if self.lexer.curr_tok.token_type == TokType::Number {
            coefficient = self.lexer.curr_tok.token_content.parse::<f64>().unwrap();
            self.lexer.get_next_token();
        }
        let mut power_list = vec![0; 3];
        loop {
            let prev_position = self.lexer.curr_pos;

            // get xvar
            if self.lexer.curr_tok.token_type == TokType::Xvar {
                let ind: usize;
                match self.lexer.curr_tok.token_content.as_str() {
                    "x" => ind = 0,
                    "y" => ind = 1,
                    "z" => ind = 2,
                    _ => {
                        error!(
                            "Received unknown token content {}",
                            self.lexer.curr_tok.token_content
                        );
                        return Err(String::from("Received unknown token content"));
                    }
                }
                self.lexer.get_next_token();
                let exponent: i32;
                // get caret
                if self.lexer.curr_tok.token_type == TokType::Caret {
                    self.lexer.get_next_token();
                    // get number
                    if self.lexer.curr_tok.token_type != TokType::Number {
                        return Err(String::from("Expected TokType::Number after ^"));
                    }
                    exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                    self.lexer.get_next_token();
                } else {
                    exponent = 1;
                }
                power_list[ind] = exponent;
            }

            if prev_position == self.lexer.curr_pos {
                break;
            }
        }
        let elapsed = now.elapsed();
        debug!("Parsed {} in {:.5?}", self.lexer.current_line, elapsed);
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

    #[rstest]
    fn parser_monomial_degree_one() {
        let mut parser = Parser::parser_init(String::from("x\n"));
        let mut monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_power(0), 1);
        assert_eq!(monomial.get_degree(), 1);

        parser = Parser::parser_init(String::from("y\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_power(1), 1);
        assert_eq!(monomial.get_degree(), 1);

        parser = Parser::parser_init(String::from("z\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_power(2), 1);
        assert_eq!(monomial.get_degree(), 1);
    }

    #[rstest]
    fn parser_monomial_multivariate() {
        let mut parser = Parser::parser_init(String::from("xyz\n"));
        let monomial = parser.parse_monomial().unwrap();
        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.get_power(0), 1);
        assert_eq!(monomial.get_power(1), 1);
        assert_eq!(monomial.get_power(2), 1);
        assert_eq!(monomial.get_degree(), 3);
    }

    #[rstest]
    fn parser_monomial_multivariate_2() {
        let mut parser = Parser::parser_init(String::from("3.5x^2yz^5\n"));
        let monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 3.5);
        assert_eq!(monomial.get_degree(), 8);
        assert_eq!(monomial.get_power(0), 2);
        assert_eq!(monomial.get_power(1), 1);
        assert_eq!(monomial.get_power(2), 5);
    }

    #[rstest]
    fn parse_polynomial_simple() {
        let mut parser = Parser::parser_init(String::from("2x + y + z + 2x + y + y + y + z\n"));
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
    }

    #[rstest]
    fn parse_polynomial_multivariate_a() {
        let mut parser = Parser::parser_init(String::from("2xyz + yzx + zxy + xy \n"));
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[1].coefficient, 4.0);
    }

    #[rstest]
    fn parse_polynomial_multivariate_b() {
        let mut parser =
            Parser::parser_init(String::from("2xyz + zyx+ zy + 2x + zy + x + yzx + yz\n"));
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "3x");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "3yz");
        assert_eq!(polynomial.monomials[2].expr().as_str(), "4xyz");
    }

    #[rstest]
    fn parse_polynomial_numbers_only() {
        let mut parser = Parser::parser_init(String::from("2 + 3 + 4.5\n"));
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 1);
        assert_eq!(polynomial.monomials[0].coefficient, 9.5);
    }
}
