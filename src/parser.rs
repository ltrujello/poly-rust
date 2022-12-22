use crate::lexer::{Lexer, TokType};
use crate::monomial::Monomial;
use crate::polynomial::Polynomial;
use std::time::Instant;

pub struct Parser {
    lexer: Lexer,
}

#[derive(Debug)]
pub enum ParserErr {
    ExpectedToken,
    UnexpectedToken,
}

impl Parser {
    pub fn parser_init(current_line: String) -> Self {
        let lexer = Lexer::lexer_init(current_line);
        let mut parser = Parser { lexer: lexer };
        parser.lexer.get_next_token().unwrap();
        parser
    }

    pub fn parse_polynomial(&mut self) -> Result<Polynomial, ParserErr> {
        let now = Instant::now();
        let mut polynomial = Polynomial::new();
        loop {
            match self.lexer.curr_tok.token_type {
                TokType::Number | TokType::Xvar => {
                    let monomial_res = self.parse_monomial()?;
                    polynomial += monomial_res;
                }
                TokType::Plus | TokType::Minus => self.lexer.get_next_token().unwrap(),
                _ => break,
            }
        }
        let elapsed = now.elapsed();
        info!("Parsed {:?} in {:.5?}", self.lexer.current_line, elapsed);

        Ok(polynomial)
    }

    pub fn parse_monomial(&mut self) -> Result<Monomial, ParserErr> {
        let now = Instant::now();
        let mut coefficient = 1.0;
        if self.lexer.curr_tok.token_type == TokType::Number {
            coefficient = self.lexer.curr_tok.token_content.parse::<f64>().unwrap();
            self.lexer.get_next_token().unwrap();
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
                        return Err(ParserErr::UnexpectedToken);
                    }
                }
                self.lexer.get_next_token().unwrap();
                let exponent: i32;
                // get caret
                if self.lexer.curr_tok.token_type == TokType::Caret {
                    self.lexer.get_next_token().unwrap();
                    // get number
                    if self.lexer.curr_tok.token_type != TokType::Number {
                        error!("Expected TokType::Number after ^");
                        return Err(ParserErr::ExpectedToken);
                    }
                    exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                    self.lexer.get_next_token().unwrap();
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
        debug!("Parsed {:?} in {:.5?}", self.lexer.current_line, elapsed);
        Ok(Monomial {
            coefficient,
            power_list,
        })
    }

    pub fn parse_factor_expr(&mut self) -> Result<Polynomial, ParserErr> {
        let polynomial: Result<Polynomial, ParserErr>;
        match self.lexer.curr_tok.token_type {
            TokType::Lpar => {
                self.lexer.get_next_token().unwrap();
                polynomial = self.parse_poly_expr();

                if self.lexer.curr_tok.token_type == TokType::Rpar {
                    self.lexer.get_next_token().unwrap();
                } else {
                    error!("Expected closing parenthesis at end of expression");
                    return Err(ParserErr::ExpectedToken);
                }
            }
            TokType::Minus => {
                self.lexer.get_next_token().unwrap();
                if self.lexer.curr_tok.token_type == TokType::Lpar {
                    polynomial = self.parse_factor_expr();
                } else {
                    polynomial = self.parse_polynomial();
                }
            }
            _ => {
                polynomial = self.parse_polynomial();
            }
        }

        polynomial
    }

    pub fn parse_term_expr(&mut self) -> Result<Polynomial, ParserErr> {
        let polynomial = self.parse_factor_expr()?;
        match self.lexer.curr_tok.token_type {
            TokType::Mul => {
                self.lexer.get_next_token().unwrap();
                let other = self.parse_factor_expr()?;
                return Ok(polynomial * other);
            }
            _ => {
                return Ok(polynomial);
            }
        }
    }

    pub fn parse_poly_expr(&mut self) -> Result<Polynomial, ParserErr> {
        let polynomial = self.parse_term_expr();

        polynomial
    }

    pub fn start_parser(&mut self) -> Result<Polynomial, ParserErr> {
        let now = Instant::now();
        let polynomial = self.parse_poly_expr();

        let elapsed = now.elapsed();
        info!("Parsed {:?} in {:.5?}", self.lexer.current_line, elapsed);

        polynomial
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
        assert_eq!(monomial.power(0), 1);
        assert_eq!(monomial.degree(), 1);

        parser = Parser::parser_init(String::from("y\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.degree(), 1);

        parser = Parser::parser_init(String::from("z\n"));
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(2), 1);
        assert_eq!(monomial.degree(), 1);
    }

    #[rstest]
    fn parser_monomial_multivariate() {
        let mut parser = Parser::parser_init(String::from("xyz\n"));
        let monomial = parser.parse_monomial().unwrap();
        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(0), 1);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.power(2), 1);
        assert_eq!(monomial.degree(), 3);
    }

    #[rstest]
    fn parser_monomial_multivariate_2() {
        let mut parser = Parser::parser_init(String::from("3.5x^2yz^5\n"));
        let monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 3.5);
        assert_eq!(monomial.degree(), 8);
        assert_eq!(monomial.power(0), 2);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.power(2), 5);
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

    #[rstest]
    fn parse_polynomial_expr_mulitplication_a() {
        let mut parser = Parser::parser_init(String::from("(x + y) * (x + y)"));
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "y^2 + 2xy + x^2"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_b() {
        let mut parser = Parser::parser_init(String::from("(((x + y) * (x + y))) * (x + y)"));
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "y^3 + 3xy^2 + 3x^2y + x^3"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_c() {
        let mut parser = Parser::parser_init(String::from("(x^4 + 1) * ((x^3 + 2x) * (x + 1))"));
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "2x + 2x^2 + x^3 + x^4 + 2x^5 + 2x^6 + x^7 + x^8"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_d() {
        let mut parser = Parser::parser_init(String::from(
            "(((x + y + z)*(x + y + z))*(x + y + z))*(x + y + z)",
        ));
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "z^4 + 4yz^3 + 4xz^3 + 6y^2z^2 + 12xyz^2 + 6x^2z^2 + 4y^3z + 12xy^2z + 12x^2yz + 4x^3z + y^4 + 4xy^3 + 6x^2y^2 + 4x^3y + x^4"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_parentheses() {
        let mut parser = Parser::parser_init(String::from("(((x + y)))"));
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "y + x"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    // Valid expressions
    // (x^4 + 1) * ((x^3 + 2x) * (x + 1))
    // (x^4 + 1) * (x^3)
    // ((x^4 + 1))
    // x^3 * x + x^4 + x^2
    // x * (x - 8)^2 * (x - 9)
}
