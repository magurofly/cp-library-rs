use super::*;
use fft::*;
use number::*;
use std::ops::*;

impl<T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + MulAssign + Div<Output = T>, C: Clone + Convolution<T>> FPS<T, C> {
  pub fn sqrt_at(&self, deg: usize) -> Self {
    let n = self.deg();
    if self[0] == T::from(0) {
      for i in 1 .. n {
        if self[i] != T::from(0) {
          if i.is_odd() {
            return Self::new();
          }
          if deg <= i / 2 {
            break;
          }
          let mut ret = &(self >> i).sqrt_at(deg - i / 2) << (i / 2);
          if ret.deg() < deg {
            ret.resize(deg);
          }
          return ret;
        }
      }
      return Self::from(vec![T::from(0); deg]);
    }
    
    let mut ret = Self::from(vec![T::from(1)]);
    let inv2 = T::from(1) / T::from(2);
    let mut i = 1;
    while i < deg {
      ret += &(self.pre(i << 1) * &ret.inv_at(i << 1));
      ret *= inv2.clone();
      i <<= 1;
    }
    ret.truncate(deg);
    ret
  }

  pub fn sqrt(&self) -> Self {
    self.sqrt_at(self.deg())
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use acl_modint::*;

  type M = ModInt998244353;
  type F = FPS998244353;

  fn fps(s: &[u32]) -> F {
    F::from(s.into_iter().map(|x| M::from(*x)).collect::<Vec<_>>())
  }

  #[test]
  fn test_sqrt() {
    assert_eq!(fps(&[0, 0, 9, 12]).sqrt(), fps(&[0, 3, 2, 332748117]));
  }
}