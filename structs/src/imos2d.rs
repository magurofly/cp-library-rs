use ranges::*;
use std::ops::RangeBounds;

// Verified: https://atcoder.jp/contests/abc005/submissions/22129384
pub struct Imos2D<T, A, S> {
  n: usize, m: usize,
  imos: Vec<Vec<T>>,
  add: A, sub: S
}
impl<T: Copy, A: Fn(T, T) -> T, S: Fn(T, T) -> T> Imos2D<T, A, S> {
  pub fn new(mat: &Vec<Vec<T>>, id: T, add: A, sub: S) -> Self {
    let n = mat.len();
    let m = mat[0].len();
    let mut imos = vec![vec![id; m + 1]; n + 1];
    for i in 0 .. n {
      for j in 0 .. m {
        imos[i + 1][j + 1] = (sub)((add)((add)(imos[i + 1][j], imos[i][j + 1]), mat[i][j]), imos[i][j]);
      }
    }
    Self { n, m, imos, add, sub }
  }
  
  pub fn at(&self, i: usize, j: usize) -> T {
    self.rect(i ..= i, j ..= j)
  }
  
  pub fn rect(&self, i: impl RangeBounds<usize>, j: impl RangeBounds<usize>) -> T {
    let i1 = i.start_close_or(0);
    let i2 = i.end_open_or(self.n);
    let j1 = j.start_close_or(0);
    let j2 = j.end_open_or(self.m);
    (self.sub)((self.add)(self.imos[i2][j2], self.imos[i1][j1]), (self.add)(self.imos[i1][j2], self.imos[i2][j1]))
  }
}