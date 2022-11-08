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
}

impl Clone for Polynomial {
    fn clone(&self) -> Self {
        Self {
            monomials: self.monomials.clone(),
        }
    }
}

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

impl ops::Add for Polynomial {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut new_poly = self.clone();
        new_poly += other;
        new_poly
    }
}

