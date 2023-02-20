mod lexer;
mod monomial;
pub mod monomial_generic;
pub mod parser;
mod polynomial;
pub mod polynomial_generic;

use num::{One, Signed, Zero};
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

pub trait RingOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + PartialEq
    + std::fmt::Display
{
}
impl<T, Rhs, Output> RingOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + PartialEq
        + std::fmt::Display
{
}

pub trait CRing: Zero + One + RingOps<Self, Self> {}
impl<T> CRing for T where T: Zero + One + RingOps<Self, Self> {}
