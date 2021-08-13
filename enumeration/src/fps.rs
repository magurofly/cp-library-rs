use std::{marker::PhantomData, ops::*};

use acl_modint::*;
use acl_convolution::*;

pub type FPSStaticModInt<M> = FPS<StaticModInt<M>, StaticModIntConvolution<M>>;
pub type FPS998244353 = FPSStaticModInt<Mod998244353>;
pub type FPSDynamicModInt<I> = FPS<DynamicModInt<I>, DynamicModIntConvolution<I>>;

#[derive(Debug, Clone)]
pub struct FPS<T, C> {
  convolution: PhantomData<C>,
  a: Vec<T>,
}
impl<T, C> FPS<T, C> {
  pub fn new() -> Self {
    Self {
      convolution: PhantomData,
      a: vec![],
    }
  }

  pub fn with_deg(deg: usize) -> Self where T: From<u8> {
    let mut a = Vec::with_capacity(deg);
    a.resize_with(deg, || T::from(0u8));
    Self::from(a)
  }

  pub fn resize(&mut self, n: usize) where T: From<u8> {
    self.a.resize_with(n, || T::from(0u8));
  }

  pub fn expand(&mut self, n: usize) where T: From<u8> {
    if self.a.len() < n {
      self.resize(n);
    }
  }

  pub fn shrink(&mut self) where T: PartialEq + From<u8> {
    while let Some(x) = self.a.last() {
      if x != &T::from(0u8) {
        break;
      }
      self.a.pop();
    }
  }

  pub fn deg(&self) -> usize {
    self.len()
  }

  pub fn pre(&self, deg: usize) -> Self where T: Clone {
    Self::from(self.a[0 .. self.len().min(deg)].to_vec())
  }

  /// `self[0]` must not be zero
  pub fn inv_deg(&self, deg: usize) -> FPS<T, C> where T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T> {
    assert!(self.len() > 0 && self[0] != T::from(0u8));
    let mut r = Self::from(vec![T::from(1) / self[0].clone()]);
    let mut i = 1;
    while i < deg {
      let mut f = r.clone();
      f += &r;
      let mut g = r.clone();
      g *= &r;
      g *= &self.pre(i << 1);
      f -= &g;
      r = f.pre(i << 1);
      i <<= 1;
    }
    r
  }

  /// `self[0]` mut not be zero
  pub fn inv(&self) -> FPS<T, C> where T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T> {
    self.inv_deg(self.deg())
  }
}
impl<T, C> Deref for FPS<T, C> {
  type Target = Vec<T>;
  fn deref(&self) -> &Vec<T> {
    &self.a
  }
}
impl<T, C> DerefMut for FPS<T, C> {
  fn deref_mut(&mut self) -> &mut Vec<T> {
    &mut self.a
  }
}
impl<T, C> From<Vec<T>> for FPS<T, C> {
  fn from(a: Vec<T>) -> Self {
    Self { convolution: PhantomData, a }
  }
}
impl<T, C> Into<Vec<T>> for FPS<T, C> {
  fn into(self) -> Vec<T> {
    self.a
  }
}
impl<T, C> Index<usize> for FPS<T, C> {
  type Output = T;

  fn index(&self, idx: usize) -> &T {
    &self.a[idx]
  }
}
impl<T, C> IndexMut<usize> for FPS<T, C> {
  fn index_mut(&mut self, idx: usize) -> &mut T {
    &mut self.a[idx]
  }
}

impl<T: Clone + From<u8> + AddAssign, C> AddAssign<&FPS<T, C>> for FPS<T, C> {
  fn add_assign(&mut self, other: &Self) {
    self.expand(other.a.len());
    for i in 0 .. other.a.len() {
      self[i] += other[i].clone();
    }
  }
}
impl<'a, T: Clone + From<u8> + AddAssign, C: Clone + From<u8>> Add<&'a FPS<T, C>> for &'a FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: Self) -> FPS<T, C> {
    let mut c = self.clone();
    c += other;
    c
  }
}

