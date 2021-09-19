use std::collections::*;
use std::hash::*;

pub trait Set<T> {
  fn has(&self, item: &T) -> bool;
  fn size(&self) -> usize;
}

pub trait SetMut<T>: Set<T> {
  fn add(&mut self, item: T) -> bool;
  fn delete(&mut self, item: &T) -> bool;
}

// BTreeSet

impl<T: Ord> Set<T> for BTreeSet<T> {
  fn has(&self, item: &T) -> bool {
    self.contains(item)
  }

  fn size(&self) -> usize {
    self.len()
  }
}

impl<T: Ord> SetMut<T> for BTreeSet<T> {
  fn add(&mut self, item: T) -> bool {
    self.insert(item)
  }

  fn delete(&mut self, item: &T) -> bool {
    self.remove(item)
  }
}

// HashSet

impl<T: Hash + Eq, S: BuildHasher> Set<T> for HashSet<T, S> {
  fn has(&self, item: &T) -> bool {
    self.contains(item)
  }

  fn size(&self) -> usize {
    self.len()
  }
}

impl<T: Hash + Eq, S: BuildHasher> SetMut<T> for HashSet<T, S> {
  fn add(&mut self, item: T) -> bool {
    self.insert(item)
  }

  fn delete(&mut self, item: &T) -> bool {
    self.remove(item)
  }
}

// Vec

impl Set<usize> for Vec<bool> {
  fn has(&self, &item: &usize) -> bool {
    item < self.len() && self[item]
  }

  /// O(len)
  fn size(&self) -> usize {
    (0 .. self.len()).filter(|&i| self[i]).count()
  }
}

impl SetMut<usize> for Vec<bool> {
  fn add(&mut self, item: usize) -> bool {
    if item >= self.len() {
      self.resize(item + 1, false);
    }
    if self[item] {
      return false;
    }
    self[item] = true;
    true
  }

  fn delete(&mut self, &item: &usize) -> bool {
    if item >= self.len() || !self[item] {
      return false;
    }
    self[item] = false;
    false
  }
}