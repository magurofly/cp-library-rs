use super::group_props::*;

/// 閉じた二項演算
pub trait Magma {
  type S: Clone;

  fn operator(a: &Self::S, b: &Self::S) -> Self::S;
}

/// 半群
pub trait Semigroup: Magma + Associativity {
  /// a に対して繰返し b を演算する
  /// 計算量: 演算を o として、 O(o log n)
  fn repeat_operator(a: &Self::S, b: &Self::S, n: usize) -> Self::S {
    if n == 0 {
      return a.clone();
    }

    let mut n = n;
    let mut r = a.clone();
    let mut x = b.clone();

    while n != 0 {
      if (n & 1) != 0 {
        r = Self::operator(&r, &x);
      }
      x = Self::operator(&x, &x);
      n >>= 1;
    }

    r
  }
}

/// モノイド
pub trait Monoid: Semigroup + Identity {
  /// 冪乗
  fn power(a: &Self::S, n: usize) -> Self::S {
    if n == 0 {
      return Self::identity();
    }

    Self::repeat_operator(&Self::identity(), &a, n)
  }
}

/// 群
pub trait Group: Monoid + Inverse {}


// impls

impl<T: Magma + Associativity> Semigroup for T {}

impl<T: Semigroup + Identity> Monoid for T {}

impl<T: Monoid + Inverse> Group for T {}