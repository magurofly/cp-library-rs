//! セグメント木の実装をしやすくするヘルパー群
//! 
//! # 実装例
//! 
//! ```
//! struct RangeAddRangeMin;
//! impl LazySeg for RangeAddRangeMin {
//!   type T = i64; // 値
//!   type F = i64; // 作用
//!
//!   fn op(x: i64, y: i64) -> i64 {
//!     x.min(y)
//!   }
//!
//!   fn op_id() -> i64 {
//!     0
//!   }
//!   
//!   fn map(f: i64, x: i64, _n: usize) -> i64 {
//!     f + x
//!   }
//!   
//!   fn map_compose(f: i64, g: i64) -> i64 {
//!     f + g
//!   }
//!   
//!   fn map_id() -> i64 {
//!     0
//!   }
//! }
//!
//! struct RangeAddRangeSum;
//! impl LazySeg for RangeAddRangeSum {
//!   type T = i64; // 値
//!   type F = i64; // 作用
//!
//!   fn op(x: i64, y: i64) -> i64 {
//!     x + y
//!   }
//!
//!   fn op_id() -> i64 {
//!     0
//!   }
//!   
//!   fn map(f: i64, x: i64, n: usize) -> i64 {
//!     f * (n as i64) + x
//!   }
//!   
//!   fn map_compose(f: i64, g: i64) -> i64 {
//!     f + g
//!   }
//!   
//!   fn map_id() -> i64 {
//!     0
//!   }
//! }
//! 
//! let mut seg1 = RangeAddRangeMin::new(10);
//! let mut seg2 = RangeAddRangeMin::from_iter(vec![1, 2, 3, 4, 5]);
//! 
//! ```

use acl_segtree::*;
use acl_lazysegtree::*;
use acl_traits::*;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Add, Mul};
use super::*;

macro_rules! impl_prepared {
  ($S:ident, [$($traits:tt)*], $monoid:ty, $name:ident, $name_vec:ident) => {
    pub fn $name<$S>(n: usize) -> Segtree<$monoid> where $($traits)* {
      Segtree::new(n)
    }

    pub fn $name_vec<$S>(v: Vec<$S>) -> Segtree<$monoid> where $($traits)* {
      Segtree::from(v)
    }
  }
}

// Range Sum
impl_prepared!(S, [S: Copy + Add<Output = S> + Zero], Additive<S>, range_sum, range_sum_from);

// Range Multiple
impl_prepared!(S, [S: Copy + Mul<Output = S> + One], Multiplicative<S>, range_mul, range_mul_from);

// Range Min
impl_prepared!(S, [S: Copy + Ord + BoundedAbove], Min<S>, range_min, range_min_from);

// Range Max
impl_prepared!(S, [S: Copy + Ord + BoundedBelow], Max<S>, range_max, range_max_from);

pub struct RangeMin<S>(PhantomData<S>);
impl<S: Copy + Ord + BoundedAbove> Monoid for RangeMin<S> {
  type S = IndexedOrd<S>;

  fn identity() -> Self::S {
    IndexedOrd::without_index(<S as BoundedAbove>::max_value())
  }

  fn binary_operation(&x: &Self::S, &y: &Self::S) -> Self::S {
    use std::cmp::Ordering::*;
    match x.cmp(&y) {
      Less => x,
      Greater => y,
      Equal => {
        if let Some(i) = x.index() {
          if let Some(j) = y.index() {
            let k = i.min(j);
            IndexedOrd::with_index(x.value(), k)
          } else {
            x
          }
        } else {
          y
        }
      },
    }
  }
}

pub struct RangeMax<S>(PhantomData<S>);
impl<S: Copy + Ord + BoundedBelow> Monoid for RangeMax<S> {
  type S = IndexedOrd<S>;

  fn identity() -> Self::S {
    IndexedOrd::without_index(<S as BoundedBelow>::min_value())
  }

  fn binary_operation(&x: &Self::S, &y: &Self::S) -> Self::S {
    use std::cmp::Ordering::*;
    match x.cmp(&y) {
      Greater => x,
      Less => y,
      Equal => {
        if let Some(i) = x.index() {
          if let Some(j) = y.index() {
            let k = i.min(j);
            IndexedOrd::with_index(x.value(), k)
          } else {
            x
          }
        } else {
          y
        }
      },
    }
  }
}

pub struct RangeSum<S>(PhantomData<S>);
impl<S: Copy + std::ops::Add<Output = S> + Zero + One> Monoid for RangeSum<S> {
  type S = (S, S);

  fn identity() -> Self::S {
    (S::zero(), S::zero())
  }

  fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
    (a + b, n + m)
  }
}

pub struct RangeAffineRangeSum<S>(PhantomData<S>);
impl<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One> MapMonoid for RangeAffineRangeSum<S> {
  type M = RangeSum<S>;
  type F = (S, S);

  fn identity_map() -> Self::F {
    (S::one(), S::zero())
  }

  fn mapping(&(m, a): &Self::F, &(s, n): &(S, S)) -> (S, S) {
    (m * s + a * n, n)
  }

  fn composition(&(m1, a1): &Self::F, &(m2, a2): &Self::F) -> Self::F {
    (m1 * m2, m1 * a2 + a1)
  }
}

