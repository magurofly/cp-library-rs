use std::collections::*;
use std::cell::*;

/// 座圧
/// 値に昇順に番号をつける
pub struct ValueCompression<T> {
  coords: BTreeSet<T>,
  cache: RefCell<Option<Vec<T>>>,
}

impl<T: Clone + Ord> ValueCompression<T> {
  pub fn len(&self) -> usize { self.coords.len() }

  pub fn new() -> Self {
    Self { coords: BTreeSet::new(), cache: RefCell::new(None) }
  }

  /// 追加 O(log N)
  pub fn add(&mut self, x: T) {
    self.coords.insert(x);
    *self.cache.borrow_mut() = None;
  }

  /// 二分探索する O(log N)
  /// 追加の後に呼び出した時は O(N log N) になる
  pub fn search(&self, x: &T) -> Result<usize, usize> {
    self.cache();
    self.cache.borrow().as_ref().unwrap().binary_search(x)
  }

  /// 番号を取得する O(log N)
  /// 追加の後に呼び出した時は O(N log N) になる
  pub fn rank(&self, x: &T) -> usize {
    let r = self.search(x);
    r.ok().or(r.err()).unwrap()
  }

  /// 値を取得する O(1)
  pub fn get(&self, idx: usize) -> T {
    self.cache();
    self.cache.borrow().as_ref().unwrap()[idx].clone()
  }

  /// 配列の値を追加し、位置に変換する O(N log N)
  pub fn convert(&mut self, a: &[T]) -> Vec<usize> {
    for x in a {
      self.coords.insert(x.clone());
    }
    *self.cache.borrow_mut() = None;
    let b = a.iter().map(|x| self.rank(x)).collect::<Vec<_>>();
    b
  }

  fn cache(&self) {
    if self.cache.borrow().is_none() {
      *self.cache.borrow_mut() = Some(self.coords.iter().cloned().collect::<Vec<_>>());
    }
  }
}