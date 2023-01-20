use crate::polynomial::Polynomial;

#[derive(Debug)]
pub struct RationalPoly {
    pub numer: Polynomial,
    pub denom: Polynomial,
}

impl RationalPoly {
    pub fn new(numer: Polynomial, denom: Polynomial) -> Self {
        RationalPoly { numer, denom }
    }

    pub fn expr(&self) -> String {
        let mut output = String::from("");
        output.push_str("(");
        output.push_str(&self.numer.expr());
        output.push_str(")/(");
        output.push_str(&self.denom.expr());
        output.push_str(")");
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_rational_poly_expr() {
        let numer = Polynomial::from("x^3 + x").unwrap();
        let denom = Polynomial::from("x + 1").unwrap();

        let rational = RationalPoly::new(numer, denom);
        assert_eq!(rational.expr(), "(x^3 + x)/(x + 1)");
    }
}
