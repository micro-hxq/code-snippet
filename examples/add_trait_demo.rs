use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self
        where T: Add<T, Output=T>,
    {
        Complex { re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<T, Output=T>,
{
    type Output = Self;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}


fn main() {
    let c1 = Complex::new(1, 2);
    let c2 = Complex::new(3, 4);
    let c3 = c1 + c2;
    println!("{:?}", c3);
    assert_eq!(c3, Complex::new(4, 6));
    assert!(c3.re == 4 && c3.im == 6);
}