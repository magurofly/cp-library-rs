use std::collections::*;

#[derive(Clone, Debug, Default)]
/// 多重集合
pub struct BTreeMultiset<T: Ord> {
  len: usize,
  set: BTreeMap<T, usize>,
}

impl<T: Ord> BTreeMultiset<T> {
	pub fn new() -> Self {
    Self {
      len: 0,
      set: BTreeMap::new(),
   }
  }

  pub fn len(&self) -> usize {
    self.len
  }

	pub fn contains(&self, item: &T) -> bool {
    self.set.contains_key(item)
  }

  /// 同一要素の個数を取得する
	pub fn count(&self, item: &T) -> usize {
    self.set.get(item).copied().unwrap_or(0)
  }

	pub fn insert(&mut self, item: T) {
    self.len += 1;
    *self.set.entry(item).or_insert(0) += 1;
  }

	pub fn remove(&mut self, item: &T) -> bool {
    if let Some(v) = self.set.get_mut(item) {
      self.len -= 1;
      if *v <= 1 {
        self.set.remove(item);
      } else {
        *v -= 1;
      }
      true
    } else {
      false
    }
  }

  /// 最小値を取得する
	pub fn min(&self) -> Option<&T> {
    self.set.keys().next()
  }

  /// 最大値を取得する
	pub fn max(&self) -> Option<&T> {
    self.set.keys().next_back()
  }

  /// `min` 以上の最小の要素を取得する
	pub fn lower_bound(&self, min: &T) -> Option<&T> {
    self.set.range(min ..).next().map(|e| e.0)
  }

  /// `min` より大きい最小の要素を取得する
	pub fn upper_bound(&self, min: &T) -> Option<&T> {
    use std::ops::Bound::*;
    self.set.range((Excluded(min), Unbounded)).next().map(|e| e.0)
  }

  /// `low` より大きい最小の要素を取得する
  pub fn greater_min(&self, low: &T) -> Option<&T> {
    self.upper_bound(low)
  }

  /// `high` より小さい最大の要素を取得する
  pub fn less_max(&self, high: &T) -> Option<&T> {
    use std::ops::Bound::*;
    self.set.range((Unbounded, Excluded(high))).next_back().map(|e| e.0)
  }

  /// 最小の要素を取り出す
  pub fn pop_min(&mut self) -> Option<T> where T: Clone {
    let min = self.min()?.clone();
    self.remove(&min);
    Some(min)
  }

  /// 最大の要素を取り出す
  pub fn pop_max(&mut self) -> Option<T> where T: Clone {
    let max = self.max()?.clone();
    self.remove(&max);
    Some(max)
  }

  pub fn iter(&self) -> multiset_iter::Iter<'_, T> {
    multiset_iter::Iter::new(self.set.iter())
  }
}

pub mod multiset_iter {
  pub struct Iter<'a, T: 'a + Ord> {
    iter: std::collections::btree_map::Iter<'a, T, usize>,
    current: Option<(&'a T, usize)>,
  }

  impl<'a, T: 'a + Ord> Iter<'a, T> {
    pub fn new(mut iter: std::collections::btree_map::Iter<'a, T, usize>) -> Self {
      let current =
        if let Some((value, count)) = iter.next() {
          Some((value, *count))
        } else {
          None
        };
      Self {
        iter,
        current,
      }
    }
  }

  impl<'a, T: 'a + Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
      if self.current.as_ref().map(|x| x.1 == 0).unwrap_or(false) {
        self.current = None;
        if let Some((value, count)) = self.iter.next() {
          self.current = Some((value, *count));
        }
      }
      self.current.as_mut()?.1 -= 1;
      Some(&self.current.as_ref()?.0)
    }
  }
}

#[derive(Clone, Debug, Default)]
pub struct HashMultiset<E: std::hash::Hash + Eq>(std::collections::HashMap<E, usize>);
impl<E: Eq + std::hash::Hash> HashMultiset<E> {
	pub fn new() -> Self { Self(std::collections::HashMap::new()) }
	pub fn contains(&self, item: &E) -> bool { self.0.contains_key(item) }
	pub fn count(&self, item: &E) -> usize { self.0.get(item).copied().unwrap_or(0) }
	pub fn last(&self) -> Option<&E> { self.0.keys().last() }
	pub fn insert(&mut self, item: E) { *self.0.entry(item).or_insert(0) += 1; }
	pub fn remove(&mut self, item: &E) { if let Some(v) = self.0.get_mut(item) { if *v <= 1 { self.0.remove(item); } else { *v -= 1; } } }
}