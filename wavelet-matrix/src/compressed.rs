use value_compression::ValueCompression;

use crate::WaveletMatrixUsize;

pub struct WaveletMatrix<T> {
  compress: ValueCompression<T>,
  matrix: WaveletMatrixUsize,
}

impl<T: Clone + Ord> WaveletMatrix<T> {
  /// Wavelet Matrix を作成する O(N log N)
  pub fn new(array: &[T]) -> Self {
    let mut compress = ValueCompression::new();
    let a = compress.convert(array);
    let sigma = compress.len();
    Self {
      compress,
      matrix: WaveletMatrixUsize::new(a, sigma),
    }
  }

  /// idx 番目の要素を返す
  /// O(log sigma + log N)
  pub fn get(&self, idx: usize) -> T {
    self.access(idx)
  }

  /// idx 番目の要素を返す
  /// O(log sigma + log N)
  pub fn access(&self, idx: usize) -> T {
    self.compress.get(self.matrix.access(idx))
  }

  /// [0, r) にある x の数を返す
  /// O(log sigma + log N)
  pub fn rank(&self, x: &T, r: usize) -> usize {
    self.matrix.rank(self.compress.rank(x), r)
  }

  /// num 番目の x の位置を返す
  /// O(log N log sigma)
  pub fn select(&self, x: &T, num: usize) -> Option<usize> {
    self.matrix.select(self.compress.rank(x), num)
  }

  /// [l, r) における k 番目に小さい要素を返す
  /// O(log sigma + log N)
  pub fn kth_smallest(&self, l: usize, r: usize, k: usize) -> T {
    self.compress.get(self.matrix.kth_smallest(l, r, k))
  }

  /// [l, r) における k 番目に大きい要素を返す
  /// O(log sigma + log N)
  pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> T {
    self.compress.get(self.matrix.kth_largest(l, r, k))
  }

  /// [l, r) における x 未満の要素の個数を返す
  /// O(log sigma + log N)
  pub fn range_freq(&self, l: usize, r: usize, x: &T) -> usize {
    self.matrix.range_freq(l, r, self.compress.rank(x))
  }

  /// [l, r) における low 以上 high 未満の要素の個数を返す
  /// O(log sigma + log N)
  pub fn range_freq_between(&self, l: usize, r: usize, low: &T, high: &T) -> usize {
    self.range_freq(l, r, high) - self.range_freq(l, r, low)
  }

  /// [l, r) における high 未満の最大値を返す
  /// O(log sigma + log N)
  pub fn prev_value(&self, l: usize, r: usize, high: &T) -> Option<T> {
    Some(self.get(self.matrix.prev_value(l, r, self.compress.rank(high))?))
  }

  /// [l, r) における low 以上の最小値を返す
  /// O(log sigma + log N)
  pub fn next_value(&self, l: usize, r: usize, low: &T) -> Option<T> {
    Some(self.get(self.matrix.next_value(l, r, self.compress.rank(low))?))
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn access() {
    let a = vec![3, 1, 4, 1, 5, 9, 2];
    let wm = WaveletMatrix::new(&a);
    let b = (0 .. a.len()).map(|i| wm.access(i)).collect::<Vec<_>>();
    assert_eq!(a, b);
  }
}