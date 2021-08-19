use super::*;
use std::{marker::PhantomData, ops::*};
use num_traits::*;

macro_rules! impl_semigroup {
  ($Name:ident, $T: ident, [$($traits:tt)*], $op:item) => {
    pub struct $Name<$T>(PhantomData<$T>);
    impl<$T: $($traits)*> SegSemiGroup for $Name<$T> {
      type T = $T;
      $op
    }
  }
}

macro_rules! impl_monoid {
  ($Name:ident, $T:ident, [$($traits:tt)*], {$($impls:item)*}) => {
    pub struct $Name<$T>(PhantomData<$T>);
    impl<$T> SegMonoid for $Name<$T> where $($traits)* {
      type T = $T;
      $($impls)*
    }
  }
}

impl_monoid!(RangeSum, T, [T: Clone + Add<Output = T> + Mul<Output = T> + Zero + NumCast], {
  fn op(x: T, y: T) -> T { x + y }
  fn id() -> T { T::zero() }
  fn pow(x: T, n: usize) -> T { x * T::from(n).unwrap() }
});

impl_monoid!(RangeMul, T, [T: Clone + Mul<Output = T> + Pow<usize, Output = T> + One], {
  fn op(x: T, y: T) -> T { x * y }
  fn id() -> T { T::one() }
  fn pow(x: T, n: usize) -> T { x.pow(n) }
});

impl_monoid!(RangeMin, T, [T: Clone + Ord + Bounded], {
  fn op(x: T, y: T) -> T { x.min(y) }
  fn id() -> T { T::max_value() }
  fn pow(x: T, _n: usize) -> T { x }
});

impl_monoid!(RangeMax, T, [T: Clone + Ord + Bounded], {
  fn op(x: T, y: T) -> T { x.max(y) }
  fn id() -> T { T::min_value() }
  fn pow(x: T, _n: usize) -> T { x }
});

impl_semigroup!(RangeSemiSum, T, [Clone + Add<Output = T>], fn op(x: T, y: T) -> T { x + y });
impl_semigroup!(RangeSemiMul, T, [Clone + Mul<Output = T>], fn op(x: T, y: T) -> T { x * y });
impl_semigroup!(RangeSemiAnd, T, [Clone + BitAnd<Output = T>], fn op(x: T, y: T) -> T { x & y });
impl_semigroup!(RangeSemiXor, T, [Clone + BitXor<Output = T>], fn op(x: T, y: T) -> T { x ^ y });
impl_semigroup!(RangeSemiOr, T, [Clone + BitOr<Output = T>], fn op(x: T, y: T) -> T { x | y });
impl_semigroup!(RangeSemiMin, T, [Clone + Ord], fn op(x: T, y: T) -> T { x.min(y) });
impl_semigroup!(RangeSemiMax, T, [Clone + Ord], fn op(x: T, y: T) -> T { x.max(y) });

pub struct RangeIdempotentMapAdd<T>(PhantomData<T>);
impl<T: Clone + Add<Output = T>> SegMap for RangeIdempotentMapAdd<T> {
  type T = Option<T>;
  type F = Option<T>;
  fn map(f: Option<T>, x: Option<T>, _n: usize) -> Option<T> { f.and_then(|f| x.clone().map(|x| f + x)).or(x) }
  fn map_id() -> Option<T> { None }
  fn map_compose(f: Option<T>, g: Option<T>) -> Option<T> { f.map(|f| g.clone().map(|g| f.clone() + g).unwrap_or(f)).or(g) }
}

pub type RangeAddRangeMin<T> = LazySegHelper<RangeMin<T>, RangeIdempotentMapAdd<T>>;
pub type RangeAddRangeMax<T> = LazySegHelper<RangeMax<T>, RangeIdempotentMapAdd<T>>;

pub struct RangeIdempotentMapUpdate<T>(PhantomData<T>);
impl<T: Clone> SegMap for RangeIdempotentMapUpdate<T> {
  type T = T;
  type F = Option<T>;
  fn map(f: Option<T>, x: T, _n: usize) -> T { f.unwrap_or(x) }
  fn map_id() -> Option<T> { None }
  fn map_compose(f: Option<T>, g: Option<T>) -> Option<T> { f.or(g) }
}

pub type RangeUpdateRangeMin<T> = LazySegHelper<RangeMin<T>, RangeIdempotentMapUpdate<T>>;
pub type RangeUpdateRangeMax<T> = LazySegHelper<RangeMax<T>, RangeIdempotentMapUpdate<T>>;

pub struct RangeUpdateRangeSum<T>(PhantomData<T>);
impl<T: Clone + Add<Output = T> + Mul<Output = T> + Zero + NumCast> LazySeg for RangeUpdateRangeSum<T> {
  type T = T;
  type F = Option<T>;
  fn op(x: T, y: T) -> T { x + y }
  fn op_id() -> T { T::zero() }
  fn map(f: Option<T>, x: T, n: usize) -> T { f.map(|f| f * T::from(n).unwrap()).unwrap_or(x) }
  fn map_id() -> Option<T> { None }
  fn map_compose(f: Option<T>, g: Option<T>) -> Option<T> { f.or(g) }
}

pub struct RangeMapAffine<T>(PhantomData<T>);
impl<T: Clone + Add<Output = T> + Mul<Output = T> + Zero + One + NumCast> SegMap for RangeAffineRangeSum<T> {
  type T = T;
  type F = (T, T);
  fn map((m, a): (T, T), x: T, n: usize) -> T { m * x + a * T::from(n).unwrap() }
  fn map_id() -> (T, T) { (T::one(), T::zero()) }
  fn map_compose((m1, a1): Self::F, (m2, a2): Self::F) -> Self::F { (m1.clone() * m2, m1 * a2 + a1) }
}

pub type RangeAffineRangeSum<T> = LazySegHelper<RangeSum<T>, RangeMapAffine<T>>;
