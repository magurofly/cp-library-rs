use primes::*;
pub mod primes {
  // Last Update: 2025-07-19 05:13
  
  pub struct LinearSieve { limit: usize, primes: Vec<usize>, table: Vec<usize> }
  impl LinearSieve {
    pub fn new<N: PrimInt>(limit: N) -> Self {
      let limit = cast::<usize>(limit);
      let mut table = (0 ..= limit).collect::<Vec<_>>();
      table[1] = 0;
      let mut primes = Vec::with_capacity(32);
      for n in 2 ..= limit {
        if table[n] == n {
          primes.push(n);
        }
        for &p in &primes {
          if n * p > limit || p > table[n] {
            break;
          }
          table[n * p] = p;
        }
      }
      Self { limit, table, primes }
    }
    
    pub fn is_prime<N: PrimInt>(&self, n: N) -> bool {
      let n: usize = cast(n);
      assert!(n <= self.limit);
      n > 1 && self.table[n] == n
    }
    
    pub fn primes<N: PrimInt>(&self) -> Vec<N> { self.primes.iter().map(|&n| cast(n) ).collect::<Vec<_>>() }
    
    pub fn least_prime_factor<N: PrimInt>(&self, n: N) -> N {
      let n: usize = cast(n);
      assert!(n <= self.limit);
      cast(self.table[n])
    }
    
    pub fn prime_division<N: PrimInt>(&self, n: N) -> Vec<(N, usize)> {
      let mut n: usize = cast(n);
      assert!(n <= self.limit);
      let mut divisors = vec![];
      while n > 1 {
        let p = self.table[n];
        n /= p;
        let mut e = 1;
        while self.table[n] == p {
            n /= p;
            e += 1;
        }
        divisors.push((cast(p), e));
      }
      divisors
    }
  }
  
  pub fn prime_division<N: PrimInt>(n: N) -> Vec<N> {
    let mut n: usize = cast(n);
    let mut divisors = vec![];
    let mut k = 2;
    while n > 1 && k * k < n {
      while (n % k).is_zero() {
        divisors.push(cast(k));
        n /= k;
      }
      k += 1;
    }
    if n > 1 { divisors.push(cast(n)); }
    divisors
  }
  
  pub fn is_prime<N: PrimInt>(n: N) -> bool {
    let n: usize = cast(n);
    if n <= 1 { return false; }
    if n == 2 || n == 7 || n == 61 { return true; }
    if n % 2 == 0 { return false; }
    let mut d = n - 1;
    d >>= d.trailing_zeros();
    for &a in &[2, 7, 61] {
      let mut t = d;
      let mut y = pow_mod(a, t, n);
      while t != n - 1 && y != 1 && y != n - 1 {
        y = y * y % n;
        t <<= 1;
      }
      if y != n - 1 && (t & 1) == 0 { return false; }
    }
    true
  }
  
  pub fn pow_mod<N: PrimInt>(a: N, e: N, m: N) -> N {
    let (mut a, mut e, m): (i64, i64, i64) = (cast(a % m), cast(e),cast(m));
    let mut r = 1;
    while e != 0 {
      if (e & 1) != 0 { r = r * a % m; }
      a = a * a % m;
      e >>= 1;
    }
    cast(r)
  }
  
  pub fn inv_mod<N: PrimInt>(a: N, m: N) -> N {
    pow_mod(a, m - N::one() - N::one(), m)
  }
  
  pub fn ext_gcd<N: PrimInt>(a: N, b: N) -> (N, N) {
    if a.is_zero() { return (b, N::zero()); }
    let (mut s, mut t): (i64, i64) = (cast(b), cast(a % b));
    let b: i64 = cast(b);
    let (mut m0, mut m1) = (0, 1);
    while !t.is_zero() {
      let u = s / t;
      s -= t * u;
      m0 -= m1 * u;
      swap(&mut s, &mut t);
      swap(&mut m0, &mut m1);
    }
    if m0 < 0 { m0 += b / s; }
    (cast(s), cast(m0))
  }
  
  pub fn inv_gcd<N: PrimInt>(a: N, m: N) -> Option<N> {
    let (g, x) = ext_gcd(a, m);
    if g.is_one() { Some(x) } else { None }
  }
  
  use num_traits::*;
  use std::mem::*;
  
  fn cast<N: PrimInt>(n: impl NumCast) -> N { N::from(n).unwrap() }
}
