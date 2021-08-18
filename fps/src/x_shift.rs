use super::*;
use acl_modint::ModIntBase;
use fft::Convolution;
use number::*;

impl<T: ModIntBase, C: Clone + Convolution<T>> FPS<T, C> where Self: Clone {
  pub fn x_shift(&mut self, a: T) {
    let n = self.deg();
    let f = FactorialInvMod::new(n, T::modulus() as i64);
    //TODO: replace with thread-local Enumeration struct
    for i in 0 .. n {
      self[i] *= T::from(f.fact(i));
    }
    self.reverse();
    let mut g = Self::from(vec![T::from(1); n]);
    for i in 1 .. n {
      g[i] = g[i - 1] * a * T::from(f.inv(i));
    }
    *self *= g;
    self.truncate(n);
    self.reverse();
    for i in 0 .. n {
      self[i] *= T::from(f.fact_inv(i));
    }
  }

  pub fn x_shifted(&self, a: T) -> Self {
    let mut f = self.clone();
    f.x_shift(a);
    f
  }
}