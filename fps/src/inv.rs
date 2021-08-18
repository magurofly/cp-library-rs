use super::*;
use std::ops::*;
use fft::*;

impl<T: Clone + PartialEq + From<u8> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T>> FPS<T, C> {
  /// `self[0]` must not be zero
  pub fn inv_at(&self, deg: usize) -> FPS<T, C> {
    assert!(self.len() > 0 && self[0] != T::from(0u8));
    let mut r = Self::from(vec![T::from(1) / self[0].clone()]);
    let mut i = 1;
    while i < deg {
      r = &r + &r - &r * &r * self.pre(i << 1);
      r.truncate(i << 1);
      i <<= 1;
    }
    r.truncate(deg);
    r
  }

  /// `self[0]` mut not be zero
  pub fn inv(&self) -> FPS<T, C> {
    self.inv_at(self.deg())
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  type F = FPS998244353;
  
  #[test]
  fn test_inv() {
    assert_eq!(F::from_slice(&[5, 4, 3, 2, 1]).inv(), F::from_slice(&[598946612, 718735934, 862483121, 635682004, 163871793]));
  }
}