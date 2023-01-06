use crate::parser::{Parser, ParserErr};
use std::clone::Clone;
use std::ops;

use crate::monomial::Monomial;

#[derive(Debug)]
pub struct Polynomial {
    pub monomials: Vec<Monomial>,
}

impl Polynomial {
    pub fn new() -> Polynomial {
        Polynomial {
            monomials: Vec::new(),
        }
    }

    pub fn insert_monomial(&mut self, monomial: Monomial) {
        match self.monomials.binary_search(&monomial) {
            Ok(_) => {}
            Err(pos) => self.monomials.insert(pos, monomial),
        }
    }

    pub fn expr(&self) -> String {
        let mut output = String::from("");
        let mut first_term_printed: bool = false;

        for ind in 0..self.monomials.len() {
            let monomial = &self.monomials[ind];
            // skip nonzero terms
            if monomial.coefficient == 0.0 {
                continue;
            }
            // first term of a polynomial is printed differently
            if first_term_printed == false {
                let first_term = monomial;
                let coeff = first_term.coefficient.abs();
                let term_expr = first_term.term_expr();
                if first_term.coefficient < 0.0 {
                    output.push_str("-");
                }
                // print coefficient
                if coeff == 1.0 {
                    if first_term.degree() == 0 {
                        output.push_str(&format!("{coeff}"));
                    }
                } else {
                    output.push_str(&format!("{coeff}"));
                }
                // print term expr
                if first_term.degree() != 0 {
                    output.push_str(&format!("{term_expr}"));
                }
                first_term_printed = true;
            } else {
                if monomial.coefficient < 0.0 {
                    output.push_str(" - ");
                } else {
                    output.push_str(" + ");
                }
                let coeff = monomial.coefficient.abs();
                let term_expr = monomial.term_expr();
                if coeff == 1.0 {
                    if monomial.degree() == 0 {
                        output.push_str(&format!("{coeff}"));
                    }
                } else {
                    output.push_str(&format!("{coeff}"));
                }
                if monomial.degree() != 0 {
                    output.push_str(&format!("{term_expr}"));
                }
            }
        }
        output
    }

    pub fn print_polynomial(&self) {
        println!("{}", self.expr());
    }

    pub fn from(expr: &str) -> Result<Polynomial, ParserErr> {
        let mut parser = Parser::parser_init(String::from(expr));
        let polynomial = parser.parse_polynomial()?;
        Ok(polynomial)
    }

    pub fn pow(&self, exponent: i32) -> Polynomial {
        let mut ind = exponent - 1;
        let mut res = self.clone();
        while ind > 0 {
            res = res * self;
            ind -= 1;
        }
        res
    }

    pub fn scale(&mut self, scale: f64) -> () {
        for monomial in self.monomials.iter_mut() {
            monomial.coefficient *= scale;
        }
    }
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Self {
            monomials: self.monomials.clone(),
        }
    }
}

// Polynomial += Monomial
impl ops::AddAssign<Monomial> for Polynomial {
    fn add_assign(&mut self, other: Monomial) {
        match self
            .monomials
            .binary_search_by(|monomial| monomial.cmp_terms(&other))
        {
            Ok(pos) => {
                self.monomials[pos].coefficient += other.coefficient;
            }
            Err(pos) => {
                self.monomials.insert(pos, other);
            }
        }
    }
}

// Polynomial += Polynomial
impl ops::AddAssign for Polynomial {
    fn add_assign(&mut self, other: Self) {
        for monomial in other.monomials {
            *self += monomial;
        }
    }
}

// Polynomial + Polynomial
impl ops::Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_poly = self.clone();
        new_poly += other;
        new_poly
    }
}

// Polynomial + &Polynomial
impl ops::Add<&Polynomial> for Polynomial {
    type Output = Self;
    fn add(self, other: &Self) -> Self {
        let mut new_poly = self.clone();
        new_poly += other.clone();
        new_poly
    }
}

// &Polynomial + Polynomial
impl ops::Add<Polynomial> for &Polynomial {
    type Output = Polynomial;
    fn add(self, other: Polynomial) -> Polynomial {
        let mut new_poly = self.clone();
        new_poly += other.clone();
        new_poly
    }
}

// &Polynomial + &Polynomial
impl<'a> ops::Add<&'a Polynomial> for &'a Polynomial {
    type Output = Polynomial;
    fn add(self, other: Self) -> Polynomial {
        let mut new_poly = self.clone();
        new_poly += other.clone();
        new_poly
    }
}

// Polynomial - Polynomial
impl ops::Sub for Polynomial {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_poly = self.clone();
        new_poly += -1.0 * other;
        new_poly
    }
}

// Polynomial * Polynomial
impl ops::Mul for Polynomial {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut polynomial = Polynomial::new();
        for monomial in self.monomials.iter() {
            for other_monomial in other.monomials.iter() {
                polynomial += monomial.clone() * other_monomial.clone();
            }
        }
        polynomial
    }
}

