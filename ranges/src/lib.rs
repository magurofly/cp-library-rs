use std::{borrow::Borrow, ops::*};
use Bound::*;
use std::cmp::*;
use Ordering::*;
use num_traits::*;

pub trait Ranges<T: Copy + Ord>: RangeBounds<T> {
  fn start_close(&self) -> Option<T>;
  fn start_open(&self) -> Option<T>;
  fn end_close(&self) -> Option<T>;
  fn end_open(&self) -> Option<T>;

  fn start_close_or(&self, default: T) -> T {
    self.start_close().unwrap_or(default)
  }
  fn start_open_or(&self, default: T) -> T {
    self.start_open().unwrap_or(default)
  }
  fn end_close_or(&self, default: T) -> T {
    self.end_close().unwrap_or(default)
  }
  fn end_open_or(&self, default: T) -> T {
    self.end_open().unwrap_or(default)
  }

  fn as_pair(&self) -> (Bound<&T>, Bound<&T>) {
    (self.start_bound(), self.end_bound())
  }

  /// `self` が `other` を包含しているか
  fn includes(&self, other: &impl Ranges<T>) -> bool {
    self.start_bound().cmp(&other.start_bound()) != Greater
    && self.end_bound().cmp(&other.end_bound()) != Less
  }
  
  /// 区間が空か
  fn is_empty(&self) -> bool {
    match (self.start_bound(), self.end_bound()) {
      (Excluded(l), Excluded(r)) => l == r,
      _ => false,
    }
  }

  /// 重なっているか
  fn overlays(&self, other: &impl Ranges<T>) -> bool where Self: Sized {
    if self.is_empty() || other.is_empty() {
      return false;
    }
    match (self.start_bound(), self.end_bound()) {
      (Unbounded, Unbounded) => true,
      (Unbounded, Included(r1)) =>
        match other.start_bound() {
          Unbounded => true,
          Included(l2) => l2 <= r1,
          Excluded(l2) => l2 < r1,
        },
      (Unbounded, Excluded(r1))  =>
        match other.start_bound() {
          Unbounded => true,
          Included(l2) | Excluded(l2) => l2 < r1,
        },
      (Included(l1), Unbounded) =>
        match other.end_bound() {
          Unbounded => true,
          Included(r2) => l1 <= r2,
          Excluded(r2) => l1 < r2,
        },
      (Included(l1), Included(r1)) =>
        match (other.start_bound(), other.end_bound()) {
          (Included(l2), Included(r2)) => (l1 <= l2 && l2 <= r1) || (l2 <= l1 && l1 <= r2),
          (Included(l2), Excluded(r2)) => (l1 <= l2 && l2 <= r1) || (l2 <= l1 && l1 < r2),
          (Excluded(l2), Excluded(r2)) => (l1 <= l2 && l2 < r1) || (l2 < l1 && l1 < r2),
          _ => other.overlays(self),
        },
      (Included(l1), Excluded(r1)) =>
        match (other.start_bound(), other.end_bound()) {
          (Included(l2), Excluded(r2)) => (l1 <= l2 && l2 < r1) || (l2 <= l1 && l1 < r2),
          (Excluded(l2), Excluded(r2)) => (l1 <= l2 && l2 < r1) || (l2 < l1 && l1 < r2),
          _ => other.overlays(self),
        },
      (Excluded(l1), Unbounded) =>
        match other.end_bound() {
          Unbounded => true,
          Included(r2) | Excluded(r2) => l1 < r2,
        },
      (Excluded(l1), Included(r1)) =>
        match (other.start_bound(), other.end_bound()) {
          (Excluded(l2), Included(r2)) => (l1 < l2 && l2 < r1) || (l2 < l1 && l1 < r2),
          (Excluded(l2), Excluded(r2)) => (l1 < l2 && l2 < r1) || (l2 < l1 && l1 < r2),
          _ => other.overlays(self),
        },
      (Excluded(l1), Excluded(r1)) =>
        match (other.start_bound(), other.end_bound()) {
          (Excluded(l2), Excluded(r2)) => (l1 < l2 && l2 < r1) || (l2 < l1 && l1 < r2),
          _ => other.overlays(self),
        },
    }
  }

  fn intersect(&self, _other: &impl Ranges<T>) -> Vec<(Bound<T>, Bound<T>)> {
    todo!()
  }

  fn union(&self, _other: &impl Ranges<T>) -> Vec<(Bound<T>, Bound<T>)> {
    todo!()
  }

