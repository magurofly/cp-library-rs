use std::ops::Index;

use super::*;

#[derive(Debug, Clone)]
pub struct InverseMod<N> {
  inv: Vec<N>,
  modulus: N,
}
impl<N: Int> InverseMod<N> {
  pub fn empty(modulus: N) -> Self {
    assert!(modulus >= N::one());
    Self { inv: vec![N::zero(), N::one()], modulus }
  }

  pub fn new(n: impl Int, modulus: N) -> Self {
    let mut this = Self::empty(modulus);
    this.ensure(n.as_usize());
    this
  }

  pub fn modulus(&self) -> N {
    self.modulus
  }

  pub fn len(&self) -> usize {
    self.inv.len()
  }

  pub fn ensure(&mut self, n: usize) {
    if self.len() > n {
      return;
    }
    
    self.inv.reserve(n - self.inv.len() + 1);
    for i in self.inv.len() ..= n {
      self.inv.push(self.modulus - self.inv[self.modulus.as_usize() % i] * (self.modulus / i.cast::<N>()) % self.modulus);
    }
  }
}
impl<I: Int, N: Int> Index<I> for InverseMod<N> {
  type Output = N;
  fn index(&self, idx: I) -> &N {
    assert!(I::zero() <= idx && idx < self.inv.len().cast());
    &self.inv[idx.cast::<usize>()]
  }
}