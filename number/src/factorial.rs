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

pub struct FactorialInvMod<N> {
  fact: FactorialMod<N>,
  inv: InverseMod<N>,
  finv: Vec<N>,
}
impl<N: Int> FactorialInvMod<N> {
  pub fn new(n: N, m: N) -> Self {
    let fact = FactorialMod::new(n, m);
    let inv = InverseMod::new(n, m);
    let mut finv = vec![N::one()];
    for i in 1 ..= n.cast::<usize>() {
      finv.push(finv[i - 1] * inv[i] % m);
    }
    Self { fact, inv, finv }
  }

  pub fn fact(&self, n: impl Int) -> N {
    self.fact[n]
  }

  pub fn inv(&self, n: impl Int) -> N {
    self.inv[n]
  }

  pub fn fact_inv(&self, n: impl Int) -> N {
    self[n]
  }
}
impl<I: Int, N: Int> Index<I> for FactorialInvMod<N> {
  type Output = N;
  fn index(&self, idx: I) -> &N {
    assert!(I::zero() <= idx && idx < self.finv.len().cast());
    &self.finv[idx.cast::<usize>()]
  }
}