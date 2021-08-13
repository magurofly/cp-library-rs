use std::{marker::PhantomData, ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign}};

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

  pub fn with_deg(deg: usize) -> Self where T: Default {
    let mut a = Vec::with_capacity(deg + 1);
    a.resize_with(deg + 1, T::default);
    Self::from(a)
  }

  fn resize(&mut self, n: usize) where T: Default {
    self.a.resize_with(n, T::default);
  }

  fn expand(&mut self, n: usize) where T: Default {
    if self.a.len() < n {
      self.resize(n);
    }
  }

  fn shrink(&mut self) where T: PartialEq + Default {
    while let Some(x) = self.a.last() {
      if x != &T::default() {
        break;
      }
      self.a.pop();
    }
  }

  fn deg(&self) -> usize {
    self.a.len().saturating_sub(1)
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

impl<T: Clone + Default + AddAssign, C> AddAssign<&FPS<T, C>> for FPS<T, C> {
  fn add_assign(&mut self, other: &Self) {
    self.expand(other.a.len());
    for i in 0 .. other.a.len() {
      self[i] += other[i].clone();
    }
  }
}
impl<'a, T: Clone + Default + AddAssign, C: Clone + Default> Add<&'a FPS<T, C>> for &'a FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: Self) -> FPS<T, C> {
    let mut c = self.clone();
    c += other;
    c
  }
}

impl<T: Clone + Default + SubAssign, C> SubAssign<&FPS<T, C>> for FPS<T, C> {
  fn sub_assign(&mut self, other: &Self) {
    self.expand(other.a.len());
    for i in 0 .. other.a.len() {
      self[i] -= other[i].clone();
    }
  }
}
impl<'a, T: Clone + Default + SubAssign, C: Clone + Default> Sub<&'a FPS<T, C>> for &'a FPS<T, C> {
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
impl<T: Clone + Default, C: Convolution<T>> MulAssign<&FPS<T, C>> for FPS<T, C> {
  fn mul_assign(&mut self, other: &Self) {
    self.a = C::convolution(&self.a, &other.a);
  }
}

pub trait Convolution<T> {
  fn convolution(a: &[T], b: &[T]) -> Vec<T>;
}

#[derive(Debug, Clone, Default)]
pub struct StaticModIntConvolution<M>(PhantomData<M>);
impl<M: Modulus> Convolution<StaticModInt<M>> for StaticModIntConvolution<M> {
  fn convolution(a: &[StaticModInt<M>], b: &[StaticModInt<M>]) -> Vec<StaticModInt<M>> {
    convolution(a, b)
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