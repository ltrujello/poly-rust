use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
struct Monomial {
    coefficient: f64,
    power_list: Vec<i32>,
}

impl Monomial {
    fn expr(&self) -> String {
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
    }

    fn cmp_terms(&self, other: &Self) -> Ordering {
        let degree_a: i32 = self.power_list.iter().sum();
        let degree_b: i32 = other.power_list.iter().sum();

        if degree_a < degree_b {
            return Ordering::Less;
        }

        if degree_a > degree_b {
            return Ordering::Greater;
        }

        for ind in (0..self.power_list.len()).rev() {
            let power_a = self.power_list[ind];
            let power_b = other.power_list[ind];

            if power_a < power_b {
                return Ordering::Greater;
            }
            if power_a > power_b {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl Ord for Monomial {
    // http://pi.math.cornell.edu/~dmehrle/notes/old/alggeo/07MonomialOrdersandDivisionAlgorithm.pdf
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cmp_terms(other) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                if self.coefficient < other.coefficient {
                    return Ordering::Less;
                }
                if self.coefficient > other.coefficient {
                    return Ordering::Greater;
                }
                Ordering::Equal
            }
        }
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
        let expr = self.expr();
        write!(f, "{}", expr)
    }
}

struct Polynomial {
    monomials: Vec<Monomial>,
}

impl Polynomial {
    fn print_polynomial(&self) {
        for monomial in &self.monomials {
            monomial.expr();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        // xyz == xyz, 5xyz == 5xyz
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);

        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: 5.0,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[test]
    fn test_ordering_a() {
        // 6x > 5x
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![1, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![1, 0, 0],
        };
        assert!(monomial_a < monomial_b);
    }

    #[test]
    fn test_ordering_b() {
        // x^2 > xy
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 0],
        };
        assert!(monomial_a > monomial_b);
    }

    #[test]
    fn test_ordering_c() {
        // xy > y^2
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 2, 0],
        };
        assert!(monomial_a > monomial_b);
    }

    #[test]
    fn test_ordering_d() {
        // y^2 > xz
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 2, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[test]
    fn test_ordering_e() {
        // xz > yz
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[test]
    fn test_ordering_f() {
        // yz > z^2
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 0, 2],
        };
        assert!(monomial_a > monomial_b);
    }
}

fn main() {
    println!("Hello world");
    let monomial_a = Monomial {
        coefficient: 5.0,
        power_list: vec![2, 0, 0],
    };
    let monomial_b = Monomial {
        coefficient: 5.0,
        power_list: vec![1, 1, 0],
    };
    let monomial_c = Monomial {
        coefficient: 5.0,
        power_list: vec![0, 2, 0],
    };
    println!(
        "{} > {}: {}",
        monomial_a,
        monomial_b,
        monomial_a > monomial_b
    );
    println!(
        "{} > {}: {}",
        monomial_b,
        monomial_c,
        monomial_b > monomial_c
    );
}
