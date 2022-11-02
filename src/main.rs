use std::cmp::Ordering;
use std::fmt;

struct Monomial {
    coefficient: f64,
    power_list: Vec<i32>,
}

impl Monomial {
    fn print_monomial(&self) -> String {
        let mut output: String = String::from("");
        for (ind, &power) in self.power_list.iter().enumerate() {
            if ind == 0 && power > 0 {
                output.push_str(&String::from("x^"));
                output.push_str(&format!("{power}"));
            }
            if ind == 1 && power > 0 {
                output.push_str(&String::from("y^"));
                output.push_str(&format!("{power}"));
            }
            if ind == 2 && power > 0 {
                output.push_str(&String::from("z^"));
                output.push_str(&format!("{power}"));
            }
        }
        format!("{}{}", self.coefficient, output)
        // println!("{}{}",self.coefficient, output);
    }
}

impl Ord for Monomial {
    fn cmp(&self, other: &Self) -> Ordering {
        for ind in 0..self.power_list.len() {
            let power_a = self.power_list[ind];
            let power_b = other.power_list[ind];

            if power_a < power_b {
                return Ordering::Less;
            }
            if power_a > power_b {
                return Ordering::Greater;
            }
        }
        if self.coefficient < other.coefficient {
            return Ordering::Less;
        }
        if self.coefficient > other.coefficient {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Monomial {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Monomial {
    fn eq(&self, other: &Self) -> bool {
        if self.coefficient != other.coefficient {
            return false;
        }
        if self.power_list != other.power_list {
            return false;
        }
        true
    }
}

impl Eq for Monomial {}

impl fmt::Display for Monomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let expr = self.print_monomial();
        write!(f, "{}", expr)
    }
}

struct Polynomial {
    monomials: Vec<Monomial>,
}

impl Polynomial {
    fn print_polynomial(&self) {
        for monomial in &self.monomials {
            monomial.print_monomial();
        }
    }
}

fn main() {
    println!("Hello world");
    let monomial_a = Monomial {
        coefficient: 5.0,
        power_list: vec![1, 0, 0],
    };
    let monomial_b = Monomial {
        coefficient: 5.0,
        power_list: vec![0, 1, 0],
    };
    // println!("{}", monomial_b);
    println!(
        "{} < {}: {}",
        monomial_a,
        monomial_b,
        monomial_a < monomial_b
    );
}
