use std::clone::Clone;
use std::cmp::Ordering;
use std::ops;

use crate::monomial::Monomial;

#[derive(Debug)]
pub struct Polynomial {
    pub monomials: Vec<Monomial>,
}

impl Polynomial {
    pub fn print_polynomial(&self) {
        for monomial in &self.monomials {
            println!("{}", monomial.expr());
        }
    }

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
        for mut monomial in self.monomials.iter_mut() {
            for other_monomial in &other.monomials {
                if monomial.cmp_terms(&other_monomial) == Ordering::Equal {
                    monomial.coefficient += other_monomial.coefficient;
                    break;
                }
            }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_1() {
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        let monomial_c = Monomial {
            coefficient: 7.0,
            power_list: vec![0, 2, 0],
        };

        let polynomial_a = Polynomial {
            monomials: vec![monomial_a, monomial_b]
        };
        let polynomial_b = Polynomial {
            monomials: vec![monomial_c]
        };
        let polynomial = polynomial_a * polynomial_b;
        assert_eq!(polynomial.monomials[0], Monomial {
            coefficient: 42.0,
            power_list: vec![0, 4, 0],
        });
        assert_eq!(polynomial.monomials[1], Monomial {
            coefficient: 35.0,
            power_list: vec![2, 2, 0],
        });

    }
}

