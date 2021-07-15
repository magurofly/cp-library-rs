use matrix::Matrix;
pub mod matrix {
  #[derive(Debug, Clone)]
  pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    mat: Vec<Vec<T>>,
  }
  impl<T> Matrix<T> {
    pub fn new(mat: Vec<Vec<T>>) -> Self { check_size(&mat); Self { rows: mat.len(), cols: mat[0].len(), mat } }
    pub fn at(&self, i: usize, j: usize) -> T where T: Clone { self.mat[i][j].clone() }
  }
  impl<T> ops::Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T { &self.mat[index.0][index.1] }
  }
  impl<T> ops::IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T { &mut self.mat[index.0][index.1] }
  }
  impl<T: ops::Neg<Output = T>> ops::Neg for Matrix<T> {
    type Output = Self;
    fn neg(self) -> Self {
      Self::new(self.mat.into_iter().map(|r| r.into_iter().map(T::neg).collect() ).collect())
    }
  }
  impl<T: ops::AddAssign> ops::AddAssign for Matrix<T> {
    fn add_assign(&mut self, other: Matrix<T>) {
      check_dimension(self, &other);
      for (row1, row2) in self.mat.iter_mut().zip(other.mat) {
        for (x, y) in row1.iter_mut().zip(row2) {
          *x += y;
  } } } }
  impl<T: ops::AddAssign> ops::Add for Matrix<T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self { self += other; self }
  }
  impl<T: ops::SubAssign> ops::SubAssign for Matrix<T> {
    fn sub_assign(&mut self, other: Matrix<T>) {
      check_dimension(self, &other);
      for (row1, row2) in self.mat.iter_mut().zip(other.mat) {
        for (x, y) in row1.iter_mut().zip(row2) {
          *x -= y;
  } } } }
  impl<T: ops::SubAssign> ops::Sub for Matrix<T> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self { self -= other; self }
  }
  impl<T: Clone + ops::MulAssign> ops::MulAssign<T> for Matrix<T> {
    fn mul_assign(&mut self, c: T) {
      for row in self.mat.iter_mut() {
        for x in row.iter_mut() {
          *x *= c.clone();
  } } } }
  impl<T: Clone + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul for &Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Self) -> Matrix<T> {
      Matrix::new((0 .. self.rows).map(|i|
        (0 .. other.cols).map(|j|
          (0 .. self.cols).map(|k| self.at(i, k) * self.at(k, j)).reduce(|x, y| x + y).unwrap()
        ).collect()
      ).collect())
    }
  }
  impl<T: Clone + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul for Matrix<T> {
    type Output = Matrix<T>;
    fn mul(self, other: Self) -> Matrix<T> { &self * &other }
  }

  fn check_size<T>(mat: &Vec<Vec<T>>) {
    let r = mat.len();
    assert!(r > 0, "number of rows cannot be zero");
    let c = mat[0].len();
    assert!(c > 0, "number of columns cannot be zero");
    assert!((0 .. r).all(|i| mat[i].len() == c ), "different number of columns");
  }

  fn check_dimension<T>(a: &Matrix<T>, b: &Matrix<T>) {
    assert!(a.rows == b.rows && a.cols == b.cols, "dimension mismatch: ({}, {}) != ({}, {})", a.rows, a.cols, b.rows, b.cols);
  }

  use std::ops;
}
