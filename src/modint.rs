pub mod modint {
  // Last Update: 2021-07-02 19:19
  thread_local!(static MOD: RefCell<u64> = RefCell::new(1_000_000_007));

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct ModInt<N: Int> {
    value: N,
    modulus: N,
  }

  impl<N: Int> ModInt<N> {
    pub fn with_mod(value: N, modulus: N) -> Self { assert!(N::from(2) <= modulus); Self { value: value.rem_euclid(modulus), modulus } }
    pub fn new(value: N) -> Self where N: From<u64> { let m = N::from(MOD.with(|m| *m.borrow())); Self::with_mod(value.rem_euclid(m), m) }
    pub fn set_mod(modulus: impl Into<u64>) { MOD.with(|m| *m.borrow_mut() = modulus.into()); }
    pub fn value(self) -> N { self.value }
    pub fn modulus(self) -> N { self.modulus }
    pub fn pow(self, mut n: u64) -> Self { let mut r = Self { value: N::from(1), modulus: self.modulus }; let mut a = self; while n != 0 { if (n & 1) == 1 { r = r * a; } a = a * a; n >>= 1; } r }
    pub fn inv(self) -> Self { Self { value: ext_gcd(self.value, self.modulus).1, modulus: self.modulus } }
  }

  fn ext_gcd<N: Int>(a: N, b: N) -> (N, N) { let (mut s, mut t) = ((b, N::from(0)), (a.rem_euclid(b), N::from(1))); while t.0 != N::from(0) { let u = s.0 / t.0; s = (s.0 - t.0 * u, s.1 - t.1 * u); std::mem::swap(&mut s, &mut t); } if s.0 < N::from(0) { s.0 = s.0 + b / s.0; } s }

  macro_rules! impl_assign { ($t:ident, $f:ident, $r:ty, $b:expr) => { impl<N: Int> ops::$t<$r> for ModInt<N> { fn $f(&mut self, other: $r) { ($b)(self, other) } } } }
  macro_rules! impl_ops { ($t:ident, $f:ident, $g:ident, $r:ty) => { impl<N: Int> ops::$t<$r> for ModInt<N> { type Output = Self; fn $f(self, other: $r) -> Self { let mut r = self.clone(); (&mut r).$g(other); r } } } }

  impl_assign!(AddAssign, add_assign, N, |x: &mut ModInt<N>, y: N| { x.value = x.value + y; if x.value > x.modulus { x.value = x.value - x.modulus; } });
  impl_assign!(SubAssign, sub_assign, N, |x: &mut ModInt<N>, y: N| { x.value = x.value - y; if x.value + x.modulus < x.modulus { x.value = x.value + x.modulus; } });
  impl_assign!(MulAssign, mul_assign, N, |x: &mut ModInt<N>, y: N| { x.value = (x.value * y).rem_euclid(x.modulus); });
  impl_assign!(DivAssign, div_assign, N, |x: &mut ModInt<N>, y: N| { *x *= ext_gcd(y, x.modulus).1; });
  impl_assign!(AddAssign, add_assign, Self, |x: &mut ModInt<N>, y: ModInt<N>| { *x += y.value; });
  impl_assign!(SubAssign, sub_assign, Self, |x: &mut ModInt<N>, y: ModInt<N>| { *x -= y.value; });
  impl_assign!(MulAssign, mul_assign, Self, |x: &mut ModInt<N>, y: ModInt<N>| { *x *= y.value; });
  impl_assign!(DivAssign, div_assign, Self, |x: &mut ModInt<N>, y: ModInt<N>| { *x /= y.value; });

  impl_ops!(Add, add, add_assign, N);
  impl_ops!(Sub, sub, sub_assign, N);
  impl_ops!(Mul, mul, mul_assign, N);
  impl_ops!(Div, div, div_assign, N);
  impl_ops!(Add, add, add_assign, Self);
  impl_ops!(Sub, sub, sub_assign, Self);
  impl_ops!(Mul, mul, mul_assign, Self);
  impl_ops!(Div, div, div_assign, Self);

  impl<N: Int> ops::Rem for ModInt<N> { type Output = Self; fn rem(self, other: Self) -> Self { Self::with_mod(self.value, other.value) } }
  impl<N: Int> ops::Neg for ModInt<N> { type Output = Self; fn neg(self) -> Self { Self::with_mod(self.modulus - self.value, self.modulus) } }
  impl<N: Int + FromStr + From<u64>> FromStr for ModInt<N> { type Err = N::Err; fn from_str(s: &str) -> Result<Self, N::Err> { Ok(Self::new(N::from_str(s)?)) } }
  impl<N: Int + fmt::Display> fmt::Display for ModInt<N> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value.fmt(f) } }

  pub trait Int: Clone + Copy + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + PartialEq + PartialOrd + From<u8> {
    fn rem_euclid(self, m: Self) -> Self { let mut x = self % m; if x < Self::from(0) { x = x + m; } x }
  }
  impl<N: Clone + Copy + ops::Add<Output = Self> + ops::Sub<Output = Self> + ops::Mul<Output = Self> + ops::Div<Output = Self> + ops::Rem<Output = Self> + PartialEq + PartialOrd + From<u8>> Int for N {}

  use std::ops::{self, *};
  use std::fmt::{self, Debug};
  use std::cell::RefCell;
  use std::str::FromStr;
}
