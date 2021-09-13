use num_traits::*;

pub mod cumulated;
pub use cumulated::*;

pub mod string;
pub use string::*;

pub mod traits;
pub use traits::*;

pub use ranges::*;

pub use value_compression::*;
pub type CompressCoords<T> = ValueCompression<T>;

use std::fmt;
use std::hash::Hash;
use std::collections::*;
use std::iter::*;
use std::ops::Add;

pub fn say(x: impl fmt::Display) { println!("{}", x); }

pub fn say_either(c: bool, when_true: impl fmt::Display, when_false: impl fmt::Display) { if c { say(when_true); } else { say(when_false); } }
pub fn yesno(c: bool) { say_either(c, "Yes", "No"); }
pub fn yes() { yesno(true); }
pub fn no() { yesno(false); }

pub trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn to_vec_of<T: From<Self::Item>>(self) -> Vec<T> where Self: Sized { self.map(|x| x.into()).to_vec() }
  fn try_to_vec_of<T: std::convert::TryFrom<Self::Item>>(self) -> Result<Vec<T>, T::Error> where Self: Sized { let mut xs = Vec::with_capacity(self.size_hint().0); for x in self { xs.push(T::try_from(x)?); } Ok(xs) }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
  // fn count_if(self, f: impl FnMut(&Self::Item) -> bool) -> usize where Self: Sized { self.filter(f).count() }
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
  fn count_if<F: FnMut(&Self::Item) -> bool>(self, f: F) -> usize { self.into_iter().filter(f).count() }
}
impl<T> MyIntoIter for T where T: IntoIterator {}

pub trait MySlice<T> : AsRef<[T]> {
  fn citer(&self) -> Cloned<std::slice::Iter<T>> where T: Clone { self.as_ref().iter().cloned() }
  fn max(&self) -> Option<T> where T: Clone + Ord { self.citer().max() }
  fn min(&self) -> Option<T> where T: Clone + Ord { self.citer().max() }
  fn sorted(&self) -> Vec<T> where T: Clone + Ord { let mut v = self.as_ref().to_vec(); v.sort(); v }
  fn sorted_by(&self, f: impl FnMut(&T, &T) -> std::cmp::Ordering) -> Vec<T> where T: Clone + Ord { let mut v = self.as_ref().to_vec(); v.sort_by(f); v }
  fn sorted_by_key<U: Ord>(&self, f: impl FnMut(&T) -> U) -> Vec<T> where T: Clone { let mut v = self.as_ref().to_vec(); v.sort_by_key(f); v }
  fn map<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U> { self.as_ref().iter().map(f).collect() }
  fn sum(&self) -> T where T: Default + Clone + Add<Output = T> { let mut sum = T::default(); for x in self.citer() { sum = sum + x; }; sum }
  fn uniq(&self) -> Vec<T> where T: Clone + std::hash::Hash + Eq { self.citer().collect::<HashSet<_>>().into_iter().collect() }
  fn replace(&self, from: T, to: T) -> Vec<T> where T: Clone + PartialEq { let mut v = self.as_ref().to_vec(); for x in &mut v { if *x == from { *x = to.clone(); } }; v }
}
impl<T> MySlice<T> for [T] {}

pub trait MyOrd : PartialOrd + Sized {
  // fn max(self, other: Self) -> Self { if &self < &other { other } else { self } }
  // fn min(self, other: Self) -> Self { if &self > &other { other } else { self } }
  fn chmax(&mut self, mut rhs: Self) -> bool { if self < &mut rhs { *self = rhs; true } else { false } }
  fn chmin(&mut self, mut rhs: Self) -> bool { if self > &mut rhs { *self = rhs; true } else { false } }
}
impl<T: Sized + PartialOrd> MyOrd for T {}

pub trait MyOpt<T> : IntoIterator<Item = T> {
  fn is_present(&self) -> bool;
  fn pop(&mut self) -> T;
  fn get(&self) -> &T;
  fn get_mut(&mut self) -> &mut T;
  fn set(&mut self, value: T);
  fn is_none_or(&self, f: impl FnOnce(&T) -> bool) -> bool {
    !self.is_present() || (f)(self.get())
  }
  fn is_some_and(&self, f: impl FnOnce(&T) -> bool) -> bool {
    self.is_present() && (f)(self.get())
  }
  fn set_max(&mut self, other: T) -> bool where T: Ord {
    if !self.is_present() || *self.get() < other {
      self.set(other);
      true
    } else {
      false
    }
  }
  fn set_min(&mut self, other: T) -> bool where T: Ord {
    if !self.is_present() || *self.get() > other {
      self.set(other);
      true
    } else {
      false
    }
  }
}
impl<T> MyOpt<T> for Option<T> {
  fn is_present(&self) -> bool { self.is_some() }
  fn pop(&mut self) -> T { self.take().unwrap() }
  fn get(&self) -> &T { self.as_ref().unwrap() }
  fn get_mut(&mut self) -> &mut T { self.as_mut().unwrap() }
  fn set(&mut self, value: T) { *self = Some(value); }
}