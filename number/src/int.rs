use num_traits::*;

pub trait Int: PrimInt {
  fn is<U: PrimInt>(self, other: U) -> bool {
    Self::from(other).map(|x| self == x).unwrap_or(false)
  }

  fn at(self, idx: usize) -> bool {
    (self >> idx & Self::one()).is_one()
  }

  fn add1(self) -> Self {
    self + Self::one()
  }

  fn sub1(self) -> Self {
    self - Self::one()
  }

  fn bit_len(self) -> usize {
    (Self::zero().leading_zeros() - self.leading_zeros()) as usize
  }

  fn is_even(self) -> bool {
    (self & Self::one()).is_zero()
  }

  fn is_odd(self) -> bool {
    (self & Self::one()).is_one()
  }

  fn div_ceil(self, other: Self) -> Self {
    (self + other - Self::one()) / other
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

  /// mod m 上の逆元を返す
  fn inv_mod(self, m: Self) -> Self {
    self.inv_gcd(m).1
  }

  /// `(gcd(self, b), self^-1 mod b)` を返す
  fn inv_gcd(self, b: Self) -> (Self, Self) {
    let a = self % b;
    if a.is_zero() {
      return (b, Self::zero());
    }
    let mut x = (b, Self::zero());
    let mut y = (a, Self::one());
    while !y.0.is_zero() {
      let u = x.0 / y.0;
      x.0 = x.0 - y.0 * u;
      x.1 = x.1 - y.1 * u;
      std::mem::swap(&mut x, &mut y);
    }
    if x.1 < Self::zero() {
      x.1 = x.1 + b / x.0;
    }
    x
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

  /// 原始根のひとつを返す（存在すれば）
  fn primitive_root(self) -> Option<Self> {
    if self.is(2) {
      return Self::from(1)
    }
    if self.is(167772161) || self.is(469762049) || self.is(998244353) {
      return Self::from(3)
    }
    if self.is(754974721) {
      return Self::from(11)
    }
    let mut divs = vec![Self::zero(); 20];
    divs[0] = Self::from(2)?;
    let mut cnt = 1;
    let mut x = self.sub1() >> 1;
    while x.is_even() {
      x = x >> 1;
    }
    let mut i = Self::from(3)?;
    while i * i <= x {
      if (x % i).is_zero() {
        divs[cnt] = i;
        cnt += 1;
        while (x % i).is_zero() {
          x = x / i;
        }
      }
      i = i + Self::from(2)?;
    }
    if x > Self::one() {
      divs[cnt] = x;
      cnt += 1;
    }
    let mut g = Self::from(2)?;
    loop {
      let mut ok = true;
      for i in 0 .. cnt {
        if g.pow_mod(self.sub1() / divs[i], self).is_one() {
          ok = false;
          break;
        }
      }
      if ok {
        return Some(g);
      }
      g = g.add1();
    }
  }

  /// `self` 以下の正整数で `self` と互いに素なものの個数
  fn euler_phi(self) -> Self {
    let mut n = self;
    let mut r = self;
    let mut i = Self::one() + Self::one();
    while i * i <= n {
      if (n % i).is_zero() {
        r = r - r / i;
        while (n % i).is_zero() {
          n = n / i;
        }
      }
      i = i.add1();
    }
    r
  }

  /// `self↑↑e mod m` を返す
  /// 計算量 O(sqrt m)
  /// https://ei1333.github.io/library/math/combinatorics/mod-tetration.cpp
  fn tetration_mod(self, e: Self, m: Self) -> Self {
    if m.is_one() {
      Self::zero()
    } else if self.is_zero() {
      if e.is_odd() {
        Self::zero()
      } else {
        Self::one()
      }
    } else if e.is_one() {
      self % m
    } else if e.is(2) {
      self.pow_mod(self, m)
    } else {
      let phi = m.euler_phi();
      let mut t = self.tetration_mod(e.sub1(), phi);
      if t.is_zero() {
        t = t + phi;
      }
      self.pow_mod(t, m)
    }
  }
}

impl<T: PrimInt> Int for T {}
