use std::ops::*;
use num_traits::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2<T> {
  pub x: T,
  pub y: T,
}

impl<T: Copy + Num> Point2<T> {
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  pub fn polar(r: T, theta: T) -> Self where T: Float {
    Point2::new(r * theta.cos(), r * theta.sin())
  }

  pub fn cross(self, other: Self) -> T {
    self.x * other.y - self.y * other.x
  }

  /// 偏角
  pub fn arg(self) -> T where T: Float {
    self.y.atan2(self.x)
  }

  /// 偏角で比較
  pub fn cmp_by_arg(&self, other: &Self) -> std::cmp::Ordering where T: PartialOrd {
    (*self).cross(*other).partial_cmp(&T::zero()).unwrap()
  }
}

impl<T: Copy + Num> Point<T> for Point2<T> {
  fn dim(&self) -> usize { 2 }

  fn get(&self, d: usize) -> T {
    match d {
      0 => self.x,
      1 => self.y,
      _ => unreachable!("Point2 is 2-dimensional"),
    }
  }

  fn from_slice(slice: &[T]) -> Self {
    assert_eq!(slice.len(), 2);
    Self::new(slice[0], slice[1])
  }
}

impl<T: Copy + Num> From<(T, T)> for Point2<T> {
  fn from((x, y): (T, T)) -> Self {
    Self { x, y }
  }
}

impl<T: Copy + Num> From<[T; 2]> for Point2<T> {
  fn from([x, y]: [T; 2]) -> Self {
    Self { x, y }
  }
}

impl<T: Copy + Num + Neg<Output = T>> Neg for Point2<T> {
  type Output = Self;
  fn neg(self) -> Self {
    Point2::new(-self.x, -self.y)
  }
}

impl<T: Copy + Num> Add for Point2<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Point2::new(self.x + other.x, self.y + other.y)
  }
}

impl<T: Copy + Num + Zero> Zero for Point2<T> {
  fn zero() -> Self {
    Point2::new(T::zero(), T::zero())
  }

  fn is_zero(&self) -> bool {
    self.x.is_zero() && self.y.is_zero()
  }
}

impl<T: Copy + Num> Sub for Point2<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    Point2::new(self.x - other.x, self.y - other.y)
  }
}

impl<T: Copy + Num> Mul for Point2<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    Point2::new(self.x * other.x, self.y * other.y)
  }
}

impl<T: Copy + Num> Div for Point2<T> {
  type Output = Self;
  fn div(self, other: Self) -> Self {
    Point2::new(self.x / other.x, self.y / other.y)
  }
}

impl<T: Copy + Num> Rem for Point2<T> {
  type Output = Self;
  fn rem(self, other: Self) -> Self {
    Point2::new(self.x % other.x, self.y % other.y)
  }
}

impl<T: Copy + Num + One> One for Point2<T> {
  fn one() -> Self {
    Point2::new(T::one(), T::one())
  }

  fn is_one(&self) -> bool {
    self.x.is_one() && self.y.is_one()
  }
}

impl<T: Copy + Num> Mul<T> for Point2<T> {
  type Output = Self;
  fn mul(self, c: T) -> Self {
    Point2::new(self.x * c.clone(), self.y * c)
  }
}

impl<T: Copy + Num> Div<T> for Point2<T> {
  type Output = Self;
  fn div(self, c: T) -> Self {
    Point2::new(self.x / c.clone(), self.y / c)
  }
}

impl<T: Copy + Num> Num for Point2<T> {
  type FromStrRadixErr = &'static str;
  fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
    let mut sp = str.split_whitespace();
    let x = T::from_str_radix(sp.next().ok_or("failed to get x")?, radix).or(Err("failed to parse x"))?;
    let y = T::from_str_radix(sp.next().ok_or("failed to get y")?, radix).or(Err("failed to parse y"))?;
    Ok(Point2::new(x, y))
  }
}