use super::*;
use std::ops::*;
use acl_modint::ModIntBase;
use fft::*;
use num_traits::*;

macro_rules! derive_op {
  ($Self:ty, $Rhs:ty, [$($trait:tt)*], $op:ident, [$($trait_assign:tt)*], $op_assign:ident, $T:ident, $C:ident, [$($cond:tt)*]) => {

    // FPS ?= X
    impl<$T, $C> $($trait_assign)*<$Rhs> for $Self where $($cond)* {
      fn $op_assign(&mut self, other: $Rhs) {
        self.$op_assign(&other);
      }
    }

    // &FPS ? &X
    impl<$T, $C> $($trait)*<&$Rhs> for &$Self where $($cond)* {
      type Output = $Self;
      fn $op(self, other: &$Rhs) -> $Self {
        let mut f: $Self = self.clone();
        f.$op_assign(other);
        f
      }
    }

    // &FPS ? X
    impl<$T, $C> $($trait)*<$Rhs> for &$Self where $($cond)* {
      type Output = $Self;
      fn $op(self, other: $Rhs) -> $Self {
        let mut f = self.clone();
        f.$op_assign(&other);
        f
      }
    }

    // FPS ? &X
    impl<$T, $C> $($trait)*<&$Rhs> for $Self where $($cond)* {
      type Output = $Self;
      fn $op(mut self, other: &$Rhs) -> $Self {
        self.$op_assign(other);
        self
      }
    }

    // FPS ? X
    impl<$T, $C> $($trait)*<$Rhs> for $Self where $($cond)* {
      type Output = $Self;
      fn $op(mut self, other: $Rhs) -> $Self {
        self.$op_assign(&other);
        self
      }
    }
  }
}

// Addition
impl<T: Clone + From<u8> + std::ops::Add<Output = T>, C> std::ops::AddAssign<&FPS<T, C>> for FPS<T, C> {
  fn add_assign(&mut self, other: &FPS<T, C>) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] = self[i].clone() + other[i].clone();
    }
  }
}

// Scalar Addition
impl<T: Clone + std::ops::Add<Output = T>, C> std::ops::AddAssign<&T> for FPS<T, C> {
  fn add_assign(&mut self, other: &T) {
    self[0] = self[0].clone() + other.clone();
  }
}

// Subtraction
impl<T: Clone + From<u8> + std::ops::Sub<Output = T>, C> std::ops::SubAssign<&FPS<T, C>> for FPS<T, C> {
  fn sub_assign(&mut self, other: &FPS<T, C>) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] = self[i].clone() - other[i].clone();
    }
  }
}


// Scalar Subtraction
impl<T: Clone + std::ops::Sub<Output = T>, C> std::ops::SubAssign<&T> for FPS<T, C> {
  fn sub_assign(&mut self, other: &T) {
    self[0] = self[0].clone() - other.clone();
  }
}

// Multiplication
impl<T: Clone, C: Convolution<T> + Clone> std::ops::MulAssign<&FPS<T, C>> for FPS<T, C> {
  fn mul_assign(&mut self, other: &FPS<T, C>) {
    self.convolve(&other.clone());
  }
}

// Scalar Multiplication
impl<T: Clone + std::ops::Mul<Output = T>, C> std::ops::MulAssign<&T> for FPS<T, C> {
  fn mul_assign(&mut self, other: &T) {
    for i in 0 .. self.len() {
      self[i] = self[i].clone() * other.clone();
    }
  }
}

// Division
impl<T: Clone + PartialEq + From<u8> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>, C: Clone + Convolution<T>> std::ops::DivAssign<&FPS<T, C>> for FPS<T, C> {
  fn div_assign(&mut self, other: &FPS<T, C>) {
    if self.len() < other.len() {
      self.clear();
    } else {
      let n = self.len() - other.len() + 1;
      *self = (&self.rev().pre(n) * &other.rev().inv_at(n)).pre(n).rev_at(n);
    }
  }
}

// Scalar Division
impl<T: Clone + std::ops::Div<Output = T>, C> std::ops::DivAssign<&T> for FPS<T, C> {
  fn div_assign(&mut self, other: &T) {
    for i in 0 .. self.len() {
      self[i] = self[i].clone() / other.clone();
    }
  }
}

// Remainder
impl<T: Clone + PartialEq + From<u8> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>, C: Clone + Convolution<T>> std::ops::RemAssign<&FPS<T, C>> for FPS<T, C> {
  fn rem_assign(&mut self, other: &FPS<T, C>) {
    self.shrink();
    *self -= &*self / other * other;
  }
}

// Negation
impl<T: Clone + std::ops::Neg<Output = T>, C> std::ops::Neg for FPS<T, C> {
  type Output = Self;
  fn neg(self) -> Self {
    self.map(|x, _| -x.clone())
  }
}

