use std::{marker::PhantomData, ops::*};
use fft::*;
use acl_modint::*;

pub type FPSStatic<M> = FPS<StaticModInt<M>, ConvolutionStatic<M>>;
pub type FPS998244353 = FPSStatic<Mod998244353>;
pub type FPSDynamic<I> = FPS<DynamicModInt<I>, ConvolutionDynamic<I>>;

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

  pub fn convolve(&mut self, other: &Self) where C: Convolution<T> {
    self.a = C::convolution(&self.a, &other.a);
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

  pub fn rev(&self) -> Self where Vec<T>: Clone {
    self.rev_at(self.len())
  }

  pub fn rev_at(&self, deg: usize) -> Self where Vec<T>: Clone {
    let mut a = self.a.clone();
    a.truncate(deg);
    a.reverse();
    Self::from(a)
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

#[cfg(test)]
mod tests {
  #[test]
  fn test_inv() {
    use super::*;

    fn inv(a: Vec<u32>) -> Vec<u32> {
      let deg = a.len();
      let f = FPS998244353::from(cast_vec(a));
      let mut g = f.inv();
      g.resize(deg);
      let b: Vec<_> = g.into();
      flat_vec(b)
    }

    assert_eq!(inv(vec![5, 4, 3, 2, 1]), vec![598946612, 718735934, 862483121, 635682004, 163871793]);
  }

  fn cast_vec<T, U: From<T>>(a: Vec<T>) -> Vec<U> {
    a.into_iter().map(U::from).collect::<Vec<_>>()
  }

  fn flat_vec<M: acl_modint::ModIntBase>(a: Vec<M>) -> Vec<u32> {
    a.into_iter().map(M::val).collect::<Vec<_>>()
  }
}