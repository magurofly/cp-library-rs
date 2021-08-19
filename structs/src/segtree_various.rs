use super::*;
use std::{marker::PhantomData, ops::*};

pub struct RangeSum<T>(PhantomData<T>);
impl<T: Clone + Add<Output = T>> SegSemiGroup for RangeSum<T> {
  type T = T;

  fn op(x: T, y: T) -> T {
    x + y
  }
}

pub struct RangeMul<T>(PhantomData<T>);
impl<T: Clone + Mul<Output = T>> SegSemiGroup for RangeMul<T> {
  type T = T;

  fn op(x: T, y: T) -> T {
    x * y
  }
}

pub struct RangeMin<T>(PhantomData<T>);
impl<T: Clone + Ord> SegSemiGroup for RangeMin<T> {
  type T = T;

  fn op(x: T, y: T) -> T {
    x.min(y)
  }
}

pub struct RangeMax<T>(PhantomData<T>);
impl<T: Clone + Ord> SegSemiGroup for RangeMax<T> {
  type T = T;

  fn op(x: T, y: T) -> T {
    x.max(y)
  }
}


// pub struct RangeAddRangeSum<T>(PhantomData<T>);
// impl<T: Copy + Add<Output = T> + Mul<Output = T> + TryFrom<usize> + Default> SegMonoid for RangeAddRangeSum<T> where <T as TryFrom<usize>>::Error: std::fmt::Debug {
//   type T = T;

//   fn op(x: T, y: T) -> T {
//     x + y
//   }

//   fn id() -> T {
//     T::default()
//   }

//   fn pow(x: T, n: usize) -> T {
//     x * T::try_from(n).unwrap()
//   }
// }

// pub struct RangeAddRangeMin<T>(PhantomData<T>);
// impl<T: Copy + Add<Output = T> + Default + Ord + BoundedAbove> LazySeg for RangeAddRangeMin<T> {
//   type T = T;
//   type F = T;

//   fn op(x: T, y: T) -> T {
//     x.min(y)
//   }

//   fn op_id() -> T {
//     T::max_value()
//   }

//   fn map(f: T, x: T, _n: usize) -> T {
//     f + x
//   }

//   fn map_id() -> T {
//     T::default()
//   }

//   fn map_compose(f: T, g: T) -> T {
//     f + g
//   }
// }

// pub struct RangeAddRangeMax<T>(PhantomData<T>);
// impl<T: Copy + Add<Output = T> + Default + Ord + BoundedBelow> LazySeg for RangeAddRangeMax<T> {
//   type T = T;
//   type F = T;

//   fn op(x: T, y: T) -> T {
//     x.max(y)
//   }

//   fn op_id() -> T {
//     T::min_value()
//   }

//   fn map(f: T, x: T, _n: usize) -> T {
//     f + x
//   }

//   fn map_id() -> T {
//     T::default()
//   }

//   fn map_compose(f: T, g: T) -> T {
//     f + g
//   }
// }

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
