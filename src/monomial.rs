use crate::parser::{Parser, ParserErr};
use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Monomial {
    pub coefficient: f64,
    pub power_list: Vec<i32>,
}

impl Monomial {
    pub fn cmp_terms(&self, other: &Self) -> Ordering {
        let degree_a: i32 = self.degree();
        let degree_b: i32 = other.degree();

        if degree_a > degree_b {
            return Ordering::Less;
        }

        if degree_a < degree_b {
            return Ordering::Greater;
        }

        let max_len = std::cmp::max(self.power_list.len(), other.power_list.len());
        for ind in 0..max_len {
            let power_a = self.power(ind);
            let power_b = other.power(ind);

            if power_a < power_b {
                return Ordering::Greater;
            }
            if power_a > power_b {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }

    pub fn expr(&self) -> String {
        let mut output: String = String::from("");
        if self.coefficient == 1.0 {
            output.push_str(&format!("{}", self.term_expr()));
        } else {
            output.push_str(&format!("{}{}", self.coefficient, self.term_expr()));
        }
        output
    }

    pub fn term_expr(&self) -> String {
        let mut output: String = String::from("");
        for (ind, &power) in self.power_list.iter().enumerate() {
            if ind == 0 && power > 0 {
                if power == 1 {
                    output.push_str(&format!("x"));
                } else {
                    output.push_str(&format!("x^{power}"));
                }
            }

            if ind == 1 && power > 0 {
                if power == 1 {
                    output.push_str(&format!("y"));
                } else {
                    output.push_str(&format!("y^{power}"));
                }
            }

            if ind == 2 && power > 0 {
                if power == 1 {
                    output.push_str(&format!("z"));
                } else {
                    output.push_str(&format!("z^{power}"));
                }
            }
        }
        output
    }

    pub fn power(&self, ind: usize) -> i32 {
        let power: i32;
        match self.power_list.get(ind) {
            Some(res) => power = *res,
            None => power = 0,
        }
        power
    }

    pub fn degree(&self) -> i32 {
        self.power_list.iter().sum()
    }

    pub fn from(expr: &str) -> Result<Monomial, ParserErr> {
        let mut parser = Parser::parser_init(String::from(expr));
        let monomial = parser.parse_monomial()?;
        Ok(monomial)
    }
}

impl Clone for Monomial {
    fn clone(&self) -> Self {
        Monomial {
            coefficient: self.coefficient,
            power_list: self.power_list.clone(),
        }
    }
}

impl Ord for Monomial {
    // http://pi.math.cornell.edu/~dmehrle/notes/old/alggeo/07MonomialOrdersandDivisionAlgorithm.pdf
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cmp_terms(other) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            // Break tie by comparing coeffs
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

// Monomial * Monomial
impl ops::Mul for Monomial {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let coefficient = self.coefficient * other.coefficient;
        let mut power_list;
        if coefficient != 0.0 {
            let max_len = std::cmp::max(self.power_list.len(), other.power_list.len());
            power_list = vec![0; max_len];
            for ind in 0..max_len {
                let power_a = self.power(ind);
                let power_b = other.power(ind);

                power_list[ind] = power_a + power_b;
            }
        } else {
            power_list = vec![0; self.power_list.len().try_into().unwrap()];
        }
        Monomial {
            coefficient: coefficient,
            power_list: power_list,
        }
    }
}

// f64 * Monomial
impl ops::Mul<Monomial> for f64 {
    type Output = Monomial;

    fn mul(self, other: Monomial) -> Monomial {
        let mut monomial = other.clone();
        monomial.coefficient *= self;
        monomial
    }
}

// Monomial * f64
impl ops::Mul<f64> for Monomial {
    type Output = Monomial;

    fn mul(self, other: f64) -> Monomial {
        other * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn monomial_a() -> Monomial {
        Monomial::from("xyz").unwrap()
    }

    #[fixture]
    fn monomial_b() -> Monomial {
        Monomial::from("3.5x^2yz^5").unwrap()
    }

    #[fixture]
    fn monomial_c() -> Monomial {
        Monomial::from("5y^2x").unwrap()
    }

    #[fixture]
    fn monomial_d() -> Monomial {
        Monomial::from("100z^5y^2").unwrap()
    }

    #[rstest]
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

    #[rstest]
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

    #[rstest]
    fn test_ordering_b() {
        // xy > x^2
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_c() {
        // y^2 > xy
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 2, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 0],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
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

    #[rstest]
    fn test_ordering_e() {
        // yz > xz
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_f() {
        // z^2 > yz
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 0, 2],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_g() {
        // x^2yz^3 > x^5y
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 1, 3],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![5, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_h() {
        // x^2y > x^3
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![3],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_i() {
        // y > x
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_j() {
        // z > y
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 0, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_k() {
        // z^2 > y^2
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 0, 2],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 2],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_l() {
        // z > x
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![0, 0, 1],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1],
        };
        assert!(monomial_a > monomial_b);
    }

    #[rstest]
    fn test_ordering_m() {
        // x^2 < x
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![1, 0, 0],
        };
        assert!(monomial_a < monomial_b);
    }

    #[rstest]
    fn test_mul_a() {
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        let monomial_c = monomial_a * monomial_b;
        assert_eq!(monomial_c.coefficient, 30.0);
        assert_eq!(monomial_c.power_list, [2, 2, 0]);
    }

    #[rstest]
    fn test_mul_b() {
        let monomial_a = Monomial {
            coefficient: 5.0,
            power_list: vec![2],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 3],
        };
        let monomial_c = monomial_a * monomial_b;
        assert_eq!(monomial_c.coefficient, 30.0);
        assert_eq!(monomial_c.power_list, [2, 2, 3]);
    }

    #[rstest]
    fn test_mul_zero() {
        let monomial_a = Monomial {
            coefficient: 0.0,
            power_list: vec![0, 0, 0],
        };
        let monomial_b = Monomial {
            coefficient: 6.0,
            power_list: vec![0, 2, 0],
        };
        let monomial_c = monomial_a * monomial_b;
        assert_eq!(monomial_c.coefficient, 0.0);
        assert_eq!(monomial_c.power_list, [0, 0, 0]);
    }

    #[rstest]
    fn test_monomial_from_str_a(monomial_a: Monomial) {
        assert_eq!(monomial_a.coefficient, 1.0);
        assert_eq!(monomial_a.degree(), 3);
        assert_eq!(monomial_a.power(0), 1);
        assert_eq!(monomial_a.power(1), 1);
        assert_eq!(monomial_a.power(2), 1);
    }

    #[rstest]
    fn test_monomial_from_str_b(monomial_b: Monomial) {
        assert_eq!(monomial_b.coefficient, 3.5);
        assert_eq!(monomial_b.degree(), 8);
        assert_eq!(monomial_b.power(0), 2);
        assert_eq!(monomial_b.power(1), 1);
        assert_eq!(monomial_b.power(2), 5);
    }

    #[rstest]
    fn test_monomial_from_str_c(monomial_c: Monomial) {
        assert_eq!(monomial_c.coefficient, 5.0);
        assert_eq!(monomial_c.degree(), 3);
        assert_eq!(monomial_c.power(0), 1);
        assert_eq!(monomial_c.power(1), 2);
        assert_eq!(monomial_c.power(2), 0);
    }

    #[rstest]
    fn test_monomial_from_str_d(monomial_d: Monomial) {
        assert_eq!(monomial_d.coefficient, 100.0);
        assert_eq!(monomial_d.degree(), 7);
        assert_eq!(monomial_d.power(0), 0);
        assert_eq!(monomial_d.power(1), 2);
        assert_eq!(monomial_d.power(2), 5);
    }

    #[rstest]
    fn test_monomial_from_str_expected_expr_a(monomial_a: Monomial) {
        assert_eq!(monomial_a.expr().as_str(), "xyz");
    }

    #[rstest]
    fn test_monomial_from_str_expected_expr_b(monomial_b: Monomial) {
        assert_eq!(monomial_b.expr().as_str(), "3.5x^2yz^5");
    }

    #[rstest]
    fn test_monomial_from_str_expected_expr_c(monomial_c: Monomial) {
        assert_eq!(monomial_c.expr().as_str(), "5xy^2");
    }

    #[rstest]
    fn test_monomial_from_str_expected_expr_d(monomial_d: Monomial) {
        assert_eq!(monomial_d.expr().as_str(), "100y^2z^5");
    }
}
