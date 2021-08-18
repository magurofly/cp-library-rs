use super::*;
use acl_modint::ModIntBase;
use fft::*;
use number::*;

impl<T: ModIntBase, C: Clone + Convolution<T>> FPS<T, C> {
  pub fn sqrt_at(&self, deg: usize) -> Option<Self> {
    if self.deg() == 0 {
      return Some(Self::new());
    }
    if self[0] == T::from(0) {
      for i in 1 .. self.deg() {
        if self[i] != T::from(0) {
          if i.is_odd() {
            return None;
          }
          if deg - i / 2 <= 0 {
            break;
          }
          let mut ret = (self >> i).sqrt_at(deg - i / 2)?;
          ret <<= i / 2;
          ret.expand(deg);
          return Some(ret);
        }
      }
      return Some(Self::with_deg(deg));
    }
    let sqrt = T::from((self[0].val() as i64).sqrt_mod(T::modulus() as i64)?);
    if sqrt * sqrt != self[0] {
      return None;
    }
    let mut ret = Self::from_slice(&[sqrt]);
    let inv2 = T::from(2).inv();
    let mut i = 1;
    while i < deg {
      ret += self.pre(i << 1) * ret.inv_at(i << 1);
      ret *= inv2;
      i <<= 1;
    }
    Some(ret)
  }

  pub fn sqrt(&self) -> Option<Self> {
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
    let samples = vec![
      fps(&[4]),
      fps(&[0, 0, 9, 12]),
    ];
    for f in samples {
      let g = f.sqrt().expect("sqrt not found");
      assert_eq!(f, (&g * &g).pre(f.deg()));
    }

    assert_eq!(None, fps(&[0, 0, 10, 12]).sqrt());
  }
}