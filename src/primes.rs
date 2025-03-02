use primes::*;
pub mod primes {
  // Last Update: 2025-01-12 18:20
  
  pub struct LinearSieve { limit: usize, primes: Vec<usize>, table: Vec<usize> }
  impl LinearSieve {
    const REM: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
    const IDX: [usize; 30] = [8, 0, 8, 8, 8, 8, 8, 1, 8, 8, 8, 2, 8, 3, 8, 8, 8, 4, 8, 5, 8, 8, 8, 6, 8, 8, 8, 8, 8, 7];
    
    pub fn new<N: PrimInt>(limit: N) -> Self {
      let limit: usize = cast(limit);
      let mut table = vec![1; (limit + 29) / 30 * 8 + 1];
      let mut primes = Vec::with_capacity(32);
      for i in 1 .. table.len() {
        let n = 30 * (i >> 3) + Self::REM[i & 7];
        if table[i] == 1 {
          table[i] = n;
          primes.push(n);
        }
        for &p in &primes {
          if n * p > limit || p > table[i] {
            break;
          }
          table[n * p / 30 << 3 | Self::IDX[n * p % 30]] = p;
        }
      }
      Self { limit, table, primes }
    }
    
    pub fn is_prime<N: PrimInt>(&self, n: N) -> bool {
      let n: usize = cast(n);
      assert!(n <= self.limit);
      n == 2 || n == 3 || n == 5 || n % 2 != 0 && n % 3 != 0 && n % 5 != 0 && self.table[Self::index(n)] == n
    }
    
    pub fn primes<N: PrimInt>(&self) -> Vec<N> { self.primes.iter().map(|&n| cast(n) ).collect::<Vec<_>>() }
    
    pub fn least_prime_factor<N: PrimInt>(&self, n: N) -> N {
      let n: usize = cast(n);
      assert!(n <= self.limit);
      if n % 2 == 0 { return cast(2); }
      if n % 3 == 0 { return cast(3); }
      if n % 5 == 0 { return cast(5); }
      cast(self.table[Self::index(n)])
    }
    
    pub fn prime_division<N: PrimInt>(&self, n: N) -> Vec<N> {
      let mut n: usize = cast(n);
      assert!(n <= self.limit);
      let mut divisors = vec![];
      while n > 1 {
        let d = self.least_prime_factor(n);
        n /= d;
        divisors.push(cast(d));
      }
      divisors
    }
    
    pub fn prime_division_pairs<N: PrimInt>(&self, n: N) -> Vec<(N, usize)> {
      if n.is_one() {
        return vec![];
      }
      let pd = self.prime_division(n);
      let mut prev_p = pd[0];
      let mut e = 0;
      let mut pairs = vec![];
      for p in pd.into_iter().chain(Some(N::one())) {
        if p == prev_p {
          e += 1;
        } else {
          pairs.push((prev_p, e));
          
          prev_p = p;
          e = 1;
        }
      }
      pairs
    }
    
    fn index(n: usize) -> usize { n / 30 << 3 | Self::IDX[n % 30] }
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
