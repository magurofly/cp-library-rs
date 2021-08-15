use std::{cell::RefCell, marker::PhantomData};
use number::*;
use acl_modint::*;

pub struct CombinationMod<N> {
  f: RefCell<FactorialInvMod<N>>,
  modulus: N,
}
impl<N: Int> CombinationMod<N> {
  pub fn new(modulus: N) -> Self {
    Self {
      f: RefCell::new(FactorialInvMod::empty(modulus)),
      modulus,
    }
  }

  pub fn modulus(&self) -> N {
    self.modulus
  }

  pub fn ensure(&self, n: impl Int) {
    self.f.borrow_mut().ensure(n.as_usize());
  }

  /// 階乗
  pub fn fact(&self, n: impl Int) -> N {
    self.ensure(n);
    self.f.borrow().fact(n)
  }

  /// 逆数
  pub fn inv(&self, n: impl Int) -> N {
    self.ensure(n);
    self.f.borrow().inv(n)
  }

  /// 階乗の逆数
  pub fn fact_inv(&self, n: impl Int) -> N {
    self.ensure(n);
    self.f.borrow().fact_inv(n)
  }

  /// 順列
  pub fn perm<M: Int>(&self, n: M, r: M) -> N {
    if r.is_negative() || n < r {
      return N::zero();
    }

    self.ensure(n);

    self.fact(n) * self.fact_inv(n - r) % self.modulus
  }

  /// 二項係数（組合せ）
  pub fn binom<M: Int>(&self, n: M, r: M) -> N {
    if n < r {
      return N::zero();
    }

    if n.is_negative() {
      // 負の二項係数
      return if r.is_even() {
        self.homo(M::zero() - n, r)
      } else {
        N::zero() - self.homo(M::zero() - n, r)
      };
    }

    self.perm(n, r) * self.fact_inv(r) % self.modulus()
  }

  /// 重複組合せ
  pub fn homo<M: Int>(&self, n: M, r: M) -> N {
    if n.is_negative() || r.is_negative() {
      return N::zero();
    }

    if r.is_zero() {
      return N::one();
    }

    self.binom(n + r - M::one(), r)
  }
}

pub struct Combination<T> {
  f: CombinationMod<i64>,
  phantom: PhantomData<T>,
}

impl<T: ModIntBase> Combination<T> {
  pub fn new() -> Self {
    Self { f: CombinationMod::new(T::modulus() as i64), phantom: PhantomData, }
  }

  pub fn inner(&self) -> &CombinationMod<i64> {
    &self.f
  }

  /// 階乗
  pub fn fact(&self, n: impl Int) -> T {
    T::from(self.f.fact(n))
  }

  /// 逆数
  pub fn inv(&self, n: impl Int) -> T {
    T::from(self.f.inv(n))
  }

  /// 階乗の逆数
  pub fn fact_inv(&self, n: impl Int) -> T {
    T::from(self.f.fact_inv(n))
  }

  /// 順列
  pub fn perm<U: Int>(&self, n: U, r: U) -> T {
    T::from(self.f.perm(n, r))
  }

  /// 二項係数（組合せ）
  pub fn binom<U: Int>(&self, n: U, r: U) -> T {
    T::from(self.f.binom(n, r))
  }

  /// 重複組合せ
  pub fn homo<U: Int>(&self, n: U, r: U) -> T {
    T::from(self.f.homo(n, r))
  }
}