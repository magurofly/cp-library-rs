use std::ops::{Add, Index, Mul};

use super::*;

pub fn factorial<N: Int>(n: N) -> N {
  if n < N::zero() {
    panic!("Negative factorial is not supported");
  }

  let mut r = N::one();
  let mut i = N::one();
  while i <= n {
    r = r * i;
    i = i.add1();
  }

  r
}

#[derive(Debug, Clone)]
pub struct FactorialMod<N> {
  fact: Vec<N>,
}
impl<N: Int> FactorialMod<N> {
  pub fn new(n: N, m: N) -> Self {
    let mut fact = vec![N::one()];
    let mut r = N::one();
    let mut i = N::one();
    while i <= n {
      r = r * i % m;
      fact.push(r);
      i = i.add1();
    }
    Self { fact }
  }
}
impl<I: Int, N: Int> Index<I> for FactorialMod<N> {
  type Output = N;
  fn index(&self, idx: I) -> &N {
    assert!(I::zero() <= idx && idx < self.fact.len().cast());
    &self.fact[idx.cast::<usize>()]
  }
}

pub fn factorials<N: Copy + Add<Output = N> + Mul<Output = N> + PartialOrd>(one: N, n: N) -> Vec<N> {
  let mut fact = vec![one];
  let mut r = one;
  let mut i = one;
  while i <= n {
    r = r * i;
    fact.push(r);
    i = i + one;
  }
  fact
}