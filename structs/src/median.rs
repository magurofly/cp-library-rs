use super::*;

#[derive(Clone, Debug, Default)]
/// オンライン中央値
/// https://twitter.com/mts1104_ml/status/1437423359390748677
pub struct Median<T: Ord> {
  // 中央値未満
  left: BTreeMultiset<T>,
  // 中央値以上（常に `right.len() >= left.len()` ）
  right: BTreeMultiset<T>,
}

impl<T: Clone + Ord> Median<T> {
  pub fn new() -> Self {
    Self {
      left: BTreeMultiset::new(),
      right: BTreeMultiset::new(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }

  pub fn len(&self) -> usize {
    self.left.len() + self.right.len()
  }

  /// 小さい方の中央値を取得する
  /// O(log N)
  pub fn median_low(&self) -> Option<&T> {
    if self.left.len() < self.right.len() {
      self.right.min()
    } else {
      self.left.max()
    }
  }

  /// 大きい方の中央値を取得する
  /// O(log N)
  pub fn median_high(&self) -> Option<&T> {
    self.right.min()
  }

  /// 追加する
  /// O(log N)
  pub fn insert(&mut self, value: T) {
    if self.is_empty() {
      self.right.insert(value);
    } else {
      if self.left.len() < self.right.len() {
        if value <= *self.right.min().unwrap() {
          self.left.insert(value);
        } else {
          self.left.insert(self.right.pop_min().unwrap());
          self.right.insert(value);
        }
      } else {
        if value < *self.left.max().unwrap() {
          self.right.insert(self.left.pop_max().unwrap());
          self.left.insert(value);
        } else {
          self.right.insert(value);
        }
      }
    }
  }

  /// 削除する
  /// O(log N)
  pub fn remove(&mut self, value: &T) -> bool {
    if !self.left.remove(value) && !self.right.remove(value) {
      return false;
    }
    if self.left.len() > self.right.len() {
      self.right.insert(self.left.pop_max().unwrap());
    }
    true
  }

  pub fn iter(&self) -> std::iter::Chain<multiset_iter::Iter<'_, T>, multiset_iter::Iter<'_, T>> {
    self.left.iter().chain(self.right.iter())
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn median() {
    let mut m = Median::new();
    let a = vec![1, 5, 6, 5, 3, 6, 7, 8, 3, 8, 1, 3, 5, 2, 5, 8, 3, 4, 1, 5];
    let medians1 = vec![1, 5, 5, 5, 5, 5, 5, 6, 5, 6, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5];
    let mut medians2 = vec![];

    for &x in &a {
      m.insert(x);
      medians2.push(*m.median_high().unwrap());
    }

    assert_eq!(medians1, medians2);
  }
}