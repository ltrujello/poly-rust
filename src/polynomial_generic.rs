use crate::parser::{Parser, ParserErr};
use log::info;
use num;
use std::clone::Clone;
use std::ops;

use crate::monomial_generic::Monomial;
use crate::CRing;

pub type Polynomial32 = Polynomial<f32>;
pub type Polynomial64 = Polynomial<f64>;

#[derive(Debug)]
pub struct Polynomial<T: CRing> {
    pub monomials: Vec<Monomial<T>>,
}

impl<T> Polynomial<T>
where
    T: CRing + Clone + num::Signed,
{
    pub fn new() -> Polynomial<T> {
        Polynomial {
            monomials: Vec::new(),
        }
    }

    pub fn insert_monomial(&mut self, monomial: Monomial<T>) {
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
            if monomial.coefficient.is_zero() {
                continue;
            }
            // first term of a polynomial is printed differently
            if first_term_printed == false {
                let first_term = monomial;
                let coeff = num::abs(first_term.coefficient.clone());
                let term_expr = first_term.term_expr();
                if first_term.coefficient.is_negative() {
                    output.push_str("-");
                }
                // print coefficient
                if coeff.is_one() {
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
                if monomial.coefficient.is_negative() {
                    output.push_str(" - ");
                } else {
                    output.push_str(" + ");
                }
                let coeff = num::abs(monomial.coefficient.clone());
                let term_expr = monomial.term_expr();
                if coeff.is_one() {
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

    // pub fn from(expr: &str) -> Result<Polynomial, ParserErr> {
    //     let mut parser = Parser::parser_init(String::from(expr))?;
    //     let polynomial = parser.parse_polynomial()?;
    //     Ok(polynomial)
    // }

    pub fn pow(&self, exponent: i32) -> Polynomial<T> {
        let mut ind = exponent - 1;
        let mut res = self.clone();
        while ind > 0 {
            res = res * self;
            ind -= 1;
        }
        res
    }

    pub fn scale(&mut self, scale: T) -> () {
        for monomial in self.monomials.iter_mut() {
            monomial.coefficient = monomial.coefficient.clone() * scale.clone();
        }
    }
}

impl<T> Clone for Polynomial<T>
where
    T: CRing + Clone,
{
    fn clone(&self) -> Self {
        Self {
            monomials: self.monomials.clone(),
        }
    }
}

// Polynomial += Monomial
impl<T> ops::AddAssign<Monomial<T>> for Polynomial<T>
where
    T: CRing + Clone,
{
    fn add_assign(&mut self, other: Monomial<T>) {
        match self
            .monomials
            .binary_search_by(|monomial| monomial.cmp_terms(&other))
        {
            Ok(pos) => {
                self.monomials[pos].coefficient =
                    self.monomials[pos].coefficient.clone() + other.coefficient;
            }
            Err(pos) => {
                self.monomials.insert(pos, other);
            }
        }
    }
}

// Polynomial -= Monomial
impl<T> ops::SubAssign<Monomial<T>> for Polynomial<T>
where
    T: CRing + Clone,
{
    fn sub_assign(&mut self, mut other: Monomial<T>) {
        match self
            .monomials
            .binary_search_by(|monomial| monomial.cmp_terms(&other))
        {
            Ok(pos) => {
                self.monomials[pos].coefficient =
                    self.monomials[pos].coefficient.clone() - other.coefficient;
            }
            Err(pos) => {
                let mut zero = other.coefficient.clone();
                zero.set_zero();
                other.coefficient = zero - other.coefficient.clone();
                self.monomials.insert(pos, other);
            }
        }
    }
}

// Polynomial += Polynomial
impl<T> ops::AddAssign for Polynomial<T>
where
    T: CRing + Clone,
{
    fn add_assign(&mut self, other: Self) {
        for monomial in other.monomials {
            *self += monomial;
        }
    }
}

// Polynomial -= Polynomial
impl<T> ops::SubAssign for Polynomial<T>
where
    T: CRing + Clone,
{
    fn sub_assign(&mut self, other: Self) {
        for monomial in other.monomials {
            *self -= monomial;
        }
    }
}

// Polynomial + Polynomial
impl<T> ops::Add for Polynomial<T>
where
    T: CRing + Clone,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_poly = self.clone();
        new_poly += other;
        new_poly
    }
}

//Polynomial - Polynomial
impl<T> ops::Sub for Polynomial<T>
where
    T: CRing + Clone,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut new_poly = self.clone();
        new_poly -= other;
        new_poly
    }
}

