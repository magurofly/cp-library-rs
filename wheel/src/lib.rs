pub mod ops;
pub use ops::*;

#[derive(Debug, Clone, Copy)]
pub enum Wheel<T> {
  NaN,
  PositiveInfinity,
  NegativeInfinity,
  Finite(T),
}

impl<T> Wheel<T> {
  /// `self` が `NaN` ではないか判定する
  pub fn is_nan(&self) -> bool {
    match self {
      NaN => true,
      _ => false,
    }
  }

  /// `self` が `Finite(_)` か判定する
  pub fn is_finite(&self) -> bool {
    match self {
      Finite(_) => true,
      _ => false,
    }
  }

  pub fn not_nan(self) -> Option<Wheel<T>> {
    match self {
      NaN => None,
      x => Some(x),
    }
  }

  pub fn finite(self) -> Option<T> {
    match self {
      Finite(x) => Some(x),
      _ => None,
    }
  }

  pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Wheel<U> {
    match self {
      Finite(x) => Finite((f)(x)),
      NaN => NaN,
      PositiveInfinity => PositiveInfinity,
      NegativeInfinity => NegativeInfinity,
    }
  }

  /// `self` が `Finite(x)` であるとき、 `x` を取り出す
  /// それ以外の時、 panic する
  pub fn unwrap(self) -> T {
    self.finite().expect("not finite")
  }
}

use std::cmp::*;
use Ordering::*;
use Wheel::*;

impl<T: PartialEq> PartialEq for Wheel<T> {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (NaN, _) | (_, NaN) => false,
      (PositiveInfinity, PositiveInfinity) | (NegativeInfinity, NegativeInfinity) => true,
      (Finite(x), Finite(y)) => x.eq(y),
      _ => false,
    }
  }
}

impl<T: Eq> Eq for Wheel<T> {}

impl<T: PartialOrd> PartialOrd for Wheel<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (NaN, _) | (_, NaN) => None,
      (PositiveInfinity, PositiveInfinity) | (NegativeInfinity, NegativeInfinity) => Some(Equal),
      (PositiveInfinity, _) | (_, NegativeInfinity) => Some(Greater),
      (NegativeInfinity, _) | (_, PositiveInfinity) => Some(Less),
      (Finite(x), Finite(y)) => x.partial_cmp(y),
    }
  }
} 

impl<T: Ord> Ord for Wheel<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).expect("NaN is not comparable!")
  }
}