impl<T: Clone + From<u8> + SubAssign, C> SubAssign<&FPS<T, C>> for FPS<T, C> {
  fn sub_assign(&mut self, other: &Self) {
    self.expand(other.a.len());
    for i in 0 .. other.a.len() {
      self[i] -= other[i].clone();
    }
  }
}
impl<'a, T: Clone + From<u8> + SubAssign, C: Clone + From<u8>> Sub<&'a FPS<T, C>> for &'a FPS<T, C> {
  type Output = FPS<T, C>;

  fn sub(self, other: Self) -> FPS<T, C> {
    let mut c = self.clone();
    c -= other;
    c
  }
}

impl<T: Clone + MulAssign, C> MulAssign<T> for FPS<T, C> {
  fn mul_assign(&mut self, other: T) {
    for i in 0 .. self.a.len() {
      self[i] *= other.clone();
    }
  }
}
impl<T: Clone + Mul<Output = T>, C> Mul<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn mul(self, other: T) -> FPS<T, C> {
    FPS::from(self.a.iter().map(|x| x.clone() * other.clone()).collect::<Vec<_>>())
  }
}

impl<'a, T, C: Convolution<T>> Mul<&'a FPS<T, C>> for &'a FPS<T, C> {
  type Output = FPS<T, C>;

  fn mul(self, other: Self) -> FPS<T, C> {
    FPS::from(C::convolution(&self.a, &other.a))
  }
}
impl<T: Clone, C: Convolution<T>> MulAssign<&FPS<T, C>> for FPS<T, C> {
  fn mul_assign(&mut self, other: &Self) {
    self.a = C::convolution(&self.a, &other.a);
  }
}

impl<T: Clone + DivAssign, C> DivAssign<T> for FPS<T, C> {
  fn div_assign(&mut self, other: T) {
    for i in 0 .. self.a.len() {
      self[i] /= other.clone();
    }
  }
}
impl<T: Clone + Div<Output = T>, C> Div<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn div(self, other: T) -> FPS<T, C> {
    FPS::from(self.a.iter().map(|x| x.clone() / other.clone()).collect::<Vec<_>>())
  }
}

impl<T, C> ShrAssign<usize> for FPS<T, C> {
  fn shr_assign(&mut self, n: usize) {
    self.a = self.a.split_off(n);
  }
}

impl<T: From<u8>, C> ShlAssign<usize> for FPS<T, C> {
  fn shl_assign(&mut self, n: usize) {
    let mut a = Vec::with_capacity(n + self.a.len());
    a.resize_with(n, || T::from(0u8));
    a.append(&mut self.a);
    self.a = a;
  }
}

pub trait Convolution<T> {
  fn convolution(a: &[T], b: &[T]) -> Vec<T>;
}

#[derive(Debug, Clone, Default)]
pub struct StaticModIntConvolution<M>(PhantomData<M>);
impl<M: Modulus> Convolution<StaticModInt<M>> for StaticModIntConvolution<M> {
  fn convolution(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>> {
    if M::HINT_VALUE_IS_PRIME && (M::VALUE - 1).trailing_zeros() >= 20 {
      convolution(a, b)
    } else {
      let a = a.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
      let b = b.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
      convolution_i64(&a, &b).into_iter().map(|x| (x % <StaticModInt<M>>::modulus() as i64).into()).collect::<Vec<_>>()
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct DynamicModIntConvolution<I>(PhantomData<I>);
impl<I: Id> Convolution<DynamicModInt<I>> for DynamicModIntConvolution<I> {
  fn convolution(a: &[DynamicModInt<I>], b: &[DynamicModInt<I>]) -> Vec<DynamicModInt<I>> {
    let a = a.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
    let b = b.iter().map(|x| x.val() as i64).collect::<Vec<_>>();
    let c = convolution_i64(&a, &b).into_iter().map(|x| (x % <DynamicModInt<I>>::modulus() as i64).into()).collect::<Vec<_>>();
    c
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_inv() {
    use super::*;
    // use acl_modint::*;

    fn inv(a: Vec<u32>) -> Vec<u32> {
      let deg = a.len();
      let f = FPS998244353::from(a.into_iter().map(ModInt998244353::from).collect::<Vec<_>>());
      let mut g = f.inv();
      g.resize(deg);
      let b: Vec<_> = g.into();
      b.into_iter().map(|x| x.val()).collect::<Vec<_>>()
    }

    assert_eq!(inv(vec![5, 4, 3, 2, 1]), vec![598946612, 718735934, 862483121, 635682004, 163871793]);
  }
}