use std::iter::*;

use super::*;

pub trait MySlice<T> : AsRef<[T]> {
  fn citer(&self) -> Cloned<std::slice::Iter<T>> where T: Clone { self.as_ref().iter().cloned() }
  fn sorted(&self) -> Vec<T> where T: Clone + Ord { let mut v = self.as_ref().to_vec(); v.sort(); v }
  fn sorted_by(&self, f: impl FnMut(&T, &T) -> std::cmp::Ordering) -> Vec<T> where T: Clone + Ord { let mut v = self.as_ref().to_vec(); v.sort_by(f); v }
  fn sorted_by_key<U: Ord>(&self, f: impl FnMut(&T) -> U) -> Vec<T> where T: Clone { let mut v = self.as_ref().to_vec(); v.sort_by_key(f); v }
  fn map<U>(&self, f: impl FnMut(&T) -> U) -> Vec<U> { self.as_ref().iter().map(f).collect() }
  fn sum(&self) -> T where T: Default + Clone + Add<Output = T> { let mut sum = T::default(); for x in self.citer() { sum = sum + x; }; sum }
  fn uniq(&self) -> Vec<T> where T: Clone + std::hash::Hash + Eq { self.citer().collect::<HashSet<_>>().into_iter().collect() }
  fn replace(&self, from: T, to: T) -> Vec<T> where T: Clone + PartialEq { let mut v = self.as_ref().to_vec(); for x in &mut v { if *x == from { *x = to.clone(); } }; v }
  fn max_value(&self) -> T where T: Clone + Ord { self.citer().max().unwrap() }
  fn min_value(&self) -> T where T: Clone + Ord { self.citer().max().unwrap() }
  fn at<I: MyIndex>(&self, idx: I) -> &T { idx.of_slice(self.as_ref()) }
}
impl<T> MySlice<T> for [T] {}

pub trait MyIndex: Copy {
  fn of_slice<T>(self, slice: &[T]) -> &T;
  fn as_usize(self) -> usize;
}
impl<I: Copy + Into<isize>> MyIndex for I {
  fn of_slice<T>(self, slice: &[T]) -> &T {
    &slice[self.into().rem_euclid(slice.len() as isize) as usize]
  }
  fn as_usize(self) -> usize {
    self.into() as usize
  }
}