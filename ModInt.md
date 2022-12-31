# ModInt

有限体のライブラリ。

## 機能

* 四則演算（ `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=` ）
* 任意 mod での逆元の計算 `.inv()`
* 冪乗 `.pow(n)`
* `FromStr`, `Display`

## 使い方

### mod 998244353
```rust

type Mint = ModInt998244353;

// 入力
proconio::input! {
  k: Mint,
}

// 出力
println!("{}", k * 2 + 1);
```

### 異なる mod を使う

```rust
#[derive(Default, Clone, Copy)]
struct Mod924844033;
impl Modulus for Mod924844033 {
  fn value(&self) -> i64 { 924844033 }
}
type Mint = ModInt<Mod924844033>;
```

## コード

```rust
use modint::*;
pub mod modint {
  // Last Update: 2022-11-12 06:11

  #[derive(Clone, Copy, PartialEq, Eq)]
  pub struct ModInt<M> { value: i64, modulus: M }

  pub trait Modulus: Copy {
    fn value(&self) -> i64;
  }

  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub struct CustomMod(i64);
  impl Modulus for CustomMod {
    fn value(&self) -> i64 { self.0 }
  }

  #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Mod998244353;
  impl Modulus for Mod998244353 { fn value(&self) -> i64 { 998244353 } }
  pub type ModInt998244353 = ModInt<Mod998244353>;

  #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
  pub struct Mod1000000007;
  impl Modulus for Mod1000000007 { fn value(&self) -> i64 { 1000000007 } }
  pub type ModInt1000000007 = ModInt<Mod1000000007>;
  
  pub trait Integer { fn into_i64(self) -> i64; }
  macro_rules! impl_int { ($t:ty) => {
    impl Integer for $t { fn into_i64(self) -> i64 { self as i64 } }
    impl<M: Modulus + Default> From<$t> for ModInt<M> { fn from(x: $t) -> Self { Self::new(x, M::default()) } } }
  }
  impl_int!(i8); impl_int!(i16); impl_int!(i32); impl_int!(i64); impl_int!(isize);
  impl_int!(u8); impl_int!(u16); impl_int!(u32); impl_int!(u64); impl_int!(usize);
  impl<M: Modulus> Integer for ModInt<M> { fn into_i64(self) -> i64 { self.value } }
  impl<T: Integer + Clone> Integer for &T { fn into_i64(self) -> i64 { self.clone().into_i64() } }

  impl<M: Modulus> ModInt<M> {
    pub fn new<N: Integer>(value: N, modulus: M) -> Self { Self { value: value.into_i64().rem_euclid(modulus.value()), modulus } }
    pub fn value(&self) -> i64 { self.value.rem_euclid(self.modulus()) }
    pub fn pow(&self, mut n: u64) -> Self { let mut r = self.make(1); let mut a = *self; while n != 0 { if (n & 1) == 1 { r *= a; } a = a * a; n >>= 1; } r }
    pub fn inv(&self) -> Self { let (g, value) = ext_gcd(self.value, self.modulus()); assert!(g == 1, "value and modulus are not coprime"); Self { value, modulus: self.modulus } }
    fn make<N: Integer>(&self, n: N) -> Self { Self::new(n, self.modulus) }
    fn modulus(&self) -> i64 { self.modulus.value() }
  }

  fn ext_gcd(a: i64, b: i64) -> (i64, i64) { let (mut s, mut t) = ((b, 0), (a.rem_euclid(b), 1)); while t.0 != 0 { let u = s.0 / t.0; s = (s.0 - t.0 * u, s.1 - t.1 * u); std::mem::swap(&mut s, &mut t); } if s.0 < 0 { s.0 += b / s.0; } s }

  macro_rules! impl_assign { ($t:ident, $f:ident, $m:ident, $r:ident, $b:expr) => { impl<$m: Modulus, $r: Integer> ops::$t<$r> for ModInt<$m> { fn $f(&mut self, other: $r) { ($b)(self, other) } } } }
  macro_rules! impl_ops { ($t:ident, $f:ident, $g:ident) => {
    impl<M: Modulus, N: Integer> ops::$t<N> for ModInt<M> { type Output = Self; fn $f(self, other: N) -> Self::Output { let mut r = self.clone(); (&mut r).$g(other); r } }
    impl<M: Modulus, N: Integer> ops::$t<N> for &ModInt<M> { type Output = ModInt<M>; fn $f(self, other: N) -> Self::Output { let mut r = self.clone(); (&mut r).$g(other); r } }
  } }

  impl_assign!(AddAssign, add_assign, M, N, |x: &mut ModInt<M>, y: N| { x.value += y.into_i64(); x.value %= x.modulus(); });
  impl_assign!(SubAssign, sub_assign, M, N, |x: &mut ModInt<M>, y: N| { x.value -= y.into_i64(); x.value %= x.modulus(); });
  impl_assign!(MulAssign, mul_assign, M, N, |x: &mut ModInt<M>, y: N| { x.value *= y.into_i64() % x.modulus(); x.value %= x.modulus(); });
  impl_assign!(DivAssign, div_assign, M, N, |x: &mut ModInt<M>, y: N| { let mut z = *x; z *= x.make(y).inv(); *x = z; });

  impl_ops!(Add, add, add_assign);
  impl_ops!(Sub, sub, sub_assign);
  impl_ops!(Mul, mul, mul_assign);
  impl_ops!(Div, div, div_assign);

  impl<M1: Modulus, M2: Modulus> ops::Rem<M2> for ModInt<M1> { type Output = ModInt<M2>; fn rem(self, modulus: M2) -> Self::Output { ModInt::new(self.value, modulus) } }
  impl<M: Modulus> ops::Neg for ModInt<M> { type Output = Self; fn neg(self) -> Self { Self::new(-self.value, self.modulus) } }
  impl<M: Modulus> fmt::Display for ModInt<M> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_fmt(format_args!("{:?}", self.value())) } }
  impl<M: Modulus> fmt::Debug for ModInt<M> { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_fmt(format_args!("({} mod {})", self.value(), self.modulus())) } }
  impl<M: Modulus + Default> FromStr for ModInt<M> { type Err = <i64 as FromStr>::Err; fn from_str(s: &str) -> Result<Self, Self::Err> { Ok(Self::new(i64::from_str(s)?, M::default())) } }

  use std::ops::{self, *};
  use std::fmt::{self, Debug};
  use std::str::FromStr;
}
```

## Verify

まだ
