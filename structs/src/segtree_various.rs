use super::*;
use std::{marker::PhantomData, ops::*};

macro_rules! impl_semigroup {
  ($Name:ident, $T: ident, [$($traits:tt)*], $op:item) => {
    pub struct $Name<$T>(PhantomData<$T>);
    impl<$T: $($traits)*> SegSemiGroup for $Name<$T> {
      type T = $T;
      $op
    }
  }
}

impl_semigroup!(RangeSum, T, [Clone + Add<Output = T>], fn op(x: T, y: T) -> T { x + y });
impl_semigroup!(RangeMul, T, [Clone + Mul<Output = T>], fn op(x: T, y: T) -> T { x * y });
impl_semigroup!(RangeAnd, T, [Clone + BitAnd<Output = T>], fn op(x: T, y: T) -> T { x & y });
impl_semigroup!(RangeXor, T, [Clone + BitXor<Output = T>], fn op(x: T, y: T) -> T { x ^ y });
impl_semigroup!(RangeOr, T, [Clone + BitOr<Output = T>], fn op(x: T, y: T) -> T { x | y });
impl_semigroup!(RangeMin, T, [Clone + Ord], fn op(x: T, y: T) -> T { x.min(y) });
impl_semigroup!(RangeMax, T, [Clone + Ord], fn op(x: T, y: T) -> T { x.max(y) });

pub struct RangeIdempotentAdd<T>(PhantomData<T>);
impl<T: Clone + Add<Output = T>> SegMap for RangeAddRangeMin<T> {
  type T = Option<T>;
  type F = Option<T>;
  fn map(f: Option<T>, x: Option<T>, _n: usize) -> Option<T> { f.and_then(|f| x.clone().map(|x| f + x)).or(x) }
  fn map_id() -> Option<T> { None }
  fn map_compose(f: Option<T>, g: Option<T>) -> Option<T> { f.map(|f| g.clone().map(|g| f.clone() + g).unwrap_or(f)).or(g) }
}

pub type RangeAddRangeMin<T> = LazySegHelper<RangeMin<T>, RangeIdempotentAdd<T>>;
pub type RangeAddRangeMax<T> = LazySegHelper<RangeMax<T>, RangeIdempotentAdd<T>>;

// pub struct RangeAffineRangeSum<T>(PhantomData<T>);
// impl<T: Clone + Add<Output = T> + Mul<Output = T> + Zero + One> LazySeg for RangeAffineRangeSum<T> {
//   type T = T;
//   type F = T;

//   fn op(x: T, y: T) -> T {
//     x + y
//   }

//   fn op_id() -> T {
//     T::zero()
//   }

//   fn map(f: T, x: T, n: usize) -> T {

//   }
// }
// impl<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One> MapMonoid for RangeAffineRangeSum<S> {
//   type M = RangeSum<S>;
//   type F = (S, S);

//   fn identity_map() -> Self::F {
//     (S::one(), S::zero())
//   }

//   fn mapping(&(m, a): &Self::F, &(s, n): &(S, S)) -> (S, S) {
//     (m * s + a * n, n)
//   }

//   fn composition(&(m1, a1): &Self::F, &(m2, a2): &Self::F) -> Self::F {
//     (m1 * m2, m1 * a2 + a1)
//   }
// }

// macro_rules! impl_prepared {
//   ($S:ident, [$($traits:tt)*], $monoid:ty, $name:ident, $name_vec:ident) => {
//     pub fn $name<$S>(n: usize) -> Segtree<$monoid> where $($traits)* {
//       Segtree::new(n)
//     }

//     pub fn $name_vec<$S>(v: Vec<$S>) -> Segtree<$monoid> where $($traits)* {
//       Segtree::from(v)
//     }
//   }
// }

// // Range Sum
// impl_prepared!(S, [S: Copy + Add<Output = S> + Zero], Additive<S>, range_sum, range_sum_from);

