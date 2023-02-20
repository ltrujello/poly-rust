use std::clone::Clone;
use std::cmp::Ordering;
use std::fmt;
use std::ops;

use crate::CRing;
use log::info;

pub type Monomial32 = Monomial<f32>;
pub type Monomial64 = Monomial<f64>;

#[derive(Debug)]
pub struct Monomial<T: CRing> {
    pub coefficient: T,
    pub power_list: Vec<i32>,
}

impl<T: CRing + std::fmt::Display> Monomial<T> {
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
        if self.coefficient.is_one() {
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

    // pub fn from(expr: &str) -> Result<Monomial, ParserErr> {
    //     let mut parser = Parser::parser_init(String::from(expr))?;
    //     let monomial = parser.parse_monomial()?;
    //     Ok(monomial)
    // }
}

impl<T> std::fmt::Display for Monomial<T>
where
    T: std::fmt::Display + CRing + Clone + num::Signed,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut term_expr = format!("");
        for (ind, &power) in self.power_list.iter().enumerate() {
            let mut letter = "";
            let mut exp = format!("");
            if power > 0 {
                if ind == 0 {
                    letter = "x";
                }
                if ind == 1 {
                    letter = "y";
                }
                if ind == 2 {
                    letter = "z";
                }
                if power != 1 {
                    exp = format!("^{power}");
                }
            }
            term_expr.push_str(&format!("{}{}", letter, exp));
        }

        let mut coeff = format!("");
        if term_expr.is_empty() {
            coeff = format!("{}", self.coefficient)
        } else {
            let mut abs_coeff = num::abs(self.coefficient.clone());

            if self.coefficient.is_negative() {
                coeff.push_str("-");
            }

            if !abs_coeff.is_one() {
                coeff.push_str(&format!("{}", self.coefficient));
            }
        }

        write!(f, "{}{}", coeff, term_expr)
    }
}

impl<T: CRing + Clone> Clone for Monomial<T> {
    fn clone(&self) -> Self {
        Monomial {
            coefficient: self.coefficient.clone(),
            power_list: self.power_list.clone(),
        }
    }
}

impl<T: CRing> Ord for Monomial<T> {
    // http://pi.math.cornell.edu/~dmehrle/notes/old/alggeo/07MonomialOrdersandDivisionAlgorithm.pdf
    fn cmp(&self, other: &Self) -> Ordering {
        match self.cmp_terms(other) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            // Break tie by comparing coeffs
            Ordering::Equal => {
                // if self.coefficient < other.coefficient {
                //     return Ordering::Less;
                // }
                // if self.coefficient > other.coefficient {
                //     return Ordering::Greater;
                // }
                Ordering::Equal
            }
        }
    }
}

impl<T: CRing> PartialOrd for Monomial<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: CRing> PartialEq for Monomial<T> {
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

impl<T: CRing> Eq for Monomial<T> {}

// Monomial* Monomial
impl<T: CRing + Clone> ops::Mul for Monomial<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let coefficient = self.coefficient.clone() * other.coefficient.clone();
        let mut power_list;
        if coefficient.is_zero() {
            power_list = vec![0; self.power_list.len().try_into().unwrap()];
        } else {
            let max_len = std::cmp::max(self.power_list.len(), other.power_list.len());
            power_list = vec![0; max_len];
            for ind in 0..max_len {
                let power_a = self.power(ind);
                let power_b = other.power(ind);

                power_list[ind] = power_a + power_b;
            }
        }
        Monomial {
            coefficient: coefficient,
            power_list: power_list,
        }
    }
}

// // f64 * Monomial
// impl<T> ops::Mul<Monomial<T>> for T
// where T: CRing
// {
//     type Output = Monomial<T>;

//     fn mul(self, other: Monomial<T>) -> Monomial<T> {
//         let mut monomial = other.clone();
//         monomial.coefficient = self * monomial.coefficient;
//         monomial
//     }
// }

// Monomial* f64
impl<T: CRing + Clone> ops::Mul<T> for Monomial<T> {
    type Output = Monomial<T>;

