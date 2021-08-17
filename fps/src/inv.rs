use super::*;
use std::ops::*;
use fft::*;

impl<T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T>> FPS<T, C> {
  /// `self[0]` must not be zero
  pub fn inv_at(&self, deg: usize) -> FPS<T, C> {
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
  use acl_modint::ModInt998244353;
  use super::*;

  type M = ModInt998244353;
  type F = FPS998244353;

  #[test]
  fn test_inv() {
    let f = F::from(vec![M::from(5), M::from(4), M::from(3), M::from(2), M::from(1)]);
    let g = f.inv();
    assert_eq!(g.to_vec(), vec![M::from(598946612), M::from(718735934), M::from(862483121), M::from(635682004), M::from(163871793)]);
  }
}