// // Range Multiple
// impl_prepared!(S, [S: Copy + Mul<Output = S> + One], Multiplicative<S>, range_mul, range_mul_from);

// // Range Min
// impl_prepared!(S, [S: Copy + Ord + BoundedAbove], Min<S>, range_min, range_min_from);

// // Range Max
// impl_prepared!(S, [S: Copy + Ord + BoundedBelow], Max<S>, range_max, range_max_from);

// pub struct RangeMin<S>(PhantomData<S>);
// impl<S: Copy + Ord + BoundedAbove> Monoid for RangeMin<S> {
//   type S = IndexedOrd<S>;

//   fn identity() -> Self::S {
//     IndexedOrd::without_index(<S as BoundedAbove>::max_value())
//   }

//   fn binary_operation(&x: &Self::S, &y: &Self::S) -> Self::S {
//     use std::cmp::Ordering::*;
//     match x.cmp(&y) {
//       Less => x,
//       Greater => y,
//       Equal => {
//         if let Some(i) = x.index() {
//           if let Some(j) = y.index() {
//             let k = i.min(j);
//             IndexedOrd::with_index(x.value(), k)
//           } else {
//             x
//           }
//         } else {
//           y
//         }
//       },
//     }
//   }
// }

// pub struct RangeMax<S>(PhantomData<S>);
// impl<S: Copy + Ord + BoundedBelow> Monoid for RangeMax<S> {
//   type S = IndexedOrd<S>;

//   fn identity() -> Self::S {
//     IndexedOrd::without_index(<S as BoundedBelow>::min_value())
//   }

//   fn binary_operation(&x: &Self::S, &y: &Self::S) -> Self::S {
//     use std::cmp::Ordering::*;
//     match x.cmp(&y) {
//       Greater => x,
//       Less => y,
//       Equal => {
//         if let Some(i) = x.index() {
//           if let Some(j) = y.index() {
//             let k = i.min(j);
//             IndexedOrd::with_index(x.value(), k)
//           } else {
//             x
//           }
//         } else {
//           y
//         }
//       },
//     }
//   }
// }

// pub struct RangeSum<S>(PhantomData<S>);
// impl<S: Copy + std::ops::Add<Output = S> + Zero + One> Monoid for RangeSum<S> {
//   type S = (S, S);

//   fn identity() -> Self::S {
//     (S::zero(), S::zero())
//   }

//   fn binary_operation(&(a, n): &Self::S, &(b, m): &Self::S) -> Self::S {
//     (a + b, n + m)
//   }
// }

// pub struct RangeAffineRangeSum<S>(PhantomData<S>);
// impl<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One> MapMonoid for RangeAffineRangeSum<S> {
//   type M = RangeSum<S>;
//   type F = (S, S);

//   fn identity_map() -> Self::F {
//     (S::one(), S::zero())
//   }

//   fn mapping(&(m, a): &Self::F, &(s, n): &(S, S)) -> (S, S) {
//     (m * s + a * n, n)
//   }

//   fn composition(&(m1, a1): &Self::F, &(m2, a2): &Self::F) -> Self::F {
//     (m1 * m2, m1 * a2 + a1)
//   }
// }

// /// Range Affine Range Sum
// /// Element: (value, width); Map: (multiply, add)
// pub fn range_affine_range_sum<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One>(n: usize) -> LazySegtree<RangeAffineRangeSum<S>> {
//   LazySegtree::from(vec![(S::zero(), S::one()); n])
// }
// /// Range Affine Range Sum
// /// Element: (value, width); Map: (multiply, add)
// pub fn range_affine_range_sum_from<S: Copy + std::ops::Add<Output = S> + std::ops::Mul<Output = S> + Zero + One>(v: Vec<S>) -> LazySegtree<RangeAffineRangeSum<S>> {
//   LazySegtree::from(v.into_iter().map(|x| (x, S::one())).collect::<Vec<_>>())
// }
