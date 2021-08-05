use std::ops::{Add, Sub, Mul, Div, Neg};
use num_traits::*;

#[derive(Debug, Clone, Copy)]
pub struct Point2<T>(T, T);
impl<T: Copy + Num> Point2<T> {
  pub fn polar(r: T, theta: T) -> Self where T: Float {
    Point2(r * theta.cos(), r * theta.sin())
  }

  pub fn x(self) -> T { self.0 }
  
  pub fn y(self) -> T { self.1 }

  pub fn dot(self, other: Self) -> T {
    self.0 * other.0 + self.1 * other.1
  }

  pub fn cross(self, other: Self) -> T {
    self.0 * other.1 - self.1 * other.0
  }

  pub fn norm2(self) -> T {
    self.0 * self.0 + self.1 * self.1
  }

  pub fn norm(self) -> T where T: Float {
    (self.0 * self.0 + self.1 * self.1).sqrt()
  }

  pub fn arg(self) -> T where T: Float {
    self.1.atan2(self.0)
  }

  pub fn dist2(self, other: Self) -> T {
    (self - other).norm2()
  }

  pub fn dist(self, other: Self) -> T where T: Float {
    (self - other).norm()
  }

  pub fn normalize(self) -> Self where T: Float {
    self / self.norm()
  }
}

impl<T: Zero> Zero for Point2<T> {
  fn zero() -> Self {
    Point2(T::zero(), T::zero())
  }

  fn is_zero(&self) -> bool {
    self.0.is_zero() && self.1.is_zero()
  }
}

impl<T: Neg<Output = T>> Neg for Point2<T> {
  type Output = Self;
  fn neg(self) -> Self {
    Point2(-self.0, -self.1)
  }
}

impl<T: Add<Output = T>> Add<Point2<T>> for Point2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Point2(self.0 + other.0, self.1 + other.1)
  }
}

impl<T: Sub<Output = T>> Sub<Point2<T>> for Point2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Point2(self.0 - other.0, self.1 - other.1)
  }
}

impl<T: Mul<Output = T> + Clone> Mul<Point2<T>> for Point2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    Point2(self.0 * other.0, self.1 * other.1)
  }
}

impl<T: Mul<Output = T> + Clone> Mul<T> for Point2<T> {
  type Output = Self;
  fn mul(self, c: T) -> Self {
    Point2(self.0 * c.clone(), self.1 * c)
  }
}

impl<T: Div<Output = T> + Clone> Div<T> for Point2<T> {
  type Output = Self;
  fn div(self, c: T) -> Self {
    Point2(self.0 / c.clone(), self.1 / c)
  }
}

