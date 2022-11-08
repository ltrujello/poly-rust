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
        coefficient: 5.0,
        power_list: vec![0, 2, 0],
    };

    let monomial_d =  monomial_c * 2.0;
    println!("{}", monomial_d); 
}
