use super::int::*;
use num_traits::*;
use std::hash::Hasher;
use std::marker::*;
use std::cmp::*;
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;
use std::ops::{Neg, Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait Modulus<N: Int>: Copy {
  fn value() -> N;

  fn inv(n: N) -> N {
    n.inv_mod(Self::value())
  }
}

#[macro_export]
macro_rules! modulus {
  ($name:ident, $modulus:expr) => {
    #[derive(Clone, Copy)]
    pub struct $name;
    impl<N: PrimInt> Modulus<N> for $name {
      fn value() -> N {
        N::from($modulus).unwrap()
      }
    }
  }
}

modulus!(Mod1000000007, 1000000007);
modulus!(Mod998244353, 998244353);

pub type ModInt1000000007<N> = ModInt<N, Mod1000000007>;
pub type ModInt998244353<N> = ModInt<N, Mod998244353>;

#[derive(Debug, Clone, Copy, Eq)]
pub struct ModInt<N: Int, M: Modulus<N>> {
  value: N,
  modulus: PhantomData<M>,
}

impl<N: Int, M: Modulus<N>> ModInt<N, M> {
  fn raw(value: N) -> Self {
    Self {
      value,
      modulus: PhantomData,
    }
  }

  pub fn new(value: N) -> Self {
    Self::raw((value % M::value() + M::value()) % M::value())
  }

  pub fn value(self) -> N {
    self.value
  }

  pub fn modulus(&self) -> N {
    M::value()
  }

  pub fn inv(self) -> Self {
    Self::raw(M::inv(self.value()))
  }
}

impl<N: Int + fmt::Display, M: Modulus<N>> fmt::Display for ModInt<N, M> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
    self.value().fmt(f)
  }
}

impl<N: Int, M: Modulus<N>> From<N> for ModInt<N, M> {
  fn from(this: N) -> Self {
    Self::new(this)
  }
}

impl<N: Int, M: Modulus<N>> PartialEq for ModInt<N, M> {
  fn eq(&self, other: &Self) -> bool {
    self.value().eq(&other.value())
  }
}

impl<N: Int, M: Modulus<N>> PartialEq<N> for ModInt<N, M> {
  fn eq(&self, other: &N) -> bool {
    self.value().eq(&other)
  }
}

impl<N: Int, M: Modulus<N>> Rem<N> for ModInt<N, M> {
  type Output = N;
  fn rem(self, _: N) -> N {
    self.value()
  }
}

impl<N: Int, M: Modulus<N>> Rem for ModInt<N, M> {
  type Output = Self;
  fn rem(self, _: Self) -> Self {
    self
  }
}

impl<N: Int, M: Modulus<N>> Zero for ModInt<N, M> {
  fn zero() -> Self {
    Self::raw(N::zero())
  }

  fn is_zero(&self) -> bool {
    self.value().is_zero()
  }
}

impl<N: Int, M: Modulus<N>> One for ModInt<N, M> {
  fn one() -> Self {
    Self::raw(N::one())
  }

  fn is_one(&self) -> bool {
    self.value().is_one()
  }
}

impl<N: Int, M: Modulus<N>> Neg for ModInt<N, M> {
  type Output = Self;
  fn neg(self) -> Self {
    Self::raw(M::value() - self.value())
  }
}

macro_rules! impl_add {
  ($N:ident, $M:ident, $Rhs:ty) => {
    impl<$N: Int, $M: Modulus<$N>> Add<$Rhs> for ModInt<$N, $M> {
      type Output = Self;
      fn add(self, other: $Rhs) -> Self {
        let mut x = self.value() + other % M::value();
        if x > $M::value() {
          x = x - $M::value();
        }
        Self::raw(x)
      }
    }
  }
}

