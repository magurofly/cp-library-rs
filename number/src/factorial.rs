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
  modulus: N,
}
impl<N: Int> FactorialMod<N> {
  pub fn empty(modulus: N) -> Self {
    assert!(modulus >= N::one());
    Self { fact: vec![N::one()], modulus }
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
    self.fact.len()
  }

  pub fn ensure(&mut self, n: usize) {
    if self.fact.len() > n {
      return;
    }

    let mut last = self.fact[self.fact.len() - 1];
    self.fact.reserve(n - self.fact.len() + 1);
    for i in self.fact.len() ..= n {
      last = last * i.cast::<N>() % self.modulus;
      self.fact.push(last);
    }
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

#[derive(Debug, Clone)]
pub struct FactorialInvMod<N> {
  fact: FactorialMod<N>,
  inv: InverseMod<N>,
  finv: Vec<N>,
  modulus: N,
}
impl<N: Int> FactorialInvMod<N> {
  pub fn empty(modulus: N) -> Self {
    assert!(modulus >= N::one());
    Self {
      fact: FactorialMod::empty(modulus),
      inv: InverseMod::empty(modulus),
      finv: vec![N::one()],
      modulus,
    }
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
    self.finv.len()
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

  pub fn ensure(&mut self, n: usize) {
    if self.len() > n {
      return;
    }

    self.fact.ensure(n);
    self.inv.ensure(n);
    let mut last = self.finv[self.len() - 1];
    self.finv.reserve(n - self.len() + 1);
    for i in self.len() ..= n {
      last = last * self.inv[i] % self.modulus;
      self.finv.push(last);
    }
  }
}
impl<I: Int, N: Int> Index<I> for FactorialInvMod<N> {
  type Output = N;
  fn index(&self, idx: I) -> &N {
    assert!(I::zero() <= idx && idx < self.finv.len().cast());
    &self.finv[idx.cast::<usize>()]
  }
}

#[cfg(test)]
mod tests {
  use super::{factorial, FactorialInvMod, FactorialMod};

  #[test]
  fn test_factorial() {
    assert_eq!(factorial(0), 1);
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(6), 720);
  }

  #[test]
  fn test_factorial_mod() {
    let f = FactorialMod::new(10, 17);
    for i in 0 ..= 10 {
      assert_eq!(f[i], factorial(i) % 17);
    }
  }

  #[test]
  fn test_factorial_inv_mod() {
    let f = FactorialInvMod::new(10, 17);
    eprintln!("finv = {:?}", &f);
    for i in 0 ..= 10 {
      assert_eq!(factorial(i) % 17, f.fact(i));
      assert_eq!(factorial(i) % 17 * f.fact_inv(i) % 17, 1);
      assert_eq!(f.fact_inv(i) * f.fact(i) % 17, 1);
    }
  }
}