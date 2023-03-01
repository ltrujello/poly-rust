use crate::parser::{Parser, ParserErr};
use std::clone::Clone;
use std::fmt;
use std::ops;

use crate::monomial::Monomial;
use crate::CRing;

pub type Polynomial32 = Polynomial<f32>;
pub type Polynomial64 = Polynomial<f64>;

#[derive(Debug)]
pub struct Polynomial<T: CRing> {
    pub monomials: Vec<Monomial<T>>,
}

impl<T> Polynomial<T>
where
    T: CRing,
{
    pub fn new() -> Polynomial<T> {
        Polynomial {
            monomials: Vec::new(),
        }
    }

    pub fn from(expr: &str) -> Result<Polynomial64, ParserErr> {
        let mut parser = Parser::parser_init(String::from(expr))?;
        let polynomial = parser.parse_poly_expr()?;
        Ok(polynomial)
    }
}

impl<T> Polynomial<T>
where
    T: CRing + PartialEq,
{
    pub fn insert_monomial(&mut self, monomial: Monomial<T>) {
        match self.monomials.binary_search(&monomial) {
            Ok(_) => {}
            Err(pos) => self.monomials.insert(pos, monomial),
        }
    }
}

impl<T> Polynomial<T>
where
    T: CRing + PartialEq + Clone,
{
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

impl<T> std::fmt::Display for Polynomial<T>
where
    T: std::fmt::Display + CRing + Clone + PartialEq,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!("");
        let mut first_term_printed: bool = false;

        for ind in 0..self.monomials.len() {
            let monomial = &self.monomials[ind];
            // skip nonzero terms
            if monomial.coefficient.is_zero() {
                continue;
            }
            // terms after first monomial term are printed differently
            if first_term_printed {
                let sign: &str;
                let mut monomial_expr = format!("{}", monomial);
                let is_negative: bool;

                let mut chars = monomial_expr.chars();
                let first_char = chars.next().unwrap();
                if first_char == '-' {
                    is_negative = true;
                    monomial_expr = chars.as_str().to_string();
                } else {
                    is_negative = false;
                }

                // sign
                if is_negative {
                    sign = " - ";
                } else {
                    sign = " + ";
                }

                output.push_str(&format!("{}{}", sign, monomial_expr));
            } else {
                output.push_str(&format!("{}", monomial));
                first_term_printed = true;
            }
        }
        write!(f, "{}", output)
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    T: CRing + Clone + PartialEq,
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
    use crate::monomial::Monomial64;
    use rstest::*;
    use smallvec::smallvec;

    #[fixture]
    fn polynomial_a() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: smallvec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: smallvec![0, 2, 0],
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
            power_list: smallvec![0, 2, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial
    }

    #[fixture]
    fn linear_polynomial() -> Polynomial64 {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: smallvec![1, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 2.0,
            power_list: smallvec![0, 0, 0],
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
            power_list: smallvec![4, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: smallvec![3, 0, 0],
        };
        let monomial_c = Monomial {
            coefficient: 1.0,
            power_list: smallvec![2, 0, 0],
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
            power_list: smallvec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: smallvec![4, 0, 0],
        };
        let monomial_c = Monomial {
            coefficient: 1.0,
            power_list: smallvec![1, 0, 0],
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
            power_list: smallvec![2, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial
    }

    #[rstest]
    fn test_insert_monomial() {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: smallvec![2, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[0].power_list[0], 2);
    }

    #[rstest]
    fn test_insert_monomial_exists_with_diff_power_list() {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: smallvec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 3.0,
            power_list: smallvec![1, 1, 1],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        assert_eq!(polynomial.monomials[0].coefficient, 3.0);
        assert_eq!(polynomial.monomials[0].power_list[0], 1);
        assert_eq!(polynomial.monomials[0].power_list[1], 1);
        assert_eq!(polynomial.monomials[0].power_list[2], 1);
    }

    #[rstest]
    fn test_insert_monomial_exists_with_same_power_list() {
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: smallvec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 3.0,
            power_list: smallvec![2, 0, 0],
        };
        let mut polynomial: Polynomial<f64> = Polynomial::new();
        polynomial.insert_monomial(monomial_a);
        polynomial.insert_monomial(monomial_b);
        assert_eq!(polynomial.monomials[0].coefficient, 3.0);
        assert_eq!(polynomial.monomials[0].power_list[0], 2);
        assert_eq!(polynomial.monomials[0].power_list[1], 0);
        assert_eq!(polynomial.monomials[0].power_list[2], 0);
    }

    #[rstest]
    fn test_polynomial_from_str_a(polynomial_a: Polynomial64) {
        // 5x^2 + 6y^2
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: smallvec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: smallvec![0, 2, 0],
        };
        assert_eq!(polynomial_a.monomials[0], monomial_a);
        assert_eq!(polynomial_a.monomials[1], monomial_b);
    }

    #[rstest]
    fn test_polynomial_from_str_b(polynomial_b: Polynomial64) {
        // 7y^2
        let monomial = Monomial {
            coefficient: 7.0,
            power_list: smallvec![0, 2, 0],
        };
        assert_eq!(polynomial_b.monomials[0], monomial);
    }

    #[rstest]
    fn test_polynomial_from_str_c() {
        let polynomial = Polynomial::<f64>::from("2x + y + z + 2x + y + y + y + z").unwrap();

        assert_eq!(polynomial.monomials.len(), 3);
        assert_eq!(format!("{}", polynomial), "4x + 4y + 2z");
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[1].coefficient, 4.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "4x");
        assert_eq!(format!("{}", polynomial.monomials[1]), "4y");
        assert_eq!(format!("{}", polynomial.monomials[2]), "2z");
    }

    #[rstest]
    fn test_polynomial_from_str_d() {
        let polynomial = Polynomial::<f64>::from("2xyz + yzx + zxy + xy").unwrap();

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(format!("{}", polynomial), "4xyz + xy");
        assert_eq!(polynomial.monomials[0].coefficient, 4.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "4xyz");
        assert_eq!(format!("{}", polynomial.monomials[1]), "xy");
    }

    #[rstest]
    fn test_addition_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_b + polynomial_a;

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(format!("{}", polynomial), "5x^2 + 13y^2");
        assert_eq!(polynomial.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial.monomials[1].coefficient, 13.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "5x^2");
        assert_eq!(format!("{}", polynomial.monomials[1]), "13y^2");
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
        assert_eq!(format!("{}", polynomial), "x^4 + x^3 + 2x^2");
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "x^4");
        assert_eq!(format!("{}", polynomial.monomials[1]), "x^3");
        assert_eq!(format!("{}", polynomial.monomials[2]), "2x^2");
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
        assert_eq!(format!("{}", polynomial), "2x^4 + x^3 + 2x^2 + x");
        assert_eq!(polynomial.monomials[0].coefficient, 2.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[2].coefficient, 2.0);
        assert_eq!(polynomial.monomials[3].coefficient, 1.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "2x^4");
        assert_eq!(format!("{}", polynomial.monomials[1]), "x^3");
        assert_eq!(format!("{}", polynomial.monomials[2]), "2x^2");
        assert_eq!(format!("{}", polynomial.monomials[3]), "x");
    }

    #[rstest]
    fn test_subtraction_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_a - polynomial_b;

        assert_eq!(polynomial.monomials.len(), 2);
        assert_eq!(format!("{}", polynomial), "5x^2 - y^2");
        assert_eq!(polynomial.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial.monomials[1].coefficient, -1.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "5x^2");
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
        assert_eq!(format!("{}", polynomial), "x^4 + x^3");
        assert_eq!(polynomial.monomials[0].coefficient, 1.0);
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(format!("{}", polynomial.monomials[0]), "x^4");
        assert_eq!(format!("{}", polynomial.monomials[1]), "x^3");
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
        assert_eq!(format!("{}", polynomial), "x^3 - x");
        assert_eq!(polynomial.monomials[1].coefficient, 1.0);
        assert_eq!(polynomial.monomials[3].coefficient, -1.0);
        assert_eq!(format!("{}", polynomial.monomials[1]), "x^3");
        assert_eq!(format!("{}", polynomial.monomials[3]), "-x");
    }

    #[rstest]
    fn test_add_assign_monomial_like_terms(mut polynomial_a: Polynomial64) {
        let monomial = Monomial64::from("5x^2").unwrap();
        polynomial_a += monomial;

        assert_eq!(format!("{}", polynomial_a), "10x^2 + 6y^2");
        assert_eq!(polynomial_a.monomials[0].coefficient, 10.0);
        assert_eq!(polynomial_a.monomials[1].coefficient, 6.0);
        assert_eq!(format!("{}", polynomial_a.monomials[0]), "10x^2");
        assert_eq!(format!("{}", polynomial_a.monomials[1]), "6y^2");
    }

    #[rstest]
    fn test_add_assign_monomial_no_like_terms(mut polynomial_a: Polynomial64) {
        let monomial = Monomial64::from("5z^2").unwrap();
        polynomial_a += monomial;

        assert_eq!(format!("{}", polynomial_a), "5x^2 + 6y^2 + 5z^2");
        assert_eq!(polynomial_a.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial_a.monomials[1].coefficient, 6.0);
        assert_eq!(polynomial_a.monomials[2].coefficient, 5.0);
        assert_eq!(format!("{}", polynomial_a.monomials[0]), "5x^2");
        assert_eq!(format!("{}", polynomial_a.monomials[1]), "6y^2");
        assert_eq!(format!("{}", polynomial_a.monomials[2]), "5z^2");
    }

    #[rstest]
    fn test_sub_assign_monomial_like_terms(mut polynomial_a: Polynomial64) {
        let monomial = Monomial64::from("3x^2").unwrap();
        polynomial_a -= monomial;

        assert_eq!(format!("{}", polynomial_a), "2x^2 + 6y^2");
        assert_eq!(polynomial_a.monomials[0].coefficient, 2.0);
        assert_eq!(polynomial_a.monomials[1].coefficient, 6.0);
        assert_eq!(format!("{}", polynomial_a.monomials[0]), "2x^2");
        assert_eq!(format!("{}", polynomial_a.monomials[1]), "6y^2");
    }

    #[rstest]
    fn test_sub_assign_monomial_no_like_terms(mut polynomial_a: Polynomial64) {
        let monomial = Monomial64::from("5z^2").unwrap();
        polynomial_a -= monomial;

        assert_eq!(format!("{}", polynomial_a), "5x^2 + 6y^2 - 5z^2");
        assert_eq!(polynomial_a.monomials[0].coefficient, 5.0);
        assert_eq!(polynomial_a.monomials[1].coefficient, 6.0);
        assert_eq!(polynomial_a.monomials[2].coefficient, -5.0);
        assert_eq!(format!("{}", polynomial_a.monomials[0]), "5x^2");
        assert_eq!(format!("{}", polynomial_a.monomials[1]), "6y^2");
        assert_eq!(format!("{}", polynomial_a.monomials[2]), "-5z^2");
    }

    #[rstest]
    fn test_mul_1(polynomial_a: Polynomial64, polynomial_b: Polynomial64) {
        let polynomial = polynomial_a * polynomial_b;
        assert_eq!(
            polynomial.monomials[0],
            Monomial {
                coefficient: 35.0,
                power_list: smallvec![2, 2, 0],
            }
        );
        assert_eq!(
            polynomial.monomials[1],
            Monomial {
                coefficient: 42.0,
                power_list: smallvec![0, 4, 0],
            }
        );
        assert_eq!(format!("{}", polynomial), "35x^2y^2 + 42y^4");
        assert_eq!(format!("{}", polynomial.monomials[0]), "35x^2y^2");
        assert_eq!(format!("{}", polynomial.monomials[1]), "42y^4");
    }

    #[rstest]
    fn test_mul_polynomial_by_scalar(polynomial_a: Polynomial64) {
        let polynomial = polynomial_a * 4.0;
        assert_eq!(format!("{}", polynomial), "20x^2 + 24y^2");
        assert_eq!(format!("{}", polynomial.monomials[0]), "20x^2");
        assert_eq!(format!("{}", polynomial.monomials[1]), "24y^2");
    }

    #[rstest]
    fn test_pow_1(linear_polynomial: Polynomial64) {
        assert_eq!(format!("{}", linear_polynomial.pow(1)), "x + 2");
    }

    #[rstest]
    fn test_pow_2(linear_polynomial: Polynomial64) {
        assert_eq!(format!("{}", linear_polynomial.pow(2)), "x^2 + 4x + 4");
    }

    #[rstest]
    fn test_pow_3(linear_polynomial: Polynomial64) {
        assert_eq!(
            format!("{}", linear_polynomial.pow(3)),
            "x^3 + 6x^2 + 12x + 8"
        );
    }
}
