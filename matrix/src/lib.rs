use num_traits::*;
use std::ops::{Index, IndexMut, Add, Sub, Mul, AddAssign, SubAssign, MulAssign};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
  rows: usize,
  cols: usize,
  mat: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
  pub fn rows(&self) -> usize {
    self.rows
  }

  pub fn cols(&self) -> usize {
    self.cols
  }

  pub fn new(rows: usize, cols: usize, init: T) -> Self where T: Clone {
    Self { rows, cols, mat: vec![vec![init; cols]; rows] }
  }

  pub fn id(n: usize) -> Self where T: Zero + One + Clone {
    let mut mat = vec![vec![T::zero(); n]; n];
    for i in 0 .. n {
      mat[i][i] = T::one();
    }
    Self { rows: n, cols: n, mat }
  }
}

impl<T> IntoIterator for Matrix<T> {
  type Item = Vec<T>;
  type IntoIter = <Vec<Vec<T>> as IntoIterator>::IntoIter;
  fn into_iter(self) -> Self::IntoIter {
    self.mat.into_iter()
  }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T> {
  fn from(mat: Vec<Vec<T>>) -> Self {
    let rows = mat.len();
    if rows == 0 {
      return Self { rows, cols: 0, mat }
    }
    let cols = mat[0].len();
    for i in 1 .. rows {
      assert!(mat[i].len() == cols);
    }
    Self { rows, cols, mat }
  }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
  type Output = T;
  fn index(&self, (r, c): (usize, usize)) -> &T {
    &self.mat[r][c]
  }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
  fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut T {
    &mut self.mat[r][c]
  }
}

impl<T> Index<usize> for Matrix<T> {
  type Output = [T];
  fn index(&self, idx: usize) -> &[T] {
    &self.mat[idx]
  }
}

impl<T> IndexMut<usize> for Matrix<T> {
  fn index_mut(&mut self, idx: usize) -> &mut [T] {
    &mut self.mat[idx]
  }
}

impl<T: AddAssign<T>> AddAssign<Matrix<T>> for Matrix<T> {
  fn add_assign(&mut self, other: Self) {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    for (i, row) in other.into_iter().enumerate() {
      for (j, x) in row.into_iter().enumerate() {
        self[i][j] += x;
      }
    }
  }
}

impl<T: AddAssign<T> + Clone> AddAssign<&Matrix<T>> for Matrix<T> {
  fn add_assign(&mut self, other: &Self) {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    for i in 0 .. self.rows() {
      for j in 0 .. self.cols() {
        self[i][j] += other[i][j].clone();
      }
    }
  }
}

impl<T: Add<Output = T>> Add for Matrix<T> {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    let rows = self.rows();
    let cols = self.cols();
    let mat = self.into_iter().zip(other)
      .map(|(row1, row2)|
        row1.into_iter().zip(row2).map(|(x1, x2)| x1 + x2).collect()
      ).collect();
    Self { rows, cols, mat }
  }
}

impl<T: SubAssign<T>> SubAssign<Matrix<T>> for Matrix<T> {
  fn sub_assign(&mut self, other: Self) {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    for (i, row) in other.into_iter().enumerate() {
      for (j, x) in row.into_iter().enumerate() {
        self[i][j] -= x;
      }
    }
  }
}

impl<T: SubAssign<T> + Clone> SubAssign<&Matrix<T>> for Matrix<T> {
  fn sub_assign(&mut self, other: &Self) {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    for i in 0 .. self.rows() {
      for j in 0 .. self.cols() {
        self[i][j] -= other[i][j].clone();
      }
    }
  }
}

impl<T: Sub<Output = T>> Sub for Matrix<T> {
  type Output = Self;
  fn sub(self, other: Self) -> Self {
    assert!(self.rows() == other.rows() && self.cols() == other.cols());
    let rows = self.rows();
    let cols = self.cols();
    let mat = self.into_iter().zip(other)
      .map(|(row1, row2)|
        row1.into_iter().zip(row2).map(|(x1, x2)| x1 - x2).collect()
      ).collect();
    Self { rows, cols, mat }
  }
}

impl<T: MulAssign<T> + Clone> MulAssign<T> for Matrix<T> {
  fn mul_assign(&mut self, c: T) {
    for i in 0 .. self.rows() {
      for j in 0 .. self.cols() {
        self[i][j] *= c.clone();
      }
    }
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default> Mul for &Matrix<T> {
  type Output = Matrix<T>;
  fn mul(self, other: Self) -> Matrix<T> {
    assert!(self.cols() == other.rows());
    let rows = self.rows();
    let cols = self.cols();
    let mut mat = vec![vec![T::default(); cols]; rows];
    for k in 0 .. cols {
      for i in 0 .. rows {
        for j in 0 .. other.cols() {
          mat[i][j] = self[i][k].clone() * other[k][j].clone();
        }
      }
    }
    Matrix { rows, cols, mat }
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default> Mul for Matrix<T> {
  type Output = Self;
  fn mul(self, other: Self) -> Self {
    (&self).mul(&other)
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default> MulAssign<&Matrix<T>> for Matrix<T> {
  fn mul_assign(&mut self, other: &Self) {
    *self = &*self * other;
  }
}

impl<T: Add<Output = T> + Mul<Output = T> + Clone + Default> MulAssign<Matrix<T>> for Matrix<T> {
  fn mul_assign(&mut self, other: Self) {
    *self = &*self * &other;
  }
}

impl<E: PrimInt, T: Add<Output = T> + Mul<Output = T> + Clone + Default + Zero + One> Pow<E> for &Matrix<T> {
  type Output = Matrix<T>;
  fn pow(self, mut e: E) -> Matrix<T> {
    assert!(self.rows() == self.cols());
    let mut r = Matrix::id(self.rows());
    let mut a = self.clone();
    while !e.is_zero() {
      if (e & E::one()).is_one() {
        r *= &a;
      }
      a = &a * &a;
      e = e >> 1;
    }
    r
  }
}