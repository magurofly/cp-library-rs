use super::*;
use std::ops::*;
use fft::*;

impl<T, C> FPS<T, C> {
  /// `self[0]` must not be zero
  pub fn inv_at(&self, deg: usize) -> FPS<T, C> where T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T> {
    assert!(self.len() > 0 && self[0] != T::from(0u8));
    let mut r = Self::from(vec![T::from(1) / self[0].clone()]);
    let mut i = 1;
    while i < deg {
      let mut f = r.clone();
      f += &r;
      let mut g = r.clone();
      g *= &r;
      g *= &self.pre(i << 1);
      f -= &g;
      r = f.pre(i << 1);
      i <<= 1;
    }
    r
  }

  /// `self[0]` mut not be zero
  pub fn inv(&self) -> FPS<T, C> where T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T> {
    self.inv_at(self.deg())
  }
}