// Polynomial * &Polynomial
impl ops::Mul<&Polynomial> for Polynomial {
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let res = self * other.clone();
        res
    }
}

// f64 * Polynomial
impl ops::Mul<Polynomial> for f64 {
    type Output = Polynomial;

    fn mul(self, other: Polynomial) -> Polynomial {
        let mut polynomial = other.clone();
        for monomial in polynomial.monomials.iter_mut() {
            monomial.coefficient *= self;
        }
        polynomial
    }
}

// Polynomial * f64
impl ops::Mul<f64> for Polynomial {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        other * self
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn polynomial_a() -> Polynomial {
        Polynomial::from("5x^2 + 6y^2").unwrap()
    }

    #[fixture]
    fn polynomial_b() -> Polynomial {
        Polynomial::from("7y^2").unwrap()
    }

    #[rstest]
    fn test_polynomial_from_str_a(polynomial_a: Polynomial) {
        // 5x^2 + 6y^2
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        assert_eq!(polynomial_a.monomials[0], monomial_a);
        assert_eq!(polynomial_a.monomials[1], monomial_b);
    }

    #[rstest]
    fn test_polynomial_from_str_b(polynomial_b: Polynomial) {
        // 7y^2
        let monomial = Monomial {
            coefficient: 7.0,
            power_list: vec![0, 2, 0],
        };
        assert_eq!(polynomial_b.monomials[0], monomial);
    }

    #[rstest]
    fn test_polynomial_from_str_c() {
        let polynomial = Polynomial::from("2x + y + z + 2x + y + y + y + z").unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(polynomial.expr(), "4x + 4y + 2z");
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[1].coefficient, 4.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "4x");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "4y");
        assert_eq!(polynomial.monomials[2].expr().as_str(), "2z");
    }

    #[rstest]
    fn test_polynomial_from_str_d() {
        let polynomial = Polynomial::from("2xyz + yzx + zxy + xy").unwrap();

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.expr(), "4xyz + xy");
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "4xyz");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "xy");
    }

    #[rstest]
    fn test_addition_1(polynomial_a: Polynomial, polynomial_b: Polynomial) {
        let polynomial = polynomial_b + polynomial_a;

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.expr(), "5x^2 + 13y^2");
        assert_eq!(polynomial.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial.monomials[1].coefficient, 13.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "5x^2");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "13y^2");
    }

    #[rstest]
    fn test_polynomial_addition_like_terms_a() {
        let mut polynomial = Polynomial::from("x^4 + x^3 + x^2").unwrap();
        let other = Polynomial::from("x^2").unwrap();
        polynomial += other;

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(polynomial.expr(), "x^4 + x^3 + 2x^2");
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "x^4");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "x^3");
        assert_eq!(polynomial.monomials[2].expr().as_str(), "2x^2");
    }

    #[rstest]
    fn test_polynomial_addition_like_terms_b() {
        let mut polynomial = Polynomial::from("x^4 + x^3 + x^2").unwrap();
        let other = Polynomial::from("x^2 + x^4 + x").unwrap();
        polynomial += other;

        assert_eq!(polynomial.monomials.len(), 4);
        assert_eq!(polynomial.expr(), "2x^4 + x^3 + 2x^2 + x");
        assert_eq!(polynomial.monomials[0].coefficient, 2.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(polynomial.monomials[3].coefficient, 1.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "2x^4");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "x^3");
        assert_eq!(polynomial.monomials[2].expr().as_str(), "2x^2");
        assert_eq!(polynomial.monomials[3].expr().as_str(), "x");
    }

    #[rstest]
    fn test_mul_1(polynomial_a: Polynomial, polynomial_b: Polynomial) {
        let polynomial = polynomial_a * polynomial_b;
        assert_eq!(polynomial.expr(), "35x^2y^2 + 42y^4");
        assert_eq!(
            polynomial.monomials[0],
            Monomial {
                coefficient: 35.0,
                power_list: vec![2, 2, 0],
            }
        );
        assert_eq!(
            polynomial.monomials[1],
            Monomial {
                coefficient: 42.0,
                power_list: vec![0, 4, 0],
            }
        );
        assert_eq!(polynomial.monomials[0].expr().as_str(), "35x^2y^2");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "42y^4");
    }

    #[rstest]
    fn test_pow_a() {
        let polynomial = Polynomial::from("x + 2").unwrap();
        assert_eq!(polynomial.pow(1).expr(), "x + 2");
    }

    #[rstest]
    fn test_pow_b() {
        let polynomial = Polynomial::from("x + 2").unwrap();
        assert_eq!(polynomial.pow(2).expr(), "x^2 + 4x + 4");
    }

    #[rstest]
    fn test_pow_c() {
        let polynomial = Polynomial::from("x + 2").unwrap();
        assert_eq!(polynomial.pow(3).expr(), "x^3 + 6x^2 + 12x + 8");
    }
}