  fn lower_bound(&self, mut f: impl FnMut(T) -> bool) -> Option<T> where T: PrimInt {
    if let Some(mut wa) = self.start_open() {
      if let Some(mut ac) = self.end_close() {
        // 二分探索
        while ac - wa > T::one() {
          let wj = wa + (ac - wa >> 1);
          if (f)(wj) {
            ac = wj;
          } else {
            wa = wj;
          }
        }
        if (f)(ac) {
          Some(ac)
        } else {
          None
        }
      } else {
        // 指数探索
        let mut prev = wa;
        let mut add = T::one();
        while !(f)(wa + add) {
          prev = wa + add;
          add = add << 1;
        }
        (prev .. wa).lower_bound(f)
      }
    }  else {
      if let Some(ac) = self.end_close() {
        // 指数探索
        let mut prev = ac;
        let mut sub = T::one();
        while (f)(ac - sub) {
          prev = ac - sub;
          sub = sub << 1;
        }
        (ac - sub ..= prev).lower_bound(f)
      } else {
        panic!("No bounds")
      }
    }
  }

  fn upper_bound(&self, mut f: impl FnMut(T) -> bool) -> Option<T> where T: PrimInt {
    if let Some(mut ac) = self.start_close() {
      if let Some(mut wa) = self.end_open() {
        // 二分探索
        while wa - ac > T::one() {
          let wj = ac + (wa - ac >> 1);
          if (f)(wj) {
            ac = wj;
          } else {
            wa = wj;
          }
        }
        if (f)(ac) {
          Some(ac)
        } else {
          None
        }
      } else {
        // 指数探索
        let mut prev = ac;
        let mut add = T::one();
        while (f)(ac + add) {
          prev = ac + add;
          add = add << 1;
        }
        (prev ..= ac + add).upper_bound(f)
      }
    }  else {
      if let Some(wa) = self.end_open() {
        // 指数探索
        let mut prev = wa;
        let mut sub = T::one();
        while !(f)(wa - sub) {
          prev = wa - sub;
          sub = sub << 1;
        }
        (wa - sub .. prev).upper_bound(f)
      } else {
        panic!("No bounds")
      }
    }
  }
}
impl<T: PrimInt, R: RangeBounds<T>> Ranges<T> for R {
  fn start_close(&self) -> Option<T> {
    match self.start_bound() {
      Included(&close) => Some(close),
      Excluded(&open) => Some(open + T::one()),
      Unbounded => None,
    }
  }
  fn start_open(&self) -> Option<T> {
    match self.start_bound() {
      Included(&close) => Some(close - T::one()),
      Excluded(&open) => Some(open),
      Unbounded => None,
    }
  }
  fn end_close(&self) -> Option<T> {
    match self.end_bound() {
      Included(&close) => Some(close),
      Excluded(&open) => Some(open - T::one()),
      Unbounded => None,
    }
  }
  fn end_open(&self) -> Option<T> {
    match self.end_bound() {
      Included(&close) => Some(close + T::one()),
      Excluded(&open) => Some(open),
      Unbounded => None,
    }
  }
}

pub trait Bounds<T>: Borrow<Bound<T>> {
  fn minmax(mut self, mut other: Self) -> (Self, Self) where Self: Sized, T: Ord {
    if self.cmp(&other) == Greater {
      std::mem::swap(&mut self, &mut other);
    }
    (self, other)
  }

  fn min(self, other: Self) -> Self where Self: Sized, T: Ord {
    self.minmax(other).0
  }

  fn max(self, other: Self) -> Self where Self: Sized, T: Ord {
    self.minmax(other).1
  }

  fn cmp_start_start(&self, other: &Self) -> Ordering where T: Ord {
    match (self.borrow(), other.borrow()) {
      (Unbounded, Unbounded) => Equal,
      (Unbounded, _) => Greater,
      (_, Unbounded) => Less,
      (Included(l1), Included(l2)) | (Excluded(l1), Excluded(l2)) => l1.cmp(l2),
      (Included(l1), Excluded(l2)) => if l1 <= l2 { Greater } else { Less },
      (Excluded(l1), Included(l2)) => if l1 < l2 { Greater } else { Less },
    }
  }

  fn cmp(&self, other: &Self) -> Ordering where T: Ord {
    match self.borrow() {
      Unbounded => match other.borrow() {
        Unbounded => Equal,
        _ => Greater,
      },
      Included(x) => match other.borrow() {
        Unbounded => Less,
        Included(y) => x.cmp(y),
        Excluded(y) => match x.cmp(y) {
          Less => Less,
          _ => Greater,
        }
      },
      Excluded(x) => match other.borrow() {
        Unbounded => Less,
        Included(y) => match x.cmp(y) {
          Greater => Greater,
          _ => Less,
        },
        Excluded(y) => x.cmp(y),
      },
    }
  }
}
impl<T> Bounds<T> for Bound<T> {}