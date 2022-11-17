use std::clone::Clone;
use std::cmp::Ordering;
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
            Ok(pos) => {}
            Err(pos) => self.monomials.insert(pos, monomial),
        }
    }

    pub fn expr(&self) -> String {
        let mut output = String::from("");
        if self.monomials.len() < 1 {
            return output;
        }
        // first term of a polynomial is printed differently
        let first_term = &self.monomials[0];
        let coeff = first_term.coefficient.abs();
        let term_expr = first_term.term_expr();
        if first_term.coefficient < 0.0 {
            output.push_str("-");
        }
        output.push_str(&format!("{coeff}{term_expr}"));

        // print the remainder of the terms
        for ind in 1..self.monomials.len() {
            let monomial = &self.monomials[ind];
            if monomial.coefficient < 0.0 {
                output.push_str(" - ");
            } else {
                output.push_str(" + ");
            }
            let coeff = monomial.coefficient.abs();
            let term_expr = monomial.term_expr();
            output.push_str(&format!("{coeff}{term_expr}"));
        }
        output
    }

    pub fn print_polynomial(&self) {
        println!("{}", self.expr());
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
        for mut monomial in self.monomials.iter_mut() {
            if monomial.cmp_terms(&other) == Ordering::Equal {
                monomial.coefficient += other.coefficient;
                return;
            }
        }
        self.insert_monomial(other);
    }
}

// Polynomial += Polynomial
impl ops::AddAssign for Polynomial {
    fn add_assign(&mut self, other: Self) {
        let mut leftover_monomials = Vec::new(); for mut monomial in self.monomials.iter_mut() {
            for other_monomial in &other.monomials {
                if monomial.cmp_terms(&other_monomial) == Ordering::Equal {
                    monomial.coefficient += other_monomial.coefficient;
                    continue;
                }
                leftover_monomials.push(other_monomial.clone());
            }
        }
        for monomial in leftover_monomials{
            self.monomials.push(monomial);
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
        // 5x^2 + 6y^2
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        let polynomial_a = Polynomial {
            monomials: vec![monomial_a, monomial_b],
        };
        polynomial_a
    }

    #[fixture]
    fn polynomial_b() -> Polynomial {
        // 7y^2 
        let monomial_c = Monomial {
            coefficient: 7.0,
            power_list: vec![0, 2, 0],
        };
        let polynomial_b = Polynomial {
            monomials: vec![monomial_c],
        };
        polynomial_b
    }

    #[rstest]
    fn test_mul_1(polynomial_a: Polynomial, polynomial_b: Polynomial) {
        let polynomial = polynomial_a * polynomial_b;
        assert_eq!(
            polynomial.monomials[0],
            Monomial {
                coefficient: 42.0,
                power_list: vec![0, 4, 0],
            }
        );
        assert_eq!(
            polynomial.monomials[1],
            Monomial {
                coefficient: 35.0,
                power_list: vec![2, 2, 0],
            }
        );
    }

    #[rstest]
    fn test_addition_1(polynomial_a: Polynomial, polynomial_b: Polynomial) {
        let polynomial = polynomial_b + polynomial_a;
        assert_eq!(polynomial.expr(), "13y^2 + 5x^2");
    }
}
