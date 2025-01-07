use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub},
    slice::SliceIndex,
};

#[derive(Clone, Copy)]
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
    T: Div<Output = T> + One,
{
    pub fn inverse(self) -> Self {
        Self {
            a: T::ONE / self.a,
            b: T::ONE / self.b,
        }
    }
}

impl<T> Div for Complex<T>
where
    T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + One + Copy,
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

// Lists
#[derive(Clone, Copy)]
pub struct List<T, const N: usize> {
    elems: [T; N],
}

impl<T, const N: usize, Idx> Index<Idx> for List<T, N>
where
    Idx: SliceIndex<[T], Output = T>,
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        self.elems.index(index)
    }
}

impl<T, const N: usize> Add for List<T, N>
where
    T: AddAssign,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (a, b) in self.elems.iter_mut().zip(rhs.elems.into_iter()) {
            *a += b;
        }
        self
    }
}

pub trait Zero {
    const ZERO: Self;
}

impl Zero for usize {
    const ZERO: Self = 0;
}

impl Zero for f64 {
    const ZERO: Self = 0.0;
}

impl Zero for isize {
    const ZERO: Self = 0;
}

impl<T> Zero for Complex<T>
where
    T: Zero,
{
    const ZERO: Self = Complex {
        a: T::ZERO,
        b: T::ZERO,
    };
}

impl<T, const N: usize> Zero for List<T, N>
where
    T: Zero,
{
    const ZERO: Self = Self {
        elems: [T::ZERO; N],
    };
}

pub trait One {
    const ONE: Self;
}

impl One for usize {
    const ONE: Self = 1;
}

impl One for isize {
    const ONE: Self = 1;
}

impl One for f64 {
    const ONE: Self = 1.0;
}

impl<T> One for Complex<T>
where
    T: One,
{
    const ONE: Self = Complex {
        a: T::ONE,
        b: T::ONE,
    };
}

impl<T, const N: usize> One for List<T, N>
where
    T: One,
{
    const ONE: Self = Self { elems: [T::ONE; N] };
}

impl<T, const N: usize> Neg for List<T, N>
where
    T: Neg<Output = T>,
{
    type Output = List<T, N>;

    fn neg(self) -> Self::Output {
        Self {
            elems: self.elems.map(|x| -x),
        }
    }
}

impl<T, const N: usize> Mul<T> for List<T, N>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = List<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            elems: self.elems.map(|x| x * rhs),
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn scratch_pad() {
        let list = List { elems: [1, 1, 1] };
        let x = list * 5_isize;
    }
}
