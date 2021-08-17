use super::*;
use std::ops::*;
use fft::*;

// -- Addition --

impl<T: Clone + From<u8> + AddAssign, C> AddAssign<&FPS<T, C>> for FPS<T, C> {
  fn add_assign(&mut self, other: &Self) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] += other[i].clone();
    }
  }
}

impl<T: Clone + From<u8> + AddAssign, C: Clone + From<u8>> Add<&FPS<T, C>> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut c = self.clone();
    c += other;
    c
  }
}

impl<T: Clone + From<u8> + AddAssign, C: Clone + From<u8>> Add<&FPS<T, C>> for FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut c = self.clone();
    c += other;
    c
  }
}

// -- Scalar Addition --

impl<T: AddAssign, C> AddAssign<T> for FPS<T, C> {
  fn add_assign(&mut self, other: T) {
    self[0] += other
  }
}

impl<T: Clone + AddAssign, C: Clone> Add<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: T) -> FPS<T, C> {
    let mut f = self.clone();
    f += other;
    f
  }
}

impl<T: Clone + AddAssign, C: Clone> Add<T> for FPS<T, C> {
  type Output = FPS<T, C>;

  fn add(self, other: T) -> FPS<T, C> {
    let mut f = self.clone();
    f += other;
    f
  }
}

// -- Subtraction --

impl<T: Clone + From<u8> + SubAssign, C> SubAssign<&FPS<T, C>> for FPS<T, C> {
  fn sub_assign(&mut self, other: &Self) {
    self.expand(other.len());
    for i in 0 .. other.len() {
      self[i] -= other[i].clone();
    }
  }
}

impl<T: Clone + From<u8> + SubAssign, C: Clone> Sub<&FPS<T, C>> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn sub(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut c = self.clone();
    c -= other;
    c
  }
}

impl<T: Clone + From<u8> + SubAssign, C: Clone> Sub<&FPS<T, C>> for FPS<T, C> {
  type Output = FPS<T, C>;

  fn sub(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut c = self.clone();
    c -= other;
    c
  }
}

// -- Scalar Subtraction --

impl<T: SubAssign, C> SubAssign<T> for FPS<T, C> {
  fn sub_assign(&mut self, other: T) {
    self[0] -= other
  }
}

impl<T: Clone + SubAssign, C: Clone> Sub<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn sub(self, other: T) -> FPS<T, C> {
    let mut f = self.clone();
    f -= other;
    f
  }
}

impl<T: Clone + SubAssign, C: Clone> Sub<T> for FPS<T, C> {
  type Output = FPS<T, C>;

  fn sub(self, other: T) -> FPS<T, C> {
    let mut f = self.clone();
    f -= other;
    f
  }
}

// -- Multiplication --

impl<T: Clone + MulAssign, C> MulAssign<T> for FPS<T, C> {
  fn mul_assign(&mut self, other: T) {
    for i in 0 .. self.len() {
      self[i] *= other.clone();
    }
  }
}

impl<T: Clone + Mul<Output = T>, C: Clone> Mul<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn mul(self, other: T) -> FPS<T, C> {
    FPS::from(self.iter().map(|x| x.clone() * other.clone()).collect::<Vec<_>>())
  }
}

impl<T: Clone, C: Convolution<T> + Clone> Mul<&FPS<T, C>> for &FPS<T, C> where Self: Sized {
  type Output = FPS<T, C>;

  fn mul(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut f = FPS::clone(&self);
    f.convolve(&other);
    f
  }
}

impl<T: Clone, C: Convolution<T> + Clone> Mul<&FPS<T, C>> for FPS<T, C> where Self: Sized {
  type Output = FPS<T, C>;

  fn mul(self, other: &FPS<T, C>) -> FPS<T, C> {
    let mut f = FPS::clone(&self);
    f.convolve(&other);
    f
  }
}

impl<T: Clone, C: Convolution<T>> MulAssign<&FPS<T, C>> for FPS<T, C> {
  fn mul_assign(&mut self, other: &Self) {
    self.convolve(&other);
  }
}

// -- Division --

impl<T: Clone + DivAssign, C> DivAssign<T> for FPS<T, C> {
  fn div_assign(&mut self, other: T) {
    for i in 0 .. self.len() {
      self[i] /= other.clone();
    }
  }
}

impl<T: Clone + Div<Output = T>, C> Div<T> for &FPS<T, C> {
  type Output = FPS<T, C>;

  fn div(self, other: T) -> FPS<T, C> {
    FPS::from(self.iter().map(|x| x.clone() / other.clone()).collect::<Vec<_>>())
  }
}

impl<T: Clone + Div<Output = T>, C> Div<T> for FPS<T, C> {
  type Output = FPS<T, C>;

  fn div(self, other: T) -> FPS<T, C> {
    FPS::from(self.iter().map(|x| x.clone() / other.clone()).collect::<Vec<_>>())
  }
}

impl<T: Clone + PartialEq + From<u8> + AddAssign + SubAssign + Mul<Output = T> + Div<Output = T>, C: Clone + Convolution<T>> DivAssign<&FPS<T, C>> for FPS<T, C> {
  fn div_assign(&mut self, other: &Self) {
    if self.len() < other.len() {
      self.clear();
    } else {
      let n = self.len() - other.len() + 1;
      *self = (&self.rev().pre(n) * &other.rev().inv_at(n)).pre(n).rev_at(n);
    }
  }
}