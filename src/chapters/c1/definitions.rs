use std::{
    fmt::{Debug, Display}, ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub}, process::Output, slice::SliceIndex
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
    type Output = Complex<T>;

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
    T: Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (a, b) in self.elems.iter_mut().zip(rhs.elems.into_iter()) {
            *a = *a + b;
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

// 1B Vector space
// Vector space V

// Scalar multiplication
pub trait MulScalar<T> {
    fn mul(self, rhs: T) -> Self;
}

impl<T, const N: usize> MulScalar<T> for List<T, N> where T: Mul<Output = T> + Copy {
    fn mul(self, rhs: T) -> Self {
        Self { 
            elems: self.elems.map(|x| x * rhs)
        }
    }
}



// u + v = v + u
pub trait Commutative : Add<Output = Self> + Sized {}
impl<T> Commutative for Complex<T> where T: Commutative {}
impl<T, const N: usize> Commutative for List<T, N> where T: Commutative, T: Copy {}
impl Commutative for f64 {}

// (u + v) + w = u + (v + w)
pub trait Associative : Add<Output = Self> + Sized {}
impl<T> Associative for Complex<T> where T: Associative {}
impl<T, const N: usize> Associative for List<T, N> where T: Associative, T: Copy {}
impl Associative for f64 {}

// there exists an element 0 so that v+0 is v for all v in V
pub trait Identity : Add<Output = Self> + Zero + Sized {}
impl<T> Identity for Complex<T> where T: Identity {}
impl<T, const N: usize> Identity for List<T, N> where T: Identity + Copy {}
impl Identity for f64 {}

// For every v there exists a w so that v + w is 0
pub trait Inverse : Add<Output = Self> + Neg<Output = Self> + Sized {}
impl<T> Inverse for Complex<T> where T: Inverse {}
impl<T, const N: usize> Inverse for List<T, N> where T: Inverse + Copy {}
impl Inverse for f64 {}

// 1 * v = v for all v
pub trait MulIdent<X> : MulScalar<X> + Sized {}
impl<T> MulIdent<T> for Complex<T> where T: One, Self: MulScalar<T> {}
impl<T, const N: usize> MulIdent<T> for List<T, N> where T: One, Self: MulScalar<T>,  {}

// a(u + v) = au + av and (a+b)v = av + bv for all a, b in F and u, v in V
// This one is hard to implement
pub trait Distributive<X>  where Self: Sized, Self: MulScalar<X>  {}
impl<T, X> Distributive<X> for Complex<T> where Self: MulScalar<X> {}
impl<T, X, const N: usize> Distributive<X> for List<T, N> where Self: MulScalar<X> {}

// A Vector Space V over F
pub trait Field<F> {}
impl<V, F> Field<F> for V where 
    V: Commutative,
    V: Associative,
    V: Identity,
    V: Inverse,
    V: MulIdent<F>,
    V: Distributive<F> {}





#[cfg(test)]
mod test {
    use super::{Field, List};

    #[test]
    fn scratch_pad() {

        accept_field::<List<f64, 3>, f64>(); // Oke
        // accept_field::<super::Complex<f64>, f64>() // Error required for `chapters::c1::definitions::Complex<f64>` to implement `chapters::c1::definitions::MulIdent<f64>` -> Complexe numbers kunnen niet gemultiplied worden met f64s en missen dus een multiplicative identity
    }

    fn accept_field<T: Field<X>, X>() {}
}

