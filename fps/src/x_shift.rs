use super::*;
use acl_modint::ModIntBase;
use fft::Convolution;
use enumeration::Enumeration;

impl<T: ModIntBase, C: Clone + Convolution<T>> FPS<T, C> where Self: Clone {
  pub fn x_shift(&mut self, a: T) {
    let n = self.deg();
    let f = Enumeration::<T>::new();
    //TODO: replace with thread-local Enumeration struct
    for i in 0 .. n {
      self[i] *= f.fact(i);
    }
    self.reverse();
    let mut g = Self::from(vec![T::from(1); n]);
    for i in 1 .. n {
      g[i] = g[i - 1] * a * f.inv(i);
    }
    *self *= g;
    self.truncate(n);
    self.reverse();
    for i in 0 .. n {
      self[i] *= f.fact_inv(i);
    }
  }

  pub fn x_shifted(&self, a: T) -> Self {
    let mut f = self.clone();
    f.x_shift(a);
    f
  }
}