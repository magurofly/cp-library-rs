use num_traits::*;

pub trait Int: PrimInt {
  fn bit_len(self) -> usize {
    (Self::zero().leading_zeros() - self.leading_zeros()) as usize
  }

  fn is_even(self) -> bool {
    (self & Self::one()).is_zero()
  }

  fn is_odd(self) -> bool {
    (self & Self::one()).is_one()
  }

  fn pow_mod(self, mut e: impl Int, m: Self) -> Self {
    let mut x = self;
    let mut r = Self::one();
    while !e.is_zero() {
      if e.is_odd() {
        r = r * x % m;
      }
      x = x * x % m;
      e = e >> 1;
    }
    r
  }

  fn gcd(self, other: Self) -> Self {
    let mut x = self.max(other);
    let mut y = self.min(other);
    while !y.is_zero() {
      let z = x % y;
      x = y;
      y = z;
    }
    x
  }

  fn lcm(self, other: Self) -> Self {
    self * other / self.gcd(other)
  }

  /// Deterministic Miller Rabin
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
    
    fn trial_division<T: PrimInt>(n: T) -> bool {
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

  /// complexity: O(sqrt self)
  fn prime_division(self) -> Vec<Self> {
    todo!()
  }
}

impl<T: PrimInt> Int for T {}

fn is_prime_miller_rabin<T: PrimInt>(n: T) -> bool {
  false
}

