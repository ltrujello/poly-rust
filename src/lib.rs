pub mod interpreter;
mod lexer;
pub mod monomial;
pub mod parser;
pub mod polynomial;

use num::{One, Zero};
use std::ops::{Add, Mul, Sub};

pub trait CRing<Rhs = Self, Output = Self>:
    Zero + One + Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output>
{
}
impl<T, Rhs, Output> CRing<Rhs, Output> for T where
    T: Zero
        + One
        + Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
{
}
