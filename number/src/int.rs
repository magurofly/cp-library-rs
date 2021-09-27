use num_traits::*;
use super::*;

pub trait Int: PrimInt + IntLike {
  fn cast<U: NumCast>(self) -> U {
    U::from(self).unwrap()
  }

  fn mul2(self) -> Self {
    self << 1
  }

  fn div2(self) -> Self {
    self >> 1
  }

  fn is<U: PrimInt>(self, other: U) -> bool {
    self.cast::<U>() == other
  }

  fn at(self, idx: usize) -> bool {
    (self >> idx & Self::one()).is_one()
  }

  fn bit_len(self) -> usize {
    (Self::zero().leading_zeros() - self.leading_zeros()) as usize
  }

  fn is_even(self) -> bool {
    (self & Self::one()).is_zero()
  }

  fn is_odd(self) -> bool {
    !self.is_even()
  }

  fn is_positive(self) -> bool {
    self > Self::zero()
  }

  fn is_negative(self) -> bool {
    self < Self::zero()
  }

  fn div_ceil(self, other: Self) -> Self {
    (self + other - Self::one()) / other
  }

  fn digits(self) -> Vec<Self> where Self: ToString {
    let mut v = Vec::new();
    let mut n = self;
    while n.is_positive() {
      let r = n % 10.cast();
      v.push(r);
      n = n / 10.cast();
    }
    if v.is_empty() {
      v.push(Self::zero());
    }
    v.reverse();
    v
  }

  fn from_base_digits(digits: impl IntoIterator<Item = char>, radix: &[char]) -> Self {
    let base = Self::from_usize(radix.len());
    let mut value = Self::zero();
    for digit in digits {
      for i in 0 .. radix.len() {
        if radix[i] == digit {
          value = value * base + Self::from_usize(i);
        }
      }
    }
    value
  }

  fn from_base(digits: impl IntoIterator<Item = char>, base: impl Int) -> Self where Self::FromStrRadixErr: std::fmt::Debug {
    Self::from_str_radix(&digits.into_iter().collect::<String>(), base.cast()).unwrap()
  }

  fn to_base(self, radix: &[char]) -> String {
    let base = radix.len().cast();
    let mut res = String::new();
    let mut n = self;
    if n.is_negative() {
      res.push('-');
      n = Self::zero() - n;
    }
    if n.is_zero() {
      res.push('0');
    }
    while n.is_positive() {
      res.push(std::char::from_digit((n % base).cast(), base.cast()).unwrap());
      n = n / base;
    }
    res
  }

  fn to_base_digits(self, radix: &[char]) -> String {
    let base = radix.len();
    let mut res = String::new();
    let mut n = self;
    if n.is_negative() {
      res.push('-');
      n = Self::zero() - n;
    }
    if n.is_zero() {
      res.push(radix[0]);
    }
    while n.is_positive() {
      res.push(radix[(n % base.cast()).as_usize() % base]);
      n = n / base.cast();
    }
    res
  }

  fn pow_mod<E: Int>(self, mut e: E, m: Self) -> Self {
    let mut x = self % m;
    if e.is_negative() {
      x = x.inv_mod(m);
      e = E::zero() - e;
    }
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

  /// mod p 上の平方根を返す
  /// `p`: 素数
  fn sqrt_mod(self, p: Self) -> Option<Self> {
    assert!(!self.is_negative() && self < p);
    if p.as_usize() <= 2 || self.as_usize() <= 1 {
      return Some(self);
    }

    if !self.pow_mod(p.sub1().div2(), p).is_one() {
      return None;
    }

    let mut add = Self::one();
    while add.mul2() < p {
      add = add.mul2();
    }
    let mut b = Self::one();
    while b.is_zero() || b.pow_mod(p.sub1().div2(), p).is_one() {
      b = (b + add) % p;
    }

    let e = p.sub1().trailing_zeros() as usize;
    let q = p.sub1() >> e;
    // p - 1 = q << e

    let mut x = self.pow_mod(q.add1().div2(), p);
    let mut b = b.pow_mod(q, p);

    let mut shift = 2;
    while x * x % p != self {
      let error = self.pow_mod(p.as_usize() - 2, p) * x % p * x % p;
      let exp = 2.pow_mod(e - shift, p.cast::<i64>() - 1);
      if !error.pow_mod(exp, p).is_one() {
        x = x * b % p;
      }
      b = b * b % p;
      shift += 1;
    }
    Some(x)
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

  fn times(self, mut f: impl FnMut(Self)) {
    let mut i = Self::zero();
    while i < self {
      (f)(i);
      i = i.add1();
    }
  }

  fn upto(self, to: Self, mut f: impl FnMut(Self)) {
    let mut i = self;
    while i <= to {
      (f)(i);
      i = i.add1();
    }
  }

  fn downto(self, to: Self, mut f: impl FnMut(Self)) {
    let mut i = self;
    while i >= to {
      (f)(i);
      i = i.sub1();
    }
  }

  /// 約数を昇順に全列挙
  fn divisors(self) -> Vec<Self> {
    let mut divisors = vec![];
    let mut divisors2 = vec![];
    let mut k = Self::one();
    while k * k < self {
      if (self % k).is_zero() {
        divisors.push(k);
        divisors2.push(self / k);
      }
      k = k.add1();
    }
    if k * k == self {
      divisors.push(k);
    }
    divisors2.reverse();
    divisors.append(&mut divisors2);
    divisors
  }
}

impl<T: PrimInt + IntLike> Int for T {}


#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_sqrt_mod() {
    assert_eq!(Some(0), 0.sqrt_mod(5));
    assert_eq!(Some(1), 1.sqrt_mod(5));
    assert_eq!(None, 2.sqrt_mod(5));
    assert_eq!(None, 3.sqrt_mod(5));
    assert_eq!(Some(2), 4.sqrt_mod(5));
  }
}