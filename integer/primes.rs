fn main() {
  println!("{:?}", factorize(10000008i64));
}

pub mod primes {
  fn two<N: PrimInt>() -> N { N::one() + N::one() }
  fn as_n<N: PrimInt>(n: i64) -> N { N::from(n).unwrap() }

  // O(sqrt n) time prime division (trial division)
  pub fn factorize_td<N: PrimInt>(n: N) -> HashMap<N, usize> {
    let mut pd = HashMap::new();
    if (n & N::one()).is_zero() {
      let mut c = 1;
      n >>= 1;
      while (n & N::one()).is_zero() {
        c += 1;
        n >>= 1;
      }
      pd.insert(two(), c);
    }
    let mut i = three();
    for i * i <= n {
      
      n /= i;
    }
  }

  use num_traits::*;
  use num_integer::*;
}

fn factorize(n: i64) -> HashMap<i64, i64> {
  use num_integer::gcd;

  fn is_prime_mr(n: i64) -> bool {
    let mut d = n - 1;
    d /= d & -d;
    let 
  }
}