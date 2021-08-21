use num_traits::*;

pub mod compress_coords;
pub use compress_coords::*;

pub mod cumulated;
pub use cumulated::*;

pub mod string;
pub use string::*;

pub mod traits;
pub use traits::*;

use std::fmt;
use std::hash::Hash;
use std::ops::{Bound::*, RangeBounds};
use std::collections::*;
use std::iter::*;

pub fn say(x: impl fmt::Display) { println!("{}", x); }

pub fn yesno(c: bool) { println!("{}", if c { "Yes" } else { "No" }); }
pub fn yes() { yesno(true); }
pub fn no() { yesno(false); }

pub trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn to_vec_of<T: From<Self::Item>>(self) -> Vec<T> where Self: Sized { self.map(|x| x.into()).to_vec() }
  fn try_to_vec_of<T: std::convert::TryFrom<Self::Item>>(self) -> Result<Vec<T>, T::Error> where Self: Sized { let mut xs = Vec::with_capacity(self.size_hint().0); for x in self { xs.push(T::try_from(x)?); } Ok(xs) }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
  fn count_if(self, f: impl FnMut(&Self::Item) -> bool) -> usize where Self: Sized { self.filter(f).count() }
  fn cumulate<Op: Fn(Self::Item, Self::Item) -> Self::Item, Inv: Fn(Self::Item) -> Self::Item>(self, init: Self::Item, op: Op, inv: Inv) -> Cumulated<Self::Item, Op, Inv> where Self: Sized, Self::Item: Copy {
    Cumulated::new(self, init, op, inv)
  }
  fn cumsum(self) -> Cumulated<Self::Item, Box<dyn Fn(Self::Item, Self::Item) -> Self::Item>, Box<dyn Fn(Self::Item) -> Self::Item>> where Self: Sized, Self::Item: Copy + Num {
    Cumulated::new(self, Self::Item::zero(), Box::new(|x, y| x + y), Box::new(|x| Self::Item::zero() - x))
  }
}
impl<T: ?Sized> MyItertools for T where T: Iterator {}

pub trait MyIntoIter : IntoIterator where Self: Sized {
  fn convert<U, V: FromIterator<U>>(self, f: impl FnMut(Self::Item) -> U) -> V { self.into_iter().map(f).collect() }
  fn implode(self, sep: &str) -> String where Self::Item: fmt::Display { self.into_iter().map(|x| format!("{}", x)).to_vec().join(sep) }
  fn with_index(self) -> Map<Enumerate<Self::IntoIter>, Box<dyn FnMut((usize, Self::Item)) -> (Self::Item, usize)>> { self.into_iter().enumerate().map(Box::new(|(i, x)| (x, i))) }
  fn it(&self) -> Self::IntoIter where Self: Clone { self.clone().into_iter() }
  fn clone_iter<'a, T: 'a + Clone>(self) -> Cloned<Self::IntoIter> where Self: IntoIterator<Item = &'a T> { self.into_iter().cloned() }
}
impl<T> MyIntoIter for T where T: IntoIterator {}

pub trait MyOrd : PartialOrd + Sized {
  // fn max(self, other: Self) -> Self { if &self < &other { other } else { self } }
  // fn min(self, other: Self) -> Self { if &self > &other { other } else { self } }
  fn chmax(&mut self, mut rhs: Self) -> bool { if self < &mut rhs { *self = rhs; true } else { false } }
  fn chmin(&mut self, mut rhs: Self) -> bool { if self > &mut rhs { *self = rhs; true } else { false } }
}
impl<T: Sized + PartialOrd> MyOrd for T {}

pub trait MyRangeBounds<T: Copy>: RangeBounds<T> {
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
impl<T: PrimInt, R: RangeBounds<T>> MyRangeBounds<T> for R {
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

pub trait MyOpt<T> : IntoIterator<Item = T> {
  fn is_present(&self) -> bool;
  fn pop(self) -> T;
  fn get(&self) -> &T;
  fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
    !self.is_present() || (f)(self.get())
  }
  fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
    self.is_present() && (f)(self.get())
  }
}
impl<T> MyOpt<T> for Option<T> {
  fn is_present(&self) -> bool { self.is_some() }
  fn pop(self) -> T { Option::unwrap(self) }
  fn get(&self) -> &T { self.as_ref().unwrap() }
}