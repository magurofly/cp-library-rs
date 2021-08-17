use super::*;
use std::ops::*;
use fft::*;

macro_rules! derive_op {
  ($Self:ty, $Rhs:ty, $trait:ident, $op:ident, $trait_assign:ident, $op_assign:ident, $T:ident, $C:ident, [$($cond:tt)*]) => {

    // FPS ?= X
    impl<$T, $C> $trait_assign<$Rhs> for $Self where $($cond)* {
      fn $op_assign(&mut self, other: $Rhs) {
        self.$op_assign(&other);
      }
    }

    // &FPS ? &X
    impl<$T, $C> $trait<&$Rhs> for &$Self where $($cond)* {
      type Output = $Self;
      fn $op(self, other: &$Rhs) -> $Self {
        let mut f: $Self = self.clone();
        f.$op_assign(other);
        f
      }
    }

    // &FPS ? X
    impl<$T, $C> $trait<$Rhs> for &$Self where $($cond)* {
      type Output = $Self;
      fn $op(self, other: $Rhs) -> $Self {
        let mut f = self.clone();
        f.$op_assign(&other);
        f
      }
    }

    // FPS ? &X
    impl<$T, $C> $trait<&$Rhs> for $Self where $($cond)* {
      type Output = $Self;
      fn $op(mut self, other: &$Rhs) -> $Self {
        self.$op_assign(other);
        self
      }
    }

    // FPS ? X
    impl<$T, $C> $trait<$Rhs> for $Self where $($cond)* {
      type Output = $Self;
      fn $op(mut self, other: $Rhs) -> $Self {
        self.$op_assign(&other);
        self
      }
    }
  }
}

// Addition
impl<T: Clone + From<u8> + Add<Output = T>, C> AddAssign<&FPS<T, C>> for FPS<T, C> {
  fn add_assign(&mut self, other: &FPS<T, C>) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] = self[i].clone() + other[i].clone();
    }
  }
}

// Scalar Addition
impl<T: Clone + Add<Output = T>, C> AddAssign<&T> for FPS<T, C> {
  fn add_assign(&mut self, other: &T) {
    self[0] = self[0].clone() + other.clone();
  }
}

// Subtraction
impl<T: Clone + From<u8> + Sub<Output = T>, C> SubAssign<&FPS<T, C>> for FPS<T, C> {
  fn sub_assign(&mut self, other: &FPS<T, C>) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] = self[i].clone() - other[i].clone();
    }
  }
}


// Scalar Subtraction
impl<T: Clone + Sub<Output = T>, C> SubAssign<&T> for FPS<T, C> {
  fn sub_assign(&mut self, other: &T) {
    self[0] = self[0].clone() - other.clone();
  }
}

// Multiplication
impl<T: Clone, C: Convolution<T> + Clone> MulAssign<&FPS<T, C>> for &FPS<T, C> {
  fn mul_assign(&mut self, other: &FPS<T, C>) {
    self.convolve(&other.clone());
  }
}

// Scalar Multiplication
impl<T: Clone + Mul<Output = T>, C> MulAssign<&T> for FPS<T, C> {
  fn mul_assign(&mut self, other: &T) {
    for i in 0 .. self.len() {
      self[i] = self[i].clone() * other.clone();
    }
  }
}

// Division
impl<T: Clone + PartialEq + From<u8> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T>> DivAssign<&FPS<T, C>> for FPS<T, C> {
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
impl<T: Clone + Div<Output = T>, C> DivAssign<&T> for FPS<T, C> {
  fn div_assign(&mut self, other: &T) {
    for i in 0 .. self.len() {
      self[i] = self[i].clone() / other.clone();
    }
  }
}

// derivations
derive_op!(FPS<T, C>, FPS<T, C>, Add, add, AddAssign, add_assign, T, C, [T: Clone + From<u8> + Add<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, FPS<T, C>, Sub, sub, SubAssign, sub_assign, T, C, [T: Clone + From<u8> + Sub<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, FPS<T, C>, Mul, mul, MulAssign, mul_assign, T, C, [T: Clone, C: Clone + Convolution<T>]);
derive_op!(FPS<T, C>, FPS<T, C>, Div, div, DivAssign, div_assign, T, C, [T: Clone + PartialEq + From<u8> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T>]);
derive_op!(FPS<T, C>, T, Add, add, AddAssign, add_assign, T, C, [T: Clone + Add<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, Sub, sub, SubAssign, sub_assign, T, C, [T: Clone + Sub<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, Mul, mul, MulAssign, mul_assign, T, C, [T: Clone + Mul<Output = T>, C: Clone]);
derive_op!(FPS<T, C>, T, Div, div, DivAssign, div_assign, T, C, [T: Clone + Div<Output = T>, C: Clone]);

#[cfg(test)]
mod tests {
  use acl_modint::ModInt998244353;
  use super::*;

  type F = FPS998244353;
  type M = ModInt998244353;

  #[test]
  fn test_add_sub() {
    assert_eq!(F::from_slice(&[1, 2, 3]) + &F::from_slice(&[4, 5, 6, 7]), F::from_slice(&[5, 7, 9, 7]));
    assert_eq!(F::from_slice(&[1, 2, 3]) + M::from(6), F::from_slice(&[7, 2, 3]));
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