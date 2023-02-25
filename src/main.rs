extern crate log;
extern crate poly;

use num::complex::Complex;
use poly::interpreter::run_interpreter;
use poly::monomial::Monomial;
use poly::polynomial::Polynomial;

fn main() {
    env_logger::init();
    run_interpreter();
}
