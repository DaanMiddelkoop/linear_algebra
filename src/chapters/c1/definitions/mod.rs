use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    process::Output,
};

pub struct Complex<T> {
    a: T,
    b: T,
}

impl<T> Debug for Complex<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} + {:?}i)", self.a, self.b)
    }
}

impl<T> Display for Complex<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}i)", self.a, self.b)
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T>,
    T: Sub<Output = T>,
    T: Add<Output = T>,
    T: Copy,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            a: (self.a * rhs.a) - (self.b * rhs.b),
            b: (self.a * rhs.b) + (self.b * rhs.a),
        }
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            a: -self.a,
            b: -self.b,
        }
    }
}

#[allow(clippy::eq_op)]
impl<T> Complex<T>
where
    T: Div<Output = T> + Copy,
{
    pub fn inverse(self) -> Self {
        Self {
            a: self.a / self.a / self.a,
            b: self.b / self.b / self.b,
        }
    }
}

impl<T> Div for Complex<T>
where
    T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<T> Sub for Complex<T>
where
    T: Neg<Output = T>,
    T: Add<Output = T>,
{
    type Output = Complex<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

// pow