// Polynomial + &Polynomial
// impl<T> ops::Add<&Polynomial<T>> for Polynomial<T>
// where T:CRing {
//     type Output = Self;
//     fn add(self, other: &Self) -> Self {
//         let mut new_poly = self.clone();
//         new_poly += other.clone();
//         new_poly
//     }
// }

// &Polynomial + Polynomial
// impl<T> ops::Add<Polynomial<T>> for &Polynomial<T>
// where T: CRing
// {
//     type Output = Polynomial<T>;
//     fn add(self, other: Polynomial<T>) -> Polynomial<T> {
//         let mut new_poly = self.clone();
//         new_poly += other.clone();
//         new_poly
//     }
// }

// &Polynomial + &Polynomial
// impl<'a> ops::Add<&'a Polynomial> for &'a Polynomial<T> {
//     type Output = Polynomial<T>;
//     fn add(self, other: Self) -> Polynomial<T> {
//         let mut new_poly = self.clone();
//         new_poly += other.clone();
//         new_poly
//     }
// }

// Polynomial * Polynomial
impl<T> ops::Mul for Polynomial<T>
where
    T: CRing + Clone + num::Signed,
{
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
impl<T> ops::Mul<&Polynomial<T>> for Polynomial<T>
where
    T: CRing + Clone + num::Signed,
{
    type Output = Self;
    fn mul(self, other: &Self) -> Self {
        let mut polynomial = Polynomial::new();
        for monomial in self.monomials.iter() {
            for other_monomial in other.monomials.iter() {
                polynomial += monomial.clone() * other_monomial.clone();
            }
        }
        polynomial
    }
}

// T * Polynomial
// impl<T> ops::Mul<Polynomial<T>> for T
// where T: CRing + Clone
// {
//     type Output = Polynomial<T>;

//     fn mul(self, other: Polynomial<T>) -> Polynomial<T> {
//         let mut polynomial = other.clone();
//         for monomial in polynomial.monomials.iter_mut() {
//             monomial.coefficient = self * monomial.coefficient;
//         }
//         polynomial
//     }
// }

// Polynomial * T
impl<T> ops::Mul<T> for Polynomial<T>
where
    T: CRing + Clone,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let mut polynomial = self.clone();
        for monomial in polynomial.monomials.iter_mut() {
            monomial.coefficient = other.clone() * monomial.coefficient.clone();
        }
        polynomial
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn polynomial_a() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        polynomial
    }

    #[fixture]
    fn polynomial_b() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 7.0,
            power_list: vec![0, 2, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial
    }

    #[fixture]
    fn linear_polynomial() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 2.0,
            power_list: vec![0, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        polynomial
    }

    #[fixture]
    fn polynomial_c() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![4, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![3, 0, 0],
        };
        let monomial_c = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        polynomial.insert_monomial(monomial_c);
        polynomial
    }

    #[fixture]
    fn polynomial_d() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![4, 0, 0],
        };
        let monomial_c = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        polynomial.insert_monomial(monomial_c);
        polynomial
    }

    #[fixture]
    fn polynomial_e() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial
    }

    // #[rstest]
    // fn test_polynomial_from_str_a(polynomial_a: Polynomial) {
    //     // 5x^2 + 6y^2
    //     let monomial_a = Monomial {
    //         coefficient: 5.0,
    //         power_list: vec![2, 0, 0],
    //     };
    //     let monomial_b = Monomial {
    //         coefficient: 6.0,
    //         power_list: vec![0, 2, 0],
    //     };
    //     assert_eq!(polynomial_a.monomials[0], monomial_a);
    //     assert_eq!(polynomial_a.monomials[1], monomial_b);
    // }

    // #[rstest]
    // fn test_polynomial_from_str_b(polynomial_b: Polynomial) {
    //     // 7y^2
    //     let monomial = Monomial {
    //         coefficient: 7.0,
    //         power_list: vec![0, 2, 0],
    //     };
    //     assert_eq!(polynomial_b.monomials[0], monomial);
    // }

    // #[rstest]
    // fn test_polynomial_from_str_c() {
    //     let polynomial = Polynomial::from("2x + y + z + 2x + y + y + y + z").unwrap();

    //     assert_eq!(polynomial.monomials.len(), 3);
    //     assert_eq!(polynomial.expr(), "4x + 4y + 2z");
    //     assert_eq!(polynomial.monomials[0].coefficient, 4.0);
    //     assert_eq!(polynomial.monomials[0].coefficient, 4.0);
    //     assert_eq!(polynomial.monomials[1].coefficient, 4.0);
    //     assert_eq!(polynomial.monomials[2].coefficient, 2.0);
    //     assert_eq!(polynomial.monomials[0].expr().as_str(), "4x");
    //     assert_eq!(polynomial.monomials[1].expr().as_str(), "4y");
    //     assert_eq!(polynomial.monomials[2].expr().as_str(), "2z");
    // }

    // #[rstest]
    // fn test_polynomial_from_str_d() {
    //     let polynomial = Polynomial::from("2xyz + yzx + zxy + xy").unwrap();

    //     assert_eq!(polynomial.monomials.len(), 2);
    //     assert_eq!(polynomial.expr(), "4xyz + xy");
    //     assert_eq!(polynomial.monomials[0].coefficient, 4.0);
    //     assert_eq!(polynomial.monomials[1].coefficient, 1.0);
    //     assert_eq!(polynomial.monomials[0].expr().as_str(), "4xyz");
    //     assert_eq!(polynomial.monomials[1].expr().as_str(), "xy");
    // }

    #[rstest]
    fn test_addition_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_b + polynomial_a;

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.expr(), "5x^2 + 13y^2");
        assert_eq!(polynomial.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial.monomials[1].coefficient, 13.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "5x^2");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "13y^2");
    }

    #[rstest]
    fn test_polynomial_addition_like_terms_a(
        polynomial_c: Polynomial64,
        polynomial_e: Polynomial64,
    ) {
        let mut polynomial = polynomial_c;
        let other = polynomial_e;
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
    fn test_polynomial_addition_like_terms_b(
        polynomial_c: Polynomial64,
        polynomial_d: Polynomial64,
    ) {
        let mut polynomial = polynomial_c.clone();
        let other = polynomial_d.clone();
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
    fn test_subtraction_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_a - polynomial_b;

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(polynomial.expr(), "5x^2 - y^2");
        assert_eq!(polynomial.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial.monomials[1].coefficient, -1.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "5x^2");
        assert_eq!(format!("{}", polynomial.monomials[1]), "-y^2");
    }

    #[rstest]
    fn test_polynomial_subtraction_like_terms_a(
        polynomial_c: Polynomial64,
        polynomial_e: Polynomial64,
    ) {
        let mut polynomial = polynomial_c;
        let other = polynomial_e;
        polynomial -= other;

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(polynomial.expr(), "x^4 + x^3");
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[0].expr().as_str(), "x^4");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "x^3");
    }

    #[rstest]
    fn test_polynomial_subtraction_like_terms_b(
        polynomial_c: Polynomial64,
        polynomial_d: Polynomial64,
    ) {
        let mut polynomial = polynomial_c.clone();
        let other = polynomial_d.clone();
        polynomial -= other;

        assert_eq!(polynomial.monomials.len(), 4);
        assert_eq!(polynomial.expr(), "x^3 - x");
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[3].coefficient, -1.0);
        assert_eq!(polynomial.monomials[1].expr().as_str(), "x^3");
        assert_eq!(format!("{}", polynomial.monomials[3]), "-x");
    }

    #[rstest]
    fn test_mul_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_a * polynomial_b;
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
        assert_eq!(polynomial.expr(), "35x^2y^2 + 42y^4");
        assert_eq!(polynomial.monomials[0].expr().as_str(), "35x^2y^2");
        assert_eq!(polynomial.monomials[1].expr().as_str(), "42y^4");
    }

    #[rstest]
    fn test_pow_a(linear_polynomial: Polynomial64) {
        assert_eq!(linear_polynomial.pow(1).expr(), "x + 2");
    }

    #[rstest]
    fn test_pow_b(linear_polynomial: Polynomial64) {
        assert_eq!(linear_polynomial.pow(2).expr(), "x^2 + 4x + 4");
    }

    #[rstest]
    fn test_pow_c(linear_polynomial: Polynomial64) {
        assert_eq!(linear_polynomial.pow(3).expr(), "x^3 + 6x^2 + 12x + 8");
    }
}
