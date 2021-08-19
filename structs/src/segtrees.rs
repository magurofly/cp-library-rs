//! セグメント木の実装をしやすくするヘルパー群
//! 
//! # 実装例
//! 
//! ```
//! use structs::segtrees::*;
//!
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
//! let mut seg1 = RangeAddRangeMin::new_lazysegtree(10);
//! let mut seg2 = RangeAddRangeMin::lazysegtree_from_iter(vec![1, 2, 3, 4, 5]);
//! 
//! ```

use acl_segtree::*;
use acl_lazysegtree::*;
use std::marker::PhantomData;

/// セグ木実装ヘルパー
pub trait Seg {
  /// 値
  type T: Clone;

  /// 二項演算
  fn op(x: Self::T, y: Self::T) -> Self::T;

  /// 二項演算の単位元
  fn op_id() -> Self::T;

  /// 長さ `n` のセグメント木を作る
  fn new_segtree(n: usize) -> Segtree<SegHelper<Self>> where Self: Sized {
    Segtree::from(vec![(Self::op_id(), 1); n])
  }

  /// 長さ `n` 、要素が全て `init` のセグメント木を作る
  fn new_segtree_of(n: usize, init: Self::T) -> Segtree<SegHelper<Self>> where Self: Sized {
    Segtree::from(vec![(init, 1); n])
  }

  /// `IntoIterator` からセグメント木を作る
  fn segtree_from_iter(i: impl IntoIterator<Item = Self::T>) -> Segtree<SegHelper<Self>> where Self: Sized {
    Segtree::from(i.into_iter().map(|x| (x, 1)).collect::<Vec<_>>())
  }
}

/// 遅延セグ木作用ヘルパー
pub trait SegMap {
  /// 値
  type T: Clone;

  /// 作用
  type F: Clone;

  /// 作用
  fn map(f: Self::F, x: Self::T, n: usize) -> Self::T;

  /// 作用の単位元
  fn map_id() -> Self::F;

  /// 作用の合成
  fn map_compose(f: Self::F, g: Self::F) -> Self::F;
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
}
impl<L: LazySeg> Seg for L {
  type T = L::T;
  fn op(x: L::T, y: L::T) -> L::T { L::op(x, y) }
  fn op_id() -> L::T { L::op_id() }
}
impl<L: LazySeg> SegMap for L {
  type T = L::T;
  type F = L::F;
  fn map(f: L::F, x: L::T, n: usize) -> L::T { L::map(f, x, n) }
  fn map_compose(f: L::F, g: L::F) -> L::F { L::map_compose(f, g) }
  fn map_id() -> L::F { L::map_id() }
}

pub trait LazySegInit<T: Clone> : Seg<T = T> + SegMap<T = T> {
  /// 長さ `n` の遅延セグメント木を作る
  fn new_lazysegtree(n: usize) -> LazySegtree<LazySegHelper<Self, Self>> where Self: Sized {
    LazySegtree::from(vec![(Self::op_id(), 1); n])
  }

  /// 長さ `n` 、要素が全て `init` のセグメント木を作る
  fn new_lazysegtree_of(n: usize, init: T) -> LazySegtree<LazySegHelper<Self, Self>> where Self: Sized {
    LazySegtree::from(vec![(init, 1); n])
  }

  /// `IntoIterator` から遅延セグメント木を作る
  fn lazysegtree_from_iter(i: impl IntoIterator<Item = T>) -> LazySegtree<LazySegHelper<Self, Self>> where Self: Sized {
    LazySegtree::from(i.into_iter().map(|x| (x, 1)).collect::<Vec<_>>())
  }
}
impl<T: Clone, L: Seg<T = T> + SegMap<T = T>> LazySegInit<T> for L {}

pub struct SegHelper<L>(PhantomData<L>);
impl<L: Seg> Monoid for SegHelper<L> {
  type S = (L::T, usize);
  fn identity() -> (L::T, usize) { (L::op_id(), 0) }
  fn binary_operation((a, n): &Self::S, (b, m): &Self::S) -> Self::S { (L::op(a.clone(), b.clone()), *n + *m) }
}

pub struct LazySegHelper<M, F>(PhantomData<(M, F)>);
impl<T: Clone, M: Seg<T = T>, F: SegMap<T = T>> Monoid for LazySegHelper<M, F> {
  type S = (M::T, usize);
  fn identity() -> (M::T, usize) { (M::op_id(), 0) }
  fn binary_operation((a, n): &Self::S, (b, m): &Self::S) -> Self::S { (M::op(a.clone(), b.clone()), *n + *m) }
}
impl<T: Clone, M: Seg<T = T>, F: SegMap<T = T>> MapMonoid for LazySegHelper<M, F> {
  type M = SegHelper<M>;
  type F = F::F;
  fn identity_map() -> F::F { F::map_id() }
  fn mapping(f: &F::F, (x, n): &(F::T, usize)) -> (F::T, usize) { (F::map(f.clone(), x.clone(), *n), *n) }
  fn composition(f: &F::F, g: &F::F) -> F::F { F::map_compose(f.clone(), g.clone()) }
}

/// Range Add Range Sum 系の、演算と作用が同じときのセグ木
pub trait SegMonoid {
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
impl<M: SegMonoid> LazySeg for M {
  type T = M::T;
  type F = M::T;

  fn op(x: Self::T, y: Self::T) -> Self::T {
    <M as SegMonoid>::op(x, y)
  }

  fn op_id() -> Self::T {
    M::id()
  }

  fn map(f: Self::F, x: Self::T, n: usize) -> Self::T {
    <M as SegMonoid>::op(M::pow(f, n), x)
  }

  fn map_id() -> Self::F {
    M::id()
  }

  fn map_compose(f: Self::F, g: Self::F) -> Self::F {
    <M as SegMonoid>::op(f, g)
  }
}

pub trait SegSemiGroup {
  /// 値
  type T: Clone;

  /// 演算
  fn op(x: Self::T, y: Self::T) -> Self::T;
}
impl<M: SegSemiGroup> SegMonoid for M {
  type T = Option<M::T>;

  fn op(x: Self::T, y: Self::T) -> Self::T {
    if let Some(x) = x {
      if let Some(y) = y {
        Some(M::op(x, y))
      } else {
        Some(x)
      }
    } else {
      y
    }
  }

  fn id() -> Self::T {
    None
  }
}