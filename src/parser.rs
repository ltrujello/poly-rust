use crate::lexer::{Lexer, TokType, Token};
use crate::monomial_generic::Monomial64;
use crate::polynomial_generic::Polynomial64;
use log::{debug, error, info};
use std::time::Instant;

pub struct Parser {
    pub lexer: Lexer,
}

#[derive(Debug, PartialEq)]
pub enum ParserErr {
    ExpectedToken(String),
    UnexpectedToken(String),
    LexerErr(String),
    InvalidSyntax(String),
}

impl Parser {
    pub fn parser_init(current_line: String) -> Result<Self, ParserErr> {
        let lexer = Lexer::lexer_init(current_line);
        let mut parser = Parser { lexer: lexer };
        parser.get_next_token()?;
        Ok(parser)
    }

    pub fn get_next_token(&mut self) -> Result<(), ParserErr> {
        match self.lexer.get_next_token() {
            Ok(()) => (),
            Err(e) => {
                let msg = format!(
                    "Error received while getting next token from lexer: {:?}",
                    e
                );
                error!("{}", msg);
                return Err(ParserErr::LexerErr(msg));
            }
        }
        Ok(())
    }

    pub fn peek_next_token(&mut self) -> Result<Token, ParserErr> {
        match self.lexer.peek_next_token() {
            Ok(v) => {
                return Ok(v);
            }
            Err(e) => {
                let msg = format!(
                    "Error received while peeking next token from lexer: {:?}",
                    e
                );
                error!("{}", msg);
                return Err(ParserErr::LexerErr(msg));
            }
        }
    }

