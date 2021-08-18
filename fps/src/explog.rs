use super::*;
use std::ops::*;
use fft::*;

impl<T: Clone + From<u8> + From<usize> + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialEq, C: Clone + Convolution<T>> FPS<T, C> {
  pub fn log_at(&self, deg: usize) -> Self {
    assert!(self[0] == T::from(1u8));
    let mut f = &self.diff() * &self.inv_at(deg);
    f.truncate(deg - 1);
    f.integral()
  }

  pub fn log(&self) -> Self {
    self.log_at(self.deg())
  }

  pub fn exp_at(&self, deg: usize) -> Self {
    assert!(self[0] == T::from(0u8));
    let mut ret = Self::from(vec![T::from(1u8)]);
    let mut i = 1;
    while i < deg {
      ret = &ret * &(&self.pre(i << 1) + T::from(1u8) - &ret.log_at(i << 1));
      ret.truncate(i << 1);
      i <<= 1;
    }
    ret.truncate(deg);
    ret
  }

  pub fn exp(&self) -> Self {
    self.exp_at(self.deg())
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use acl_modint::*;
  type M = ModInt998244353;
  type F = FPS998244353;

  #[test]
  fn test_log() {
    let f = F::from(vec![M::from(1), M::from(1), M::from(499122179), M::from(166374064), M::from(291154613)]);
    let g = f.log();
    assert_eq!(g.to_vec(), vec![M::from(0), M::from(1), M::from(2), M::from(3), M::from(4)]);
  }

  #[test]
  fn test_exp() {
    let f = F::from(vec![M::from(0), M::from(1), M::from(2), M::from(3), M::from(4)]);
    let g = f.exp();
    assert_eq!(g.to_vec(), vec![M::from(1), M::from(1), M::from(499122179), M::from(166374064), M::from(291154613)]);
  }
}