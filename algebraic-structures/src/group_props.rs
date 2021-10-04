use super::group::*;

/// 可換性
/// `operator(&x, &y) == operator(&y, &x)` が成り立つ
pub trait Commutativity: Magma {}

/// 結合性
/// `operator(&x, operator(&y, &z)) == operator(operator(&x, &y), &z)` が成り立つ
pub trait Associativity: Magma {}

/// 単位元
pub trait Identity: Magma {
  /// 単位元を返す
  /// `operator(&x, identity()) == x` かつ `operator(identity(), &x) == x` が成り立つ
  fn identity() -> Self::S;
}

/// 逆元
pub trait Inverse: Magma {
  /// 逆元を返す
  /// `operator(&x, inverse(&x)) == identity()` かつ `operator(inverse(&x), &x) == identity()` が成り立つ
  fn inverse(a: &Self::S) -> Self::S;
}

/// 冪等
pub trait Idempotence: Magma {}