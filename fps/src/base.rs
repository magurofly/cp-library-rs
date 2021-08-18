use std::{cmp::*, marker::PhantomData, ops::*};
use fft::*;
use acl_modint::*;

pub type FPSStatic<M> = FPS<StaticModInt<M>, ConvolutionStatic<M>>;
pub type FPS998244353 = FPSStatic<Mod998244353>;
pub type FPSDynamic<I> = FPS<DynamicModInt<I>, ConvolutionDynamic<I>>;

#[derive(Clone)]
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

  pub fn from_slice<U: Clone + Into<T>>(slice: &[U]) -> Self {
    Self::from(slice.into_iter().map(|x| x.clone().into()).collect::<Vec<T>>())
  }

  pub fn convolve(&mut self, other: &FPS<T, C>) where C: Convolution<T> {
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

impl<T: Clone, C: Clone> Shr<usize> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn shr(self, n: usize) -> FPS<T, C> {
    let mut f = self.clone();
    f >>= n;
    f
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

impl<T: Clone + From<u8>, C: Clone> Shl<usize> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn shl(self, n: usize) -> FPS<T, C> {
    let mut f = self.clone();
    f <<= n;
    f
  }
}

impl<T: std::fmt::Debug, C> std::fmt::Debug for FPS<T, C> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("FPS[")?;
    let terms = self.iter().enumerate().map(|(n, x)| format!("{:?} x{}", x, n)).collect::<Vec<_>>();
    f.write_str(&terms.join(" + "))?;
    f.write_str("]")?;
    Ok(())
  }
}

impl<T: PartialEq + From<u8>, C> PartialEq for FPS<T, C> {
  fn eq(&self, other: &Self) -> bool {
    let n = self.deg().min(other.deg());
    (0 .. n).all(|i| self[i] == other[i])
    && (n .. self.deg()).all(|i| self[i] == T::from(0))
    && (n .. other.deg()).all(|i| other[i] == T::from(0)) 
  }
}

#[cfg(test)]
mod tests {
}