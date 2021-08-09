use std::cell::RefCell;
use num_traits::*;

pub struct Combination<T> {
  m: T,
  f: RefCell<Vec<(T, T, T)>>,
}

impl<T: PrimInt> Combination<T> {
  pub fn new(m: T) -> Self {
    Self { m, f: RefCell::new(vec![]) }
  }

  fn check(&self, n: T) -> usize {
    let n: usize = n.into().unwrap();
    let m: usize = self.m.into().unwrap();
    let l = self.f.borrow().len();
    if l > n {
      return n;
    }
    let f = self.f.borrow_mut();
    let mut l2 = l;
    while l2 <= n {
      l2 <<= 1;
    }
    f.reserve(l2 - l);
    for i in l ..= l2 {
      let j = T::from(i);
      let inv = self.m - f[m % i] * (m / j) % self.m;
      let fac = f[i - 1].1 * j % self.m;
      let fin = f[i - 1].2 * inv % self.m;
      f.push((inv, fac, fin));
    }
    n
  }

  pub fn inv(&self, n: T) -> T {
    let n = self.check(n);
    self.f.borrow()[n].0
  }

  pub fn fact(&self, n: T) -> T {
    let n = self.check(n);
    self.f.borrow()[n].1
  }

  pub fn fact_inv(&self, n: T) -> T {
    let n = self.check(n);
    self.f.borrow()[n].2
  }

  pub fn perm(&self, n: T, k: T) -> T {
    self.fact(n) * self.fact_inv(k) % self.m
  }

  pub fn comb(&self, n: T, k: T) -> T {
    if n < T::zero() || k > n || k < T::zero() {
      return T::zero();
    }
    self.perm(n, k) * self.fact_inv(n - k) % self.m
  }
}