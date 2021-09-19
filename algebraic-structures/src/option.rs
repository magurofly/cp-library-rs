use std::marker::PhantomData;
use super::group::*;
use super::group_props::*;

/// 半群を `Option` でラップしてモノイドにする
pub struct OptionMonoid<T>(PhantomData<T>);

impl<T: Magma> Magma for OptionMonoid<T> {
  type S = Option<T::S>;

  fn operator(a: &Self::S, b: &Self::S) -> Self::S {
    if a.is_none() {
      b.clone()
    } else if b.is_none() {
      a.clone()
    } else {
      Some(T::operator(a.as_ref().unwrap(), b.as_ref().unwrap()))
    }
  }
}

impl<T: Magma> Identity for OptionMonoid<T> {
  /// 単位元は `None`
  fn identity() -> Self::S {
    None
  }
}

impl<T: Commutativity> Commutativity for OptionMonoid<T> {}

impl<T: Associativity> Associativity for OptionMonoid<T> {}