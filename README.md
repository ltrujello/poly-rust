# Poly-rust

This is a multivariate polynomial library over generic types for Rust, as I was unable to find a suitable one that already exists. 

Well actually, this is a combination of two projects: A polynomial library for Rust, and an implementation of a polynomial grammar I made to parse multivaraite polynomial expressions. One day I'll push this as a crate.

The goal is to extend this to infinite series over arbitrary fields, create an interpreter using the parser, and then write programs to mess around with infinite series.

## Example
```rust
extern crate poly;
pub use poly::Polynomial64; 

let polynomial = Polynomial64::from("(x^2 + y^2 + z^2)^2").unwrap();
println!("{}", polynomial); 
// x^4 + 2x^2y^2 + 2x^2z^2 + y^4 + 2y^2z^2 + z^4

let other = Polynomial64::from("x^4 + y^4 + z^4").unwrap();
let sub = polynomial - other;
println!("{}", sub); 
// 2x^2y^2 + 2x^2z^2 + 2y^2z^2

println!("{}", sub.pow(2)); 
// 4x^4y^4 + 8x^4y^2z^2 + 4x^4z^4 + 8x^2y^4z^2 + 8x^2y^2z^4 + 4y^4z^4
```

## Type Generics
Polynomials are generally elements over a commutative ring whose coefficients are elements of a field. However, you really only need the coefficients to be elements over a commutative ring. Hence, this library enforces that the types `T` for polynomial coefficients of the `Polynomial` struct (and hence monomial coefficients of the `Monomial` struct) must satisfy the trait `CRing` defined as follows. 
```rust
CRing: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + PartialEq
```

## Grammar
The following grammar is used to create the set of acceptable polynomial expressions.
```
polyexpr -> polyexpr + term
            | polyexpr - term
            | term

term -> term * factor 
        | factor

factor -> (polyexpr)
        | -(polyexpr)
        | polynomial
        | (polyexpr)^n

polynomial -> polynomial + monomial
            | polynomial - monomial
            | monomial

monomial -> x^Int
            | Float x^Int 
            | Float x 
            | x 
            | Float 
```
