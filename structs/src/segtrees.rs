use acl_segtree::*;
use acl_lazysegtree::*;
use acl_traits::*;
use std::marker::PhantomData;

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
impl_prepared!(S, [S: Copy + std::ops::Add<Output = S> + Zero], Additive<S>, range_sum, range_sum_from);

// Range Multiple
impl_prepared!(S, [S: Copy + std::ops::Mul<Output = S> + One], Multiplicative<S>, range_mul, range_mul_from);

// Range Min
impl_prepared!(S, [S: Copy + Ord + BoundedAbove], Min<S>, range_min, range_min_from);

// Range Max
impl_prepared!(S, [S: Copy + Ord + BoundedBelow], Max<S>, range_max, range_max_from);

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
