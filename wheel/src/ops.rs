use super::*;
use std::ops::*;
use Wheel::*;

impl<T: Neg<Output = T>> Neg for Wheel<T> {
  type Output = Self;
  fn neg(self) -> Self {
    match self {
      NaN => NaN,
      PositiveInfinity => NegativeInfinity,
      NegativeInfinity => PositiveInfinity,
      Finite(x) => Finite(-x),
    }
  }
}

impl<T: Add<Output = T>> Add for Wheel<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    match (self, other) {
      (NaN, _) | (_, NaN) => NaN,
      (PositiveInfinity, NegativeInfinity) | (NegativeInfinity, PositiveInfinity) => NaN,
      (PositiveInfinity, _) | (_, PositiveInfinity) => PositiveInfinity,
      (NegativeInfinity, _) | (_, NegativeInfinity) => NegativeInfinity,
      (Finite(x), Finite(y)) => Finite(x + y),
    }
  }
}

impl<T: Add<Output = T>> Add<T> for Wheel<T> {
  type Output = Self;
  fn add(self, other: T) -> Self { self + Finite(other) }
}

impl<T: Sub<Output = T>> Sub for Wheel<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    match (self, other) {
      (NaN, _) | (_, NaN) => NaN,
      (PositiveInfinity, PositiveInfinity) | (NegativeInfinity, NegativeInfinity) => NaN,
      (PositiveInfinity, _) | (_, NegativeInfinity) => PositiveInfinity,
      (NegativeInfinity, _) | (_, PositiveInfinity) => NegativeInfinity,
      (Finite(x), Finite(y)) => Finite(x - y),
    }
  }
}

impl<T: Sub<Output = T>> Sub<T> for Wheel<T> {
  type Output = Self;
  fn sub(self, other: T) -> Self { self - Finite(other) }
}

impl<T: Clone + Mul<Output = T> + Sub<Output = T> + Ord> Mul for Wheel<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    match (self, other) {
      (NaN, _) | (_, NaN) => NaN,
      (PositiveInfinity, PositiveInfinity) | (NegativeInfinity, NegativeInfinity) => PositiveInfinity,
      (PositiveInfinity, NegativeInfinity) | (NegativeInfinity, PositiveInfinity) => NegativeInfinity,
      (PositiveInfinity, Finite(x)) | (Finite(x), PositiveInfinity) =>
        match x.cmp(&(x.clone() - x.clone())) {
          Equal => NaN,
          Less => NegativeInfinity,
          Greater => PositiveInfinity,
        },
      (NegativeInfinity, Finite(x)) | (Finite(x), NegativeInfinity) =>
        match x.cmp(&(x.clone() - x.clone())) {
          Equal => NaN,
          Less => PositiveInfinity,
          Greater => NegativeInfinity,
        },
      (Finite(x), Finite(y)) => Finite(x * y),
    }
  }
}

impl<T: Clone + Mul<Output = T> + Sub<Output = T> + Ord> Mul<T> for Wheel<T> {
  type Output = Self;
  fn mul(self, other: T) -> Self { self * Finite(other) }
}

impl<T: Clone + Div<Output = T> + Sub<Output = T> + Ord> Div for Wheel<T> {
  type Output = Self;
  fn div(self, other: Self) -> Self {
    match (self, other) {
      (Finite(x), PositiveInfinity) | (Finite(x), NegativeInfinity) => Finite(x.clone() - x.clone()),
      (PositiveInfinity, Finite(y)) =>
        match y.cmp(&(y.clone() - y.clone())) {
          Less => NegativeInfinity,
          _ => PositiveInfinity,
        },
      (NegativeInfinity, Finite(y)) =>
        match y.cmp(&(y.clone() - y.clone())) {
          Less => PositiveInfinity,
          _ => NegativeInfinity,
        },
      (Finite(x), Finite(y)) =>
        if y == y.clone() - y.clone() {
          match x.cmp(&(x.clone() - x.clone())) {
            Equal => NaN,
            Greater => PositiveInfinity,
            Less => NegativeInfinity,
          }
        } else {
          Finite(x / y)
        },
      _ => NaN,
    }
  }
}

impl<T: Clone + Div<Output = T> + Sub<Output = T> + Ord> Div<T> for Wheel<T> {
  type Output = Self;
  fn div(self, other: T) -> Self { self / Finite(other) }
}

impl<T: Rem<Output = T>> Rem for Wheel<T> {
  type Output = Self;
  fn rem(self, other: Self) -> Self {
    match (self, other) {
      (Finite(x), PositiveInfinity) => Finite(x),
      (Finite(x), Finite(y)) => Finite(x % y),
      _ => NaN,
    }
  }
}

impl<T: Rem<Output = T>> Rem<T> for Wheel<T> {
  type Output = Self;
  fn rem(self, other: T) -> Self { self % Finite(other) }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_ops() {
    assert_eq!(Finite(1) + Finite(2), Finite(3));
  }
}