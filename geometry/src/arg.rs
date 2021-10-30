use std::cmp::Ordering;

use super::*;
use num_traits::*;

#[derive(Clone, Copy, Debug)]
/// 偏角で比較
pub struct OrdByArg<T>(Point2<T>);
impl<T: Copy + Num> OrdByArg<T> {
  pub fn new(x: T, y: T) -> Self {
    Self(Point2::new(x, y))
  }

  pub fn point(self) -> Point2<T> {
    self.0
  }
}
impl<T: Copy + Num> PartialEq for OrdByArg<T> {
  fn eq(&self, other: &Self) -> bool {
    self.0.cross(other.0) == T::zero()
  }
}
impl<T: Copy + Num> Eq for OrdByArg<T> {}
impl<T: Copy + Num + PartialOrd> PartialOrd for OrdByArg<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.cross(other.0).partial_cmp(&T::zero())
  }
}
impl<T: Copy + Num + PartialOrd> Ord for OrdByArg<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}

/// 偏角で比較
pub fn cmp_by_arg<T: Copy + Num + PartialOrd>(x: &Point2<T>, y: &Point2<T>) -> Ordering {
  x.cross(*y).partial_cmp(&T::zero()).unwrap()
}

/// 偏角ソート
pub fn sort_by_arg<T: Copy + Num + PartialOrd>(slice: &mut [Point2<T>]) {
  slice.sort_by(cmp_by_arg);
}