    pub fn parse_monomial(&mut self) -> Result<Monomial64, ParserErr> {
        let now = Instant::now();
        let start_ind = self.lexer.curr_pos;

        // get minus symbol
        let mut coefficient = 1.0;
        if self.lexer.curr_tok.token_type == TokType::Minus {
            coefficient = -1.0;
            self.get_next_token()?;
        }

        // get coefficient
        if self.lexer.curr_tok.token_type == TokType::Number {
            let abs_coeff = self.lexer.curr_tok.token_content.parse::<f64>().unwrap();
            if coefficient > 0.0 {
                coefficient = abs_coeff;
            } else {
                coefficient = -1.0 * abs_coeff;
            }
            self.get_next_token()?;
        }

        let mut power_list = vec![0; 3];
        // A single loop will parse x ^ num
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
                        let msg = format!(
                            "Received unknown token content {}",
                            self.lexer.curr_tok.token_content
                        );
                        error!("{}", msg);
                        return Err(ParserErr::UnexpectedToken(msg));
                    }
                }
                self.get_next_token()?;
                let exponent: i32;
                // get caret
                if self.lexer.curr_tok.token_type == TokType::Caret {
                    self.get_next_token()?;
                    // get number
                    if self.lexer.curr_tok.token_type != TokType::Number {
                        let msg = format!(
                            "Expected number after ^, found {:?}",
                            self.lexer.curr_tok.token_type
                        );
                        error!("{}", msg);
                        return Err(ParserErr::ExpectedToken(msg));
                    }
                    exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                    self.get_next_token()?;
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
        let end_ind = self.lexer.curr_pos;
        debug!(
            "Parsed {:?} in {:.5?}",
            &self.lexer.current_line[start_ind..end_ind],
            elapsed
        );
        Ok(Monomial64 {
            coefficient,
            power_list,
        })
    }

    pub fn parse_polynomial(&mut self) -> Result<Polynomial64, ParserErr> {
        let now = Instant::now();
        let mut polynomial = Polynomial64::new();
        // Get the first term
        match self.lexer.curr_tok.token_type {
            TokType::Number | TokType::Xvar => {
                let monomial_res = self.parse_monomial()?;
                polynomial += monomial_res;
            }
            _ => {
                return Err(ParserErr::InvalidSyntax(String::from("Invalid Syntax")));
            }
        }
        loop {
            match self.lexer.curr_tok.token_type {
                TokType::Minus => {
                    let next_token = self.peek_next_token()?;
                    if next_token.token_type == TokType::Number
                        || next_token.token_type == TokType::Xvar
                    {
                        self.get_next_token()?;
                        let monomial_res = self.parse_monomial()?;
                        polynomial += monomial_res * -1.0;
                    } else {
                        info!(
                            "token is back to MINUS: {:?}",
                            self.lexer.curr_tok.token_type
                        );
                        break;
                    }
                }
                TokType::Plus => {
                    let next_token = self.peek_next_token()?;
                    if next_token.token_type == TokType::Number
                        || next_token.token_type == TokType::Xvar
                    {
                        self.get_next_token()?;
                        let monomial_res = self.parse_monomial()?;
                        polynomial += monomial_res;
                    } else {
                        info!(
                            "token is back to PLUS: {:?}",
                            self.lexer.curr_tok.token_type
                        );
                        break;
                    }
                }
                _ => break,
            }
        }
        let elapsed = now.elapsed();
        let line: String = self.lexer.current_line.iter().collect();
        debug!("Parsed {:?} in {:.5?}", line, elapsed);

        Ok(polynomial)
    }

    pub fn parse_factor_expr(&mut self) -> Result<Polynomial64, ParserErr> {
        info!(
            "parse_factor_expr: recieved token {:?}",
            self.lexer.curr_tok.token_type
        );
        let polynomial: Result<Polynomial64, ParserErr>;
        match self.lexer.curr_tok.token_type {
            TokType::Lpar => {
                self.get_next_token()?;
                let inner = self.parse_poly_expr()?;

                if self.lexer.curr_tok.token_type == TokType::Rpar {
                    self.get_next_token()?;
                } else {
                    let msg = String::from("Expected closing parenthesis at end of expression");
                    error!("{}", msg);
                    return Err(ParserErr::ExpectedToken(msg));
                }
                // Check for exponent on closing parenthesis
                if self.lexer.curr_tok.token_type == TokType::Caret {
                    self.get_next_token()?;
                    if self.lexer.curr_tok.token_type == TokType::Number {
                        self.get_next_token()?;
                        let exponent = self.lexer.curr_tok.token_content.parse::<i32>().unwrap();
                        polynomial = Ok(inner.pow(exponent));
                    } else {
                        let msg = format!(
                            "Expected number after caret, received {:?}",
                            self.lexer.curr_tok.token_type
                        );
                        error!("{}", msg);
                        return Err(ParserErr::ExpectedToken(msg));
                    }
                } else {
                    polynomial = Ok(inner);
                }
            }
            TokType::Minus => {
                self.get_next_token()?;
                if self.lexer.curr_tok.token_type == TokType::Lpar {
                    let mut inner = self.parse_factor_expr()?;
                    inner.scale(-1.0);
                    polynomial = Ok(inner);
                } else {
                    let msg = format!(
                        "Unexpected token received {:?}",
                        self.lexer.curr_tok.token_type
                    );
                    error!("{}", msg);
                    return Err(ParserErr::UnexpectedToken(msg));
                }
            }
            _ => {
                polynomial = self.parse_polynomial();
            }
        }

        polynomial
    }

    pub fn parse_term_expr(&mut self) -> Result<Polynomial64, ParserErr> {
        info!(
            "parse_term_expr: recieved token {:?}",
            self.lexer.curr_tok.token_type
        );
        let polynomial = self.parse_factor_expr()?;
        match self.lexer.curr_tok.token_type {
            TokType::Mul => {
                self.get_next_token()?;
                let other = self.parse_factor_expr()?;
                let mut mul = polynomial * other;

                while self.lexer.curr_tok.token_type == TokType::Mul {
                    self.get_next_token()?;
                    let other = self.parse_factor_expr()?;
                    mul = mul * other;
                }

                return Ok(mul);
            }
            _ => {
                return Ok(polynomial);
            }
        }
    }

    pub fn parse_poly_expr(&mut self) -> Result<Polynomial64, ParserErr> {
        info!(
            "parse_poly_expr: recieved token {:?}",
            self.lexer.curr_tok.token_type
        );
        let polynomial = self.parse_term_expr()?;
        match self.lexer.curr_tok.token_type {
            TokType::Plus | TokType::Minus => {
                let mut sum = polynomial;
                loop {
                    match self.lexer.curr_tok.token_type {
                        TokType::Plus => {
                            self.get_next_token()?;
                            let other = self.parse_term_expr()?;
                            sum = sum + other
                        }
                        TokType::Minus => {
                            self.get_next_token()?;
                            let other = self.parse_term_expr()?;
                            sum = sum - other
                        }
                        _ => break,
                    }
                }
                return Ok(sum);
            }
            _ => {
                return Ok(polynomial);
            }
        }
    }

    pub fn start_parser(&mut self) -> Result<Polynomial64, ParserErr> {
        let now = Instant::now();
        // Check for empty input
        while self.lexer.curr_tok.token_type == TokType::Newl {
            self.get_next_token()?
        }
        if self.lexer.curr_tok.token_type == TokType::End {
            return Ok(Polynomial64::new());
        }

        let mut parser_res = self.parse_poly_expr();
        let elapsed = now.elapsed();

        let line: String = self.lexer.current_line.iter().collect();
        info!("Parsed {:?} in {:.5?}", line, elapsed);

        if self.lexer.curr_tok.token_type != TokType::End
            && self.lexer.curr_tok.token_type != TokType::Newl
        {
            error!("{:?}", self.lexer.curr_tok.token_type);
            parser_res = Err(ParserErr::InvalidSyntax("Invalid syntax".to_string()));
        }
        parser_res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::mem;

    #[rstest]
    fn parser_monomial_degree_one() {
        let mut parser = Parser::parser_init(String::from("x\n")).unwrap();
        let mut monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(0), 1);
        assert_eq!(monomial.degree(), 1);

        parser = Parser::parser_init(String::from("y\n")).unwrap();
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.degree(), 1);

        parser = Parser::parser_init(String::from("z\n")).unwrap();
        monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(2), 1);
        assert_eq!(monomial.degree(), 1);
    }

    #[rstest]
    fn parser_monomial_multivariate() {
        let mut parser = Parser::parser_init(String::from("xyz\n")).unwrap();
        let monomial = parser.parse_monomial().unwrap();
        assert_eq!(monomial.coefficient, 1.0);
        assert_eq!(monomial.power(0), 1);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.power(2), 1);
        assert_eq!(monomial.degree(), 3);
    }

    #[rstest]
    fn parser_monomial_multivariate_2() {
        let mut parser = Parser::parser_init(String::from("3.5x^2yz^5\n")).unwrap();
        let monomial = parser.parse_monomial().unwrap();

        assert_eq!(monomial.coefficient, 3.5);
        assert_eq!(monomial.degree(), 8);
        assert_eq!(monomial.power(0), 2);
        assert_eq!(monomial.power(1), 1);
        assert_eq!(monomial.power(2), 5);
    }

    #[rstest]
    fn parse_polynomial_simple() {
        let mut parser =
            Parser::parser_init(String::from("2x + y + z + 2x + y + y + y + z\n")).unwrap();
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
    }

    #[rstest]
    fn parse_polynomial_multivariate_a() {
        let mut parser = Parser::parser_init(String::from("2xyz + yzx + zxy + xy \n")).unwrap();
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
    }

    #[rstest]
    fn parse_polynomial_multivariate_b() {
        let mut parser =
            Parser::parser_init(String::from("2xyz + zyx+ zy + 2x + zy + x + yzx + yz\n")).unwrap();
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "4xyz");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "3yz");
        assert_eq!(polynomial.monomials[2].expr().as_str(), "3x");
    }

    #[rstest]
    fn parse_polynomial_numbers_only() {
        let mut parser = Parser::parser_init(String::from("2 + 3 + 4.5\n")).unwrap();
        let polynomial = parser.parse_polynomial().unwrap();

        assert_eq!(polynomial.monomials.len(), 1);
        assert_eq!(polynomial.monomials[0].coefficient, 9.5);
    }

    #[rstest]
    fn parse_polynomial_error_a() {
        let mut parser = Parser::parser_init(String::from("+ y + z")).unwrap();
        let res = parser.parse_polynomial();
        match res {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(
                mem::discriminant(&e),
                mem::discriminant(&ParserErr::InvalidSyntax(String::from("")))
            ),
        }
    }

    #[rstest]
    fn parse_polynomial_error_b() {
        let mut parser = Parser::parser_init(String::from("- y + z")).unwrap();
        let res = parser.parse_polynomial();
        match res {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(
                mem::discriminant(&e),
                mem::discriminant(&ParserErr::InvalidSyntax(String::from("")))
            ),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_a() {
        let mut parser = Parser::parser_init(String::from("(x + y) * (x + y)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^2 + 2xy + y^2"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_b() {
        let mut parser =
            Parser::parser_init(String::from("(((x + y) * (x + y))) * (x + y)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 3x^2y + 3xy^2 + y^3"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_c() {
        let mut parser =
            Parser::parser_init(String::from("(x^4 + 1) * ((x^3 + 2x) * (x + 1))")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^8 + x^7 + 2x^6 + 2x^5 + x^4 + x^3 + 2x^2 + 2x"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_d() {
        let mut parser = Parser::parser_init(String::from(
            "(((x + y + z)*(x + y + z))*(x + y + z))*(x + y + z)",
        ))
        .unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^4 + 4x^3y + 4x^3z + 6x^2y^2 + 12x^2yz + 6x^2z^2 + 4xy^3 + 12xy^2z + 12xyz^2 + 4xz^3 + y^4 + 4y^3z + 6y^2z^2 + 4yz^3 + z^4"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_negative_a() {
        let mut parser =
            Parser::parser_init(String::from("((x - y) * (x + y)) * (x + y)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + x^2y - xy^2 - y^3"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_mulitplication_negative_b() {
        let mut parser =
            Parser::parser_init(String::from("(((x - y) * (x + y)) * (x + y)) * (x + y)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^4 + 2x^3y - 2xy^3 - y^4"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_expr_parentheses() {
        let mut parser = Parser::parser_init(String::from("(((x + y)))")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x + y"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_addition_and_multiplication() {
        let mut parser = Parser::parser_init(String::from("x^3 + x^2 + (x + 5)*(x - 7)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 2x^2 - 2x - 35"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_subtraction_and_multiplication_a() {
        let mut parser = Parser::parser_init(String::from("x^3 - x^2 + (x + 5)*(x - 7)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 - 2x - 35"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_subtraction_and_multiplication_b() {
        let mut parser = Parser::parser_init(String::from("x^3 - x^2 - (x + 5)*(x - 7)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => {
                println!("{:?}", v);
                assert_eq!(v.expr(), "x^3 - 2x^2 + 2x + 35");
            }
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_subtraction_and_multiplication_c() {
        let mut parser = Parser::parser_init(String::from("x^2 - (x + 5)*(x - 7)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => {
                println!("{:?}", v);
                assert_eq!(v.expr(), "2x + 35");
            }
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_linear_factors() {
        let mut parser = Parser::parser_init(String::from("(x + 5)*(x - 7)*(x - 4)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 - 6x^2 - 27x + 140"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_a() {
        let mut parser =
            Parser::parser_init(String::from("x^4 + x^3 + (x + 5)*(x - 7)*(x - 4)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^4 + 2x^3 - 6x^2 - 27x + 140"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_b() {
        let mut parser = Parser::parser_init(String::from(
            "(x + 5)*(x - 7)*(x - 4) + (x + 5)*(x - 7)*(x - 4)",
        ))
        .unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "2x^3 - 12x^2 - 54x + 280"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_quadratic_factor() {
        let mut parser =
            Parser::parser_init(String::from("(2x + 10)*(x - 7)*(x^2 + x + 1)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "2x^4 - 2x^3 - 72x^2 - 74x - 70"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_multivariate_factor_a() {
        let mut parser =
            Parser::parser_init(String::from("(2x + 10)*(x - 7)*(x^2 + y + z + 1)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(
                v.expr(),
                "2x^4 - 4x^3 + 2x^2y + 2x^2z - 68x^2 - 4xy - 4xz - 4x - 70y - 70z - 70"
            ),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_multiplication_multivariate_factor_b() {
        let mut parser = Parser::parser_init(String::from(
            "(x^2 + y + z + 1)*(x - 7 + y)*(x^2 + y + z + 1)",
        ))
        .unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^5 + x^4y - 7x^4 + 2x^3y + 2x^3z + 2x^2y^2 + 2x^2yz + 2x^3 - 12x^2y - 14x^2z + xy^2 + 2xyz + xz^2 + y^3 + 2y^2z + yz^2 - 14x^2 + 2xy + 2xz - 5y^2 - 12yz - 7z^2 + x - 13y - 14z - 7"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_exponentiation_a() {
        let mut parser = Parser::parser_init(String::from("(x + 2)^3")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 6x^2 + 12x + 8"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_exponentiation_b() {
        let mut parser = Parser::parser_init(String::from("(x + 3)^2*(x + 3)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 9x^2 + 27x + 27"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_exponentiation_multivariate() {
        let mut parser = Parser::parser_init(String::from("(x + y + z)^5")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^5 + 5x^4y + 5x^4z + 10x^3y^2 + 20x^3yz + 10x^3z^2 + 10x^2y^3 + 30x^2y^2z + 30x^2yz^2 + 10x^2z^3 + 5xy^4 + 20xy^3z + 30xy^2z^2 + 20xyz^3 + 5xz^4 + y^5 + 5y^4z + 10y^3z^2 + 10y^2z^3 + 5yz^4 + z^5"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_exponentiation_expression() {
        let mut parser = Parser::parser_init(String::from("(x^2 + (x + 2)*(x + 2))^2")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "4x^4 + 16x^3 + 32x^2 + 32x + 16"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_a() {
        let mut parser = Parser::parser_init(String::from("x^3 + (x^3) + x^3 ")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "3x^3"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_b() {
        let mut parser = Parser::parser_init(String::from("x^3 + (x + 3)*(x + 3) + x^2")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 2x^2 + 6x + 9"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_c() {
        let mut parser =
            Parser::parser_init(String::from("x^3 + (x + 5)^3*(x + 4) + x^3 ")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^4 + 21x^3 + 135x^2 + 425x + 500"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_d() {
        let mut parser = Parser::parser_init(String::from(
            "x^3 + (x + 2)*(x + 2) + x^3 + (x + 2)*(x + 2) - x^3",
        ))
        .unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 2x^2 + 8x + 8"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_e() {
        let mut parser = Parser::parser_init(String::from("x^3 + (x + 3)*(x + 3) - x^2")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 + 6x + 9"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_f() {
        let mut parser = Parser::parser_init(String::from("x^3 - (x^3) - x^3 ")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "-x^3"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_repeated_addition_g() {
        let mut parser = Parser::parser_init(String::from("x^3 - (x + 3)*(x + 3) + x^2")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^3 - 6x - 9"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_distribute_negative_a() {
        let mut parser = Parser::parser_init(String::from("-(x + y + z)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "-x - y - z"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_distribute_negative_b() {
        let mut parser = Parser::parser_init(String::from("-(x + 2)*(x + 2)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "-x^2 - 4x - 4"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_distribute_negative_c() {
        let mut parser =
            Parser::parser_init(String::from("-(x^3 - (x + 3)*(x + 3) + x^2)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "-x^3 + 6x + 9"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_distribute_negative_d() {
        let mut parser = Parser::parser_init(String::from("-(x + 2)*-(x + 2)*-(x + 2)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "-x^3 - 6x^2 - 12x - 8"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn parse_polynomial_distribute_negative_e() {
        let mut parser = Parser::parser_init(String::from("-(x + 2)*-(x + 2) - -(x + 2)")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert_eq!(v.expr(), "x^2 + 5x + 6"),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn test_invalid_syntax_a() {
        let mut parser = Parser::parser_init(String::from("(x + 2)5")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(false, "{:?}", v),
            Err(e) => assert_eq!(ParserErr::InvalidSyntax(String::from("Invalid syntax")), e),
        }
    }

    #[rstest]
    fn test_invalid_syntax_b() {
        let mut parser = Parser::parser_init(String::from("(x + 5")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(false, "{:?}", v),
            Err(e) => assert_eq!(
                ParserErr::ExpectedToken(String::from(
                    "Expected closing parenthesis at end of expression"
                )),
                e
            ),
        }
    }

    #[rstest]
    fn test_parser_empty_input() {
        let mut parser = Parser::parser_init(String::from("")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(true, "{:?}", v),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn test_parser_input_space() {
        let mut parser = Parser::parser_init(String::from(" ")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(true, "{:?}", v),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn test_parser_input_new_line() {
        let mut parser = Parser::parser_init(String::from("\n")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(true, "{:?}", v),
            Err(e) => assert!(false, "{:?}", e),
        }
    }

    #[rstest]
    fn test_parser_input_space_and_new_line() {
        let mut parser = Parser::parser_init(String::from("   \n")).unwrap();
        let polynomial = parser.start_parser();
        match polynomial {
            Ok(v) => assert!(true, "{:?}", v),
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
