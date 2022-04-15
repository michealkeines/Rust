use std::ops::Add;
use std::convert::From;
use std::fmt::{Formatter, Display, Result};


#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> From<(T,T)> for Complex<T> {
    fn from(value: (T, T)) -> Complex<T> {
        Complex { re: value.0, im: value.1 }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex { re: self.re + rhs.re, im: self.im + rhs.im }
    }
}



#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn complex_basics() {
        let first = Complex::new(3,5);
        let second: Complex<i32> = Complex::default();
        assert_eq!(first.re, 3);
        assert_eq!(first.im, 5);
        println!("{} {}", second.re, second.im);
        assert!(second.re == second.im);
    }
     #[test]
    fn complex_addition() {
        let a = Complex::new(1,-2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_form() {
        let a = (12,23);
        let complex = Complex::from(a);
        assert_eq!(complex.re, 12);
        assert_eq!(complex.im, 23);
    }

    #[test]
    fn complex_display() {
        let my_imaginary = Complex::new(12, 34);
        println!("{}", my_imaginary);
    }
}
