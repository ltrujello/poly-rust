mod monomial;
mod polynomial;

pub use crate::monomial::Monomial;
pub use crate::polynomial::Polynomial;

fn main() {
    println!("Hello world");
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

    polynomial.print_polynomial();
}
