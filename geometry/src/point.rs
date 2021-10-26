use num_traits::*;
use std::ops::*;

pub trait Point<T: Copy + Num>: Num + Mul<T, Output = Self> + Div<T, Output = Self> {
  fn dim(&self) -> usize;

  fn get(&self, d: usize) -> T;

  /// スライスから初期化する
  /// スライスの長さが対象の次元でない場合、panicする
  fn from_slice(slice: &[T]) -> Self;

  /// 内積を計算する
  fn dot(&self, other: &Self) -> T {
    let mut x = T::zero();
    for i in 0 .. self.dim() {
      x = x + self.get(i) * other.get(i);
    }
    x
  }

  /// ノルムの2乗を計算する
  fn norm2(&self) -> T {
    let mut x = T::zero();
    for i in 0 .. self.dim() {
      x = x + self.get(i) * self.get(i);
    }
    x
  }

  /// ノルムを計算する
  fn norm(&self) -> T where T: Float {
    self.norm2().sqrt()
  }

  /// ノルムが1になるように正規化する
  fn normalize(self) -> Self where T: Float {
    let r = self.norm();
    self / r
  }

  fn dist2(self, other: Self) -> T {
    (self - other).norm2()
  }

  fn dist(self, other: Self) -> T where T: Float {
    (self - other).norm()
  }
}