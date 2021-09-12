use std::{cell::RefCell, collections::*, marker::PhantomData, rc::*};
use number::*;
use super::*;
use acl_modint::*;

thread_local!(pub static ENUMERATION_CACHE: RefCell<HashMap<u32, Weak<EnumerationMod<i64>>>> = RefCell::new(HashMap::new()));

pub struct EnumerationMod<N> {
  f: RefCell<FactorialInvMod<N>>,
  p: RefCell<PartitionTableMod<N>>,
  modulus: N,
}
impl<N: Int> EnumerationMod<N> {
  pub fn new(modulus: N) -> Self {
    Self {
      f: RefCell::new(FactorialInvMod::empty(modulus)),
      p: RefCell::new(PartitionTableMod::empty(modulus)),
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
    if n.as_usize() < self.f.borrow().len() || n.as_usize() <= 1_000_000 {
      self.ensure(n);
      self.f.borrow().inv(n)
    } else {
      n.cast::<N>().inv_mod(self.modulus)
    }
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

    if n.as_usize() < self.f.borrow().len() || n.as_usize() <= 10_000_000 {
      self.perm(n, r) * self.fact_inv(r) % self.modulus()
    } else {
      let mut x = self.fact_inv(r);
      for i in 0 .. r.as_usize() {
        x = x * (n.as_usize() - i).cast() % self.modulus;
      }
      x
    }
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
  
  /// カタラン数 <O(n), O(log n)>
  pub fn catalan(&self, n: impl Int) -> N {
    self.binom(n + n, n) * self.inv(n.add1()) % self.modulus
  }

  /// 分割数 <O(nk), O(1)>
  pub fn partition(&self, n: impl Int, k: impl Int) -> N {
    let n = n.as_usize();
    let k = k.as_usize();
    self.p.borrow_mut().ensure(n, k);
    self.p.borrow().partition(n, k)
  }

  /// 第二種スターリング数 <O(k), O(k log n)>
  pub fn stirling2(&self, n: impl Int, k: impl Int) -> N {
    let n = n.as_usize();
    let k = k.as_usize();
    let mut ret = N::zero();
    for i in 0 ..= k {
      let add = i.cast::<N>().pow_mod(n, self.modulus) * self.binom(k, i) % self.modulus;
      if (k - i).is_even() {
        ret = ret + add;
        if ret >= self.modulus {
          ret = ret - self.modulus;
        }
      } else {
        ret = (ret - add) % self.modulus;
        if ret.is_negative() {
          ret = ret + self.modulus;
        }
      }
    }
    ret * self.fact_inv(k) % self.modulus
  }

  /// ベル数 <O(k), O(min(n, k) log n)>
  pub fn bell(&self, n: impl Int, k: impl Int) -> N {
    if n.is_zero() {
      return N::one();
    }
    let n = n.as_usize();
    let k = k.as_usize();
    let mut ret = N::zero();
    let mut pref = vec![N::zero(); k + 1];
    pref[0] = N::one();
    for i in 1 ..= k {
      if i.is_even() {
        pref[i] = pref[i - 1] + self.fact_inv(i);
        if pref[i] >= self.modulus {
          pref[i] = pref[i] - self.modulus;
        }
      } else {
        pref[i] = pref[i - 1] - self.fact_inv(i);
        if pref[i].is_negative() {
          pref[i] = pref[i] + self.modulus;
        }
      }
    }
    for i in 1 ..= k {
      ret = (ret + i.cast::<N>().pow_mod(n, self.modulus()) * self.fact_inv(i) % self.modulus * pref[k - i] % self.modulus) % self.modulus;
    }
    ret
  }
}

#[derive(Clone)]
pub struct Enumeration<T> {
  f: Rc<EnumerationMod<i64>>,
  phantom: PhantomData<T>,
}

impl<T: ModIntBase> Enumeration<T> {
  pub fn new() -> Self {
    let f = Self::request().unwrap_or_else(|| {
      let f = Rc::new(EnumerationMod::new(T::modulus() as i64));
      Self::register(&f);
      f
    });
    Self { f, phantom: PhantomData, }
  }

  fn register(f: &Rc<EnumerationMod<i64>>) {
    ENUMERATION_CACHE.with(|h| h.borrow_mut().insert(T::modulus(), Rc::downgrade(f)));
  }

  fn request() -> Option<Rc<EnumerationMod<i64>>> {
    ENUMERATION_CACHE.with(|m| m.borrow().get(&T::modulus()).and_then(Weak::upgrade))
  }

  pub fn inner(&self) -> &EnumerationMod<i64> {
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

  /// カタラン数
  pub fn catalan(&self, n: impl Int) -> T {
    T::from(self.f.catalan(n))
  }

  /// 分割数
  pub fn partition(&self, n: impl Int, k: impl Int) -> T {
    T::from(self.f.partition(n, k))
  }

  /// 第二種スターリング数
  pub fn stirling2(&self, n: impl Int, k: impl Int) -> T {
    T::from(self.f.stirling2(n, k))
  }

  /// ラグランジュ補間 O(|y| + log mod)
  pub fn interpolation(&self, y: &[T], t: T) -> T {
    lagrange::lagrange_polynomial(self, y, t)
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_enumeration_mod() {
    let f  = EnumerationMod::new(1000000007i64);

    assert_eq!(1, f.fact(0));
    assert_eq!(1, f.fact(1));
    assert_eq!(720, f.fact(6));

    assert_eq!(1, 7 * f.inv(7) % f.modulus());

    assert_eq!(1, f.fact_inv(0));
    assert_eq!(1, f.fact(6) * f.fact_inv(6) % f.modulus());

    assert_eq!(42, f.perm(7, 2));
    
    assert_eq!(21, f.binom(7, 2));

    assert_eq!(4, f.homo(2, 3));

    assert_eq!(1430, f.catalan(8));

    assert_eq!(6, f.stirling2(4, 3));
    assert_eq!(42525, f.stirling2(10, 5));
    assert_eq!(203169470, f.stirling2(100, 30));

    assert_eq!(5, f.bell(3, 5));
    assert_eq!(41, f.bell(5, 3));
    assert_eq!(193120002, f.bell(100, 100));
  }
}