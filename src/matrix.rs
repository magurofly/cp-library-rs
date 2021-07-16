pub mod matrix {
  // Last Update: 2021-07-16 09:58
  #[derive(Debug, Clone)]
  pub struct Matrix<'a, T: Clone> {
    rows: usize,
    cols: usize,
    mat: Cow<'a, Vec<Vec<T>>>,
  }
  impl<'a, T: Clone> Matrix<'a, T> {
    pub fn new(mat: Vec<Vec<T>>) -> Self { check_size(&mat); Self { rows: mat.len(), cols: mat[0].len(), mat: Cow::Owned(mat) } }
    pub fn at(&self, i: usize, j: usize) -> T where T: Clone { self.mat[i][j].clone() }
    pub fn map<R: Clone>(self, mut f: impl FnMut(T) -> R) -> Matrix<'a, R> { Matrix::new(self.mat.iter().map(|r| r.iter().map(|x| (f)(x.clone()) ).collect() ).collect()) }
    pub fn tap(&mut self, mut f: impl FnMut(&mut T)) { for r in self.mat.to_mut().iter_mut() { for x in r { (f)(x); } } }
    pub fn map2<U: Clone, R: Clone>(self, other: Matrix<U>, mut f: impl FnMut(T, U) -> R) -> Matrix<R> where T: Clone { check_dimension(&self, &other); Matrix::new(self.mat.iter().zip(other.mat.iter()).map(|(r1, r2)| r1.iter().zip(r2).map(|(x, y)| (f)(x.clone(), y.clone()) ).collect() ).collect()) }
    pub fn tap2<U: Clone>(&mut self, other: Matrix<U>, mut f: impl FnMut(&mut T, U)) { check_dimension(self, &other); for (r1, r2) in self.mat.to_mut().iter_mut().zip(other.mat.iter()) { for (x, y) in r1.iter_mut().zip(r2) { (f)(x, y.clone()) } } }
  }
  impl<'a, T: Clone> ops::Index<(usize, usize)> for Matrix<'a, T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T { &self.mat[index.0][index.1] }
  }
  impl<'a, T: Clone> ops::IndexMut<(usize, usize)> for Matrix<'a, T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T { &mut self.mat.to_mut()[index.0][index.1] }
  }
  impl<'a, T: Clone + ops::Neg<Output = T>> ops::Neg for Matrix<'a, T> {
    type Output = Self;
    fn neg(self) -> Self { self.map(T::neg) }
  }
  impl<'a, T: Clone + ops::AddAssign> ops::AddAssign for Matrix<'a, T> {
    fn add_assign(&mut self, other: Matrix<T>) { self.tap2(other, |x, y| { *x += y; }); }
  }
  impl<'a, T: Clone + ops::AddAssign> ops::Add for Matrix<'a, T> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self { self += other; self }
  }
  impl<'a, T: Clone + ops::SubAssign> ops::SubAssign for Matrix<'a, T> {
    fn sub_assign(&mut self, other: Matrix<T>) { self.tap2(other, |x, y| { *x -= y; }); }
  }
  impl<'a, T: Clone + ops::SubAssign> ops::Sub for Matrix<'a, T> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self { self -= other; self }
  }
  impl<'a, T: Clone + ops::MulAssign> ops::MulAssign<T> for Matrix<'a, T> {
    fn mul_assign(&mut self, c: T) { self.tap(|x| { *x *= c.clone(); }); }
  }
  impl<'a, T: Clone + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul for &Matrix<'a, T> {
    type Output = Matrix<'a, T>;
    fn mul(self, other: Self) -> Matrix<'a, T> {
      Matrix::new((0 .. self.rows).map(|i|
        (0 .. other.cols).map(|j|
          (0 .. self.cols).map(|k| self.at(i, k) * self.at(k, j)).reduce(|x, y| x + y).unwrap()
        ).collect()
      ).collect())
    }
  }
  impl<'a, T: Clone + ops::Add<Output = T> + ops::Mul<Output = T>> ops::Mul for Matrix<'a, T> {
    type Output = Matrix<'a, T>;
    fn mul(self, other: Self) -> Matrix<'a, T> { &self * &other }
  }

  fn check_size<T>(mat: &Vec<Vec<T>>) {
    let r = mat.len();
    assert!(r > 0, "number of rows cannot be zero");
    let c = mat[0].len();
    assert!(c > 0, "number of columns cannot be zero");
    assert!((0 .. r).all(|i| mat[i].len() == c ), "different number of columns");
  }

  fn check_dimension<T: Clone, U: Clone>(a: &Matrix<T>, b: &Matrix<U>) {
    assert!(a.rows == b.rows && a.cols == b.cols, "dimension mismatch: ({}, {}) != ({}, {})", a.rows, a.cols, b.rows, b.cols);
  }

  use std::ops;
  use std::borrow::*;
}