    fn mul(self, other: T) -> Monomial<T> {
        let mut monomial = self.clone();
        monomial.coefficient = other * monomial.coefficient;
        monomial
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;
    use num::rational::Ratio;
    use rstest::*;

    #[fixture]
    fn monomial_a() -> Monomial64 {
        return Monomial {
            coefficient: 1.0,
            power_list: vec![1, 1, 1],
        };
    }

    #[fixture]
    fn monomial_b() -> Monomial64 {
        return Monomial {
            coefficient: 3.5,
            power_list: vec![2, 1, 5],
        };
    }

    #[fixture]
    fn monomial_c() -> Monomial64 {
        return Monomial {
            coefficient: 5.0,
            power_list: vec![1, 2, 0],
        };
    }

    #[fixture]
    fn monomial_d() -> Monomial64 {
        return Monomial {
            coefficient: 100.0,
            power_list: vec![0, 2, 5],
        };
    }

    #[rstest]
    fn test_equality_i32() {
        let coefficient: i32 = 2;
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[rstest]
    fn test_equality_i64() {
        let coefficient: i64 = 2;
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[rstest]
    fn test_equality_f32() {
        let coefficient: f32 = 2.0;
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[rstest]
    fn test_equality_f64() {
        let coefficient: f64 = 2.0;
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[rstest]
    fn test_equality_complex() {
        let coefficient = Complex::new(2, 3);
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    #[rstest]
    fn test_equality_rational() {
        let coefficient = Ratio::new(2, 3);
        let monomial_a = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        let monomial_b = Monomial {
            coefficient: coefficient,
            power_list: vec![1, 1, 1],
        };
        assert_eq!(monomial_a, monomial_b);
    }

    // #[rstest]
    // fn test_ordering_a() {
    //     // 6x > 5x
    //     let monomial_a = Monomial {
    //         coefficient: 5.0,
    //         power_list: vec![1, 0, 0],
    //     };
    //     let monomial_b = Monomial {
    //         coefficient: 6.0,
    //         power_list: vec![1, 0, 0],
    //     };
    //     assert!(monomial_a < monomial_b);
    // }

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
    fn test_ordering_n() {
        // x^2y^2 < x^3
        let monomial_a = Monomial {
            coefficient: 1.0,
            power_list: vec![2, 2, 0],
        };
        let monomial_b = Monomial {
            coefficient: 1.0,
            power_list: vec![3, 0, 0],
        };
        assert!(monomial_a < monomial_b);
    }

    #[rstest]
    fn test_monomial_from_str_a(monomial_a: Monomial64) {
        assert_eq!(monomial_a.coefficient, 1.0);
        assert_eq!(monomial_a.degree(), 3);
        assert_eq!(monomial_a.power(0), 1);
        assert_eq!(monomial_a.power(1), 1);
        assert_eq!(monomial_a.power(2), 1);
    }

    #[rstest]
    fn test_monomial_from_str_b(monomial_b: Monomial64) {
        assert_eq!(monomial_b.coefficient, 3.5);
        assert_eq!(monomial_b.degree(), 8);
        assert_eq!(monomial_b.power(0), 2);
        assert_eq!(monomial_b.power(1), 1);
        assert_eq!(monomial_b.power(2), 5);
    }

    #[rstest]
    fn test_monomial_from_str_c(monomial_c: Monomial64) {
        assert_eq!(monomial_c.coefficient, 5.0);
        assert_eq!(monomial_c.degree(), 3);
        assert_eq!(monomial_c.power(0), 1);
        assert_eq!(monomial_c.power(1), 2);
        assert_eq!(monomial_c.power(2), 0);
    }

    #[rstest]
    fn test_monomial_from_str_d(monomial_d: Monomial64) {
        assert_eq!(monomial_d.coefficient, 100.0);
        assert_eq!(monomial_d.degree(), 7);
        assert_eq!(monomial_d.power(0), 0);
        assert_eq!(monomial_d.power(1), 2);
        assert_eq!(monomial_d.power(2), 5);
    }

    #[rstest]
    fn test_monomial_negate(monomial_a: Monomial64) {
        let res = monomial_a * -1.0;
        assert_eq!(res.coefficient, -1.0);
        assert_eq!(res.degree(), 3);
        assert_eq!(res.power(0), 1);
        assert_eq!(res.power(1), 1);
        assert_eq!(res.power(2), 1);
    }

    #[rstest]
    fn test_monomial_display(monomial_a: Monomial64) {
        let monomial_a = Monomial {
            coefficient: -1.0,
            power_list: vec![1, 0, 0],
        };
        assert_eq!("-x", format!("{}", monomial_a).as_str());
    }
}
