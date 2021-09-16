use std::ops::*;
use super::group::*;
use super::group_props::*;

// macros

#[macro_export]
/// 半群を定義する
/// `impl_semigroup!(name, S, [Clone + ...traits], |a, b| operator(a, b))`
macro_rules! impl_semigroup {
  ($name:ident, $value:ident, [$($bound:tt)*], |$a:ident, $b:ident| $op:expr) => {
    pub struct $name<$value: $($bound)*>(std::marker::PhantomData<S>);

    impl<$value: $($bound)*> Magma for $name<$value> {
      type S = $value;

      fn operator($a: &Self::S, $b: &Self::S) -> Self::S { $op }
    }

    impl<$value: $($bound)*> Associativity for $name<$value> {}
  };
}

#[macro_export]
/// モノイドを定義する
/// `impl_monoid!(name, S, [Clone + ...traits], |a, b| operator(a, b), || identity())`
macro_rules! impl_monoid {
  ($name:ident, $value:ident, [$($bound:tt)*], |$a:ident, $b:ident| $op:expr, || $id:expr) => {
    impl_semigroup!($name, $value, [$($bound)*], |$a, $b| $op);

    impl<$value: $($bound)*> Identity for $name<$value> {
      fn identity() -> Self::S { $id }
    }
  };
}

#[macro_export]
/// 群を定義する
/// `impl_group!(name, S, [Clone + ...traits], |a, b| operator(a, b), || identity(), |a| inverse(a))`
macro_rules! impl_group {
  ($name:ident, $value:ident, [$($bound:tt)*], |$a:ident, $b:ident| $op:expr, || $id:expr, |$c:ident| $inv:expr) => {
    impl_monoid!($name, $value, [$($bound)*], |$a, $b| $op, || $id);

    impl<$value: $($bound)*> Inverse for $name<$value> {
      fn inverse($c: &Self::S) -> Self::S { $inv }
    }
  };
}

// impls

impl_semigroup!(AddSemigroup, S, [Clone + Add<Output = S>], |a, b| a.clone() + b.clone());
impl_monoid!(AddMonoid, S, [Clone + From<u8> + Add<Output = S>], |a, b| a.clone() + b.clone(), || S::from(0u8));
impl_group!(AddGroup, S, [Clone + From<u8> + Add<Output = S> + Neg<Output = S>], |a, b| a.clone() + b.clone(), || S::from(0u8), |a| -a.clone());

impl_semigroup!(MulSemigroup, S, [Clone + Mul<Output = S>], |a, b| a.clone() * b.clone());
impl_monoid!(MulMonoid, S, [Clone + From<u8> + Mul<Output = S>], |a, b| a.clone() * b.clone(), || S::from(1u8));
impl_group!(MulGroup, S, [Clone + From<u8> + Mul<Output = S> + Div<Output = S>], |a, b| a.clone() * b.clone(), || S::from(1u8), |a| S::from(1u8) / a.clone());

impl_semigroup!(MinSemigroup, S, [Clone + Ord], |a, b| a.min(b).clone());
impl_semigroup!(MaxSemigroup, S, [Clone + Ord], |a, b| a.max(b).clone());

impl_semigroup!(BitAndSemigroup, S, [Clone + BitAnd<Output = S>], |a, b| a.clone() & b.clone());
impl<S: Clone + BitAnd<Output = S>> Idempotence for BitAndSemigroup<S> {}

impl_semigroup!(BitOrSemigroup, S, [Clone + BitOr<Output = S>], |a, b| a.clone() | b.clone());
impl<S: Clone + BitOr<Output = S>> Idempotence for BitOrSemigroup<S> {}
impl_monoid!(BitOrMonoid, S, [Clone + From<u8> + BitOr<Output = S>], |a, b| a.clone() | b.clone(), || S::from(0u8));
impl<S: Clone + From<u8> + BitOr<Output = S>> Idempotence for BitOrMonoid<S> {}

impl_semigroup!(BitXorSemigroup, S, [Clone + BitXor<Output = S>], |a, b| a.clone() ^ b.clone());
impl_monoid!(BitXorMonoid, S, [Clone + From<u8> + BitXor<Output = S>], |a, b| a.clone() ^ b.clone(), || S::from(0u8));
impl_group!(BitXorGroup, S, [Clone + From<u8> + BitXor<Output = S>], |a, b| a.clone() ^ b.clone(), || S::from(0u8), |a| a.clone());