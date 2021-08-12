use std::ops::Index;

use super::*;

pub struct InverseMod<N> {
  inv: Vec<N>,
}
impl<N: Int> InverseMod<N> {
  pub fn new(n: N, m: N) -> Self {
    assert!(m >= N::one());
    let mut inv = vec![N::zero(), N::one()];
    let mut i = N::one() + N::one();
    for _ in 2 ..= n.cast::<usize>() {
      inv.push(m - inv[(m % i).cast::<usize>()] * (m / i) % m);
      i = i.add1();
    }
    Self { inv }
  }
}
impl<I: Int, N: Int> Index<I> for InverseMod<N> {
  type Output = N;
  fn index(&self, idx: I) -> &N {
    assert!(I::zero() <= idx && idx < self.inv.len().cast());
    &self.inv[idx.cast::<usize>()]
  }
}