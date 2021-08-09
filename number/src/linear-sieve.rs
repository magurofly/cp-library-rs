use num_traits::*;
use std::cell::RefCell;

pub struct LinearSieve {
  primes: Vec<usize>,
  lpf: Vec<usize>,
}
impl LinearSieve {
  pub fn new(n: impl PrimInt) -> Self {
    let n: usize = n.into().unwrap();
    let mut primes = vec![];
    let mut lpf = vec![0; n + 1];
    lpf[0] = 1;
    lpf[1] = 1;
    for i in 2 ..= n {
      if lpf[i] == 0 {
        primes.push(i);
        lpf[i] = i;
      }
      for &p in &primes {
        if p * i > n || p > lpf[i] {
          break;
        }
        lpf[p * i] = p;
      }
    }
    Self { primes, lpf }
  }

  pub fn is_prime(&self, n: impl PrimInt) -> bool {
    !n.is_one() && self.lpf(n) == n
  }

  pub fn primes(&self) -> &[usize] {
    &self.lpf
  }

  /// `n` の最小の素因数を返す
  pub fn lpf<T: PrimInt>(&self, n: T) -> T {
    let n: usize = n.into().unwrap();
    assert!(n < self.lpf.len());
    T::from(self.lpf[n]).unwrap()
  }

  pub fn prime_division<T: PrimInt>(&self, n: T) -> Vec<T> {
    let mut n: usize = n.into().unwrap();
    let mut divisors = vec![];
    while n > 1 {
      let d = self.lpf[n];
      n /= d;
      divisors.push(T::from(d).unwrap());
    }
    divisors
  }
}