impl<T: Clone + std::ops::Neg<Output = T>, C> std::ops::Neg for &FPS<T, C> {
  type Output = FPS<T, C>;
  fn neg(self) -> FPS<T, C> {
    self.map(|x, _| -x.clone())
  }
}

// derivations
derive_op!(FPS<T, C>, FPS<T, C>, [std::ops::Add], add, [std::ops::AddAssign], add_assign, T, C, [T: Clone + From<u8> + std::ops::Add<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, FPS<T, C>, [std::ops::Sub], sub, [std::ops::SubAssign], sub_assign, T, C, [T: Clone + From<u8> + std::ops::Sub<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, FPS<T, C>, [std::ops::Mul], mul, [std::ops::MulAssign], mul_assign, T, C, [T: Clone, C: Clone + Convolution<T>]);
derive_op!(FPS<T, C>, FPS<T, C>, [std::ops::Div], div, [std::ops::DivAssign], div_assign, T, C, [T: Clone + PartialEq + From<u8> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>, C: Clone + Convolution<T>]);
derive_op!(FPS<T, C>, FPS<T, C>, [std::ops::Rem], rem, [std::ops::RemAssign], rem_assign, T, C, [T: Clone + PartialEq + From<u8> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Div<Output = T>, C: Clone + Convolution<T>]);
derive_op!(FPS<T, C>, T, [std::ops::Add], add, [std::ops::AddAssign], add_assign, T, C, [T: Clone + std::ops::Add<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, [std::ops::Sub], sub, [std::ops::SubAssign], sub_assign, T, C, [T: Clone + std::ops::Sub<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, [std::ops::Mul], mul, [std::ops::MulAssign], mul_assign, T, C, [T: Clone + std::ops::Mul<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, [std::ops::Div], div, [std::ops::DivAssign], div_assign, T, C, [T: Clone + std::ops::Div<Output = T>, C: Clone]);

// impl Num
impl<T: Clone + From<u8> + std::ops::Add<Output = T> + PartialEq, C: Clone> Zero for FPS<T, C> {
  fn zero() -> Self {
    Self::new()
  }

  fn is_zero(&self) -> bool {
    self.iter().all(|x| *x == T::from(0))
  }
}

impl<T: Clone + From<u8> + PartialEq, C: Clone + Convolution<T>> One for FPS<T, C> {
  fn one() -> Self {
    Self::from(vec![T::from(1)])
  }

  fn is_one(&self) -> bool {
    self.deg() >= 1 && self[0] == T::from(1) && (1 .. self.deg()).all(|i| self[i] == T::from(0))
  }
}

impl<T: ModIntBase + From<u64>, C: Clone + Convolution<T>> Num for FPS<T, C> {
  type FromStrRadixErr = <u64 as Num>::FromStrRadixErr;
  fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
    Ok(Self::from_slice(&[u64::from_str_radix(str, radix)?]))
  }
}

#[cfg(test)]
mod tests {
  use acl_modint::ModInt998244353;
  use super::*;

  type F = FPS998244353;
  type M = ModInt998244353;

  #[test]
  fn test_add_sub() {
    let mut f = F::from_slice(&[1, 2, 3]);
    f += M::from(1);
    f += F::from_slice(&[2, 3, 4, 5]);
    f += &F::from_slice(&[1]);
    assert_eq!(f, F::from_slice(&[5, 5, 7, 5]));

    assert_eq!(&F::from_slice(&[1, 2, 3]) + &F::from_slice(&[4, 5, 6, 7]), F::from_slice(&[5, 7, 9, 7]));
    assert_eq!(&F::from_slice(&[1, 2, 3]) + M::from(6), F::from_slice(&[7, 2, 3]));
    assert_eq!(F::from_slice(&[1, 2, 3]) - &F::from_slice(&[4, 5, 6, 7]), F::from_slice(&[998244350, 998244350, 998244350, 998244346]));
    assert_eq!(F::from_slice(&[1, 2, 3]) - M::from(6), F::from_slice(&[998244348, 2, 3]));
  }

  #[test]
  fn test_mul_div_scalar() {
    assert_eq!(F::from_slice(&[1, 2, 3, 499122177]) * M::from(2), F::from_slice(&[2, 4, 6, 1]));
    assert_eq!(F::from_slice(&[2, 4, 6, 1]) / M::from(2), F::from_slice(&[1, 2, 3, 499122177]));
  }

  #[test]
  fn test_mul() {
    assert_eq!(F::from_slice(&[1, 2, 3, 4]) * &F::from_slice(&[5, 6, 7, 8, 9]), F::from_slice(&[5, 16, 34, 60, 70, 70, 59, 36]));
  }
}