/// Range Affine Range Sum
/// Element: (value, width); Map: (multiply, add)
pub fn range_affine_range_sum<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One>(n: usize) -> LazySegtree<RangeAffineRangeSum<S>> {
  LazySegtree::from(vec![(S::zero(), S::one()); n])
}
/// Range Affine Range Sum
/// Element: (value, width); Map: (multiply, add)
pub fn range_affine_range_sum_from<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One>(v: Vec<S>) -> LazySegtree<RangeAffineRangeSum<S>> {
  LazySegtree::from(v.into_iter().map(|x| (x, S::one())).collect::<Vec<_>>())
}

/// 遅延セグ木実装ヘルパー
pub trait LazySeg {
  /// 値
  type T: Clone;

  /// 作用
  type F: Clone;

  /// 二項演算
  fn op(x: Self::T, y: Self::T) -> Self::T;

  /// 二項演算の単位元
  fn op_id() -> Self::T;

  /// 作用
  fn map(f: Self::F, x: Self::T, n: usize) -> Self::T;

  /// 作用の単位元
  fn map_id() -> Self::F;

  /// 作用の合成
  fn map_compose(f: Self::F, g: Self::F) -> Self::F;

  /// 長さ `n` の遅延セグメント木を作る
  fn new(n: usize) -> LazySegtree<LazySegHelper<Self>> where Self: Sized {
    LazySegtree::from(vec![(Self::op_id(), 1); n])
  }

  /// `IntoIterator` から遅延セグメント木を作る
  fn from_iter(i: impl IntoIterator<Item = Self::T>) -> LazySegtree<LazySegHelper<Self>> where Self: Sized {
    LazySegtree::from(i.into_iter().map(|x| (x, 1)).collect::<Vec<_>>())
  }
}

pub struct LazySegHelper<L>(PhantomData<L>);
impl<L: LazySeg> Monoid for LazySegHelper<L> {
  type S = (L::T, usize);

  fn identity() -> (L::T, usize) {
    (L::op_id(), 0)
  }

  fn binary_operation((a, n): &Self::S, (b, m): &Self::S) -> Self::S {
    (L::op(a.clone(), b.clone()), *n + *m)
  }
}
impl<L: LazySeg> MapMonoid for LazySegHelper<L> {
  type M = LazySegHelper<L>;
  type F = L::F;

  fn identity_map() -> L::F {
    L::map_id()
  }

  fn mapping(f: &L::F, (x, n): &(L::T, usize)) -> (L::T, usize) {
    (L::map(f.clone(), x.clone(), *n), *n)
  }

  fn composition(f: &L::F, g: &L::F) -> L::F {
    L::map_compose(f.clone(), g.clone())
  }
}

pub struct RangeAddRangeSum<T>(PhantomData<T>);
impl<T: Copy + Add<Output = T> + Mul<Output = T> + TryFrom<usize> + Default> RangeMonoid for RangeAddRangeSum<T> where <T as TryFrom<usize>>::Error: std::fmt::Debug {
  type T = T;

  fn op(x: T, y: T) -> T {
    x + y
  }

  fn id() -> T {
    T::default()
  }

  fn pow(x: T, n: usize) -> T {
    x * T::try_from(n).unwrap()
  }
}

pub struct RangeAddRangeMin<T>(PhantomData<T>);
impl<T: Copy + Add<Output = T> + Default + Ord + BoundedAbove> LazySeg for RangeAddRangeMin<T> {
  type T = T;
  type F = T;

  fn op(x: T, y: T) -> T {
    x.min(y)
  }

  fn op_id() -> T {
    T::max_value()
  }

  fn map(f: T, x: T, _n: usize) -> T {
    f + x
  }

  fn map_id() -> T {
    T::default()
  }

  fn map_compose(f: T, g: T) -> T {
    f + g
  }
}

pub struct RangeAddRangeMax<T>(PhantomData<T>);
impl<T: Copy + Add<Output = T> + Default + Ord + BoundedBelow> LazySeg for RangeAddRangeMax<T> {
  type T = T;
  type F = T;

  fn op(x: T, y: T) -> T {
    x.max(y)
  }

  fn op_id() -> T {
    T::min_value()
  }

  fn map(f: T, x: T, _n: usize) -> T {
    f + x
  }

  fn map_id() -> T {
    T::default()
  }

  fn map_compose(f: T, g: T) -> T {
    f + g
  }
}

/// Range Add Range Sum 系の、演算と作用が同じときのセグ木
pub trait RangeMonoid {
  /// 値
  type T: Clone;

  /// 演算
  fn op(x: Self::T, y: Self::T) -> Self::T;

  /// 単位元
  fn id() -> Self::T;

  /// 繰返し（範囲に作用させるときに使う）
  fn pow(mut x: Self::T, mut n: usize) -> Self::T {
    let mut r = Self::id();
    while n != 0 {
      if (n & 1) != 0 {
        r = Self::op(r.clone(), x.clone());
      }
      x = Self::op(x.clone(), x.clone());
      n >>= 1;
    }
    r
  }
}
impl<M: RangeMonoid> LazySeg for M {
  type T = M::T;
  type F = M::T;

  fn op(x: Self::T, y: Self::T) -> Self::T {
    <M as RangeMonoid>::op(x, y)
  }

  fn op_id() -> Self::T {
    M::id()
  }

  fn map(f: Self::F, x: Self::T, n: usize) -> Self::T {
    <M as RangeMonoid>::op(M::pow(f, n), x)
  }

  fn map_id() -> Self::F {
    M::id()
  }

  fn map_compose(f: Self::F, g: Self::F) -> Self::F {
    <M as RangeMonoid>::op(f, g)
  }
}