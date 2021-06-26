pub mod primes {
  pub struct LinearSieve { limit: usize, primes: Vec<usize>, table: Vec<usize> }
  impl LinearSieve {
    const R: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
    const I: [usize; 30] = [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6, 7];
    pub fn new<N: PrimInt>(n: N) -> Self {
      let n: usize = cast(n);
      let mut primes = vec![2, 3, 5];
      let mut table = vec![0; Self::index(n) + 1];
      for i in 1..=Self::index(n) {
        let d = 30 * (i >> 3) + Self::R[i & 7];
        if table[i] == 0 {
          table[i] = d;
          primes.push(d);
        }
        for &p in &primes[3..] {
          if p * d > n || p > table[i] { break; }
          table[Self::index(p * d)] = p;
        }
      }
      Self { limit: n, primes, table }
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
    fn index(n: usize) -> usize { n / 30 << 3 | Self::I[n % 30] }
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
  fn mod_pow(mut a: usize, mut e: usize, m: usize) -> usize {
    let mut r = 1;
    while e != 0 {
      if (e & 1) != 0 { r = r * a % m; }
      a = a * a % m;
      e >>= 1;
    }
    r
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
      let mut y = mod_pow(a, t, n);
      while t != n - 1 && y != 1 && y != n - 1 {
        y = y * y % n;
        t <<= 1;
      }
      if y != n - 1 && (t & 1) == 0 { return false; }
    }
    true
  }
  
  use num_traits::*;

  fn cast<N: PrimInt>(n: impl NumCast) -> N { N::from(n).unwrap() }
}
