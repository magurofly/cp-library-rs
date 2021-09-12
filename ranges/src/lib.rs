use std::ops::*;
use Bound::*;
use num_traits::*;

pub trait Ranges<T: Copy>: RangeBounds<T> {
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