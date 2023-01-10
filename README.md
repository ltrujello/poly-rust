# Poly-rust

This is a multivariate polynomial library for Rust over the real field, as I was unable to find a suitable one that already exists. 

Well actually, this is a combination of two projects: A polynomial library for Rust, and an implementation of a polynomial grammar I made to parse multivaraite polynomial expressions. One day I'll push this as a crate.

The goal is to extend this to infinite series over arbitrary fields, create an interpreter using the parser, and then write programs to mess around with infinite series.

## Example
```
extern crate polynomial;
pub use polynomial::Polynomial;

let polynomial = Polynomial::from("(x^2 + y^2 + z^2)^2").unwrap();
println!("{}", polynomial.expr());
// will print
x^4 + 2x^2y^2 + 2x^2z^2 + y^4 + 2y^2z^2 + z^4
```

