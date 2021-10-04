pub mod linear_sieve;
pub use linear_sieve::*;

use number::*;

pub trait PrimeTrait: Int {
  /// 素数判定する
  /// 計算量: O(sqrt self) もしくは O(log self)
  fn is_prime(self) -> bool {
    let two = Self::one() + Self::one();
    let three = two + Self::one();
    let five = three + two;
    if self <= Self::one() {
      return false;
    }
    if self == two || self == three || self == five {
      return true;
    }
    if !self.gcd(two * three * five).is_one() {
      return false;
    }
    
    fn trial_division<T: Int>(n: T) -> bool {
      if n.is_one() {
        return false;
      }
      let mut i = T::one() + T::one();
      while i * i <= n {
        if (n % i).is_zero() {
          return false;
        }
        i = i + T::from(30).unwrap();
      }
      true
    }

    fn miller_rabin_test<T: Int>(n: T, bases: &[u32]) -> bool {
      let mut r = 0;
      let mut d = n >> 1;
      while d.is_even() {
        d = d >> 1;
        r += 1;
      }

      let n_ = n - T::one();
      for &a in bases {
        let a = T::from(a).unwrap();
        let mut x = a.pow_mod(d, n);
        if x.is_one() || x == n_ || a == n {
          continue;
        }

        let mut non_prime = true;
        for _ in 0 .. r {
          x = x.pow_mod(2, n);
          if x == n_ {
            non_prime = false;
            break;
          }
        }

        if non_prime {
          return false;
        }
      }

      true
    }

    let len = self.bit_len();
    if len <= 16 || len >= 82 {
      return trial_division(self);
    }

    let base: &[u32] =
      if len <= 20 {
        &[2]
      } else if len <= 23 {
        &[31, 73]
      } else if len <= 32 {
        &[2, 7, 61]
      } else if len <= 48 {
        &[2, 3, 5, 7, 11, 13, 17]
      } else {
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]
      };

    miller_rabin_test(self, base)
  }

  /// 素因数を小さい順に `Vec` で返す
  /// 計算量: O(sqrt self)
  fn prime_division(self) -> Vec<Self> {
    let mut divisors = vec![];
    let mut x = self;
    let mut i = Self::one() + Self::one();
    while i * i <= x {
      while (x % i).is_zero() {
        divisors.push(i);
        x = x / i;
      }
      i = i.add1();
    }
    if !x.is_one() {
      divisors.push(x);
    }
    divisors
  }
}

impl<T: Int> PrimeTrait for T {}