macro_rules! impl_sub {
  ($N:ident, $M:ident, $Rhs:ty) => {
    impl<$N: Int, $M: Modulus<$N>> Sub<$Rhs> for ModInt<$N, $M> {
      type Output = Self;
      fn sub(self, other: $Rhs) -> Self {
        let mut x = self.value();
        let y = other % M::value();
        if x < y {
          x = x + $M::value();
        }
        Self::raw(x + y)
      }
    }
  }
}

macro_rules! impl_mul {
  ($N:ident, $M:ident, $Rhs:ty) => {
    impl<$N: Int, $M: Modulus<$N>> Mul<$Rhs> for ModInt<$N, $M> {
      type Output = Self;
      fn mul(self, other: $Rhs) -> Self {
        let mut x = self.value() * (other % M::value());
        if x < N::zero() {
          x = x + $M::value();
        }
        Self::raw(x)
      }
    }
  }
}

macro_rules! impl_div {
  ($N:ident, $M:ident, $Rhs:ty) => {
    impl<$N: Int, $M: Modulus<$N>> Div<$Rhs> for ModInt<$N, $M> {
      type Output = Self;
      fn div(self, other: $Rhs) -> Self {
        let mut x = self.value() * M::inv(other % M::value());
        if x < N::zero() {
          x = x + $M::value();
        }
        Self::raw(x)
      }
    }
  }
}

impl_add!(N, M, N);
impl_add!(N, M, ModInt<N, M>);

impl_sub!(N, M, N);
impl_sub!(N, M, ModInt<N, M>);

impl_mul!(N, M, N);
impl_mul!(N, M, ModInt<N, M>);

impl_div!(N, M, N);
impl_div!(N, M, ModInt<N, M>);

impl<N: Int, M: Modulus<N>> Num for ModInt<N, M> {
  type FromStrRadixErr = N::FromStrRadixErr;

  fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
      Ok(Self::new(N::from_str_radix(str, radix)?))
  }
}

macro_rules! impl_pow {
  ($N:ident, $M:ident, $Rhs:ty) => {
    impl<$N: Int, $M: Modulus<$N>> Pow<$Rhs> for ModInt<$N, $M> {
      type Output = Self;
      fn pow(self, mut e: $Rhs) -> Self {
        let mut a = self;
        let mut b = Self::one();
        while !e.is_zero() {
          if (e % (<$Rhs>::one() + <$Rhs>::one())).is_one() {
            a = a * b;
          }
          b = b * b;
          e = e - <$Rhs>::one();
        }
        a
      }
    }
  }
}

impl_pow!(N, M, N);
impl_pow!(N, M, ModInt<N, M>);

impl<N: Int + FromStr, M: Modulus<N>> FromStr for ModInt<N, M> {
  type Err = <N as FromStr>::Err;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
      Ok(Self::new(N::from_str(str)?))
  }
}

macro_rules! impl_assign {
  ($N:ident, $M:ident, $Rhs:ty, $method:ident, $assign_trait:ident, $assign_method:ident) => {
    impl<$N: Int, $M: Modulus<N>> $assign_trait<$Rhs> for ModInt<$N, $M> {
      fn $assign_method(&mut self, other: $Rhs) {
        *self = self.$method(other)
      }
    }
  }
}

impl_assign!(N, M, N, add, AddAssign, add_assign);
impl_assign!(N, M, ModInt<N, M>, add, AddAssign, add_assign);

impl_assign!(N, M, N, sub, SubAssign, sub_assign);
impl_assign!(N, M, ModInt<N, M>, sub, SubAssign, sub_assign);

impl_assign!(N, M, N, mul, MulAssign, mul_assign);
impl_assign!(N, M, ModInt<N, M>, mul, MulAssign, mul_assign);

impl_assign!(N, M, N, div, DivAssign, div_assign);
impl_assign!(N, M, ModInt<N, M>, div, DivAssign, div_assign);

impl<N: Int + Hash, M: Modulus<N>> Hash for ModInt<N, M> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value().hash(state);
  }
}

impl<N: Int, M: Modulus<N>> PartialOrd for ModInt<N, M> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value().partial_cmp(&other.value())
  }
}
