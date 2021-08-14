use std::cmp::*;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, Ord)]
pub struct IndexedOrd<T> {
  value: T,
  index: Option<usize>,
}
impl<T> IndexedOrd<T> {
  pub fn new(value: T, index: Option<usize>) -> Self {
    Self {
      value,
      index,
    }
  }

  pub fn with_index(value: T, index: usize) -> Self {
    Self {
      value,
      index: Some(index),
    }
  }

  pub fn without_index(value: T) -> Self {
    Self {
      value,
      index: None,
    }
  }

  pub fn value(self) -> T {
    self.value
  }

  pub fn index(&self) -> Option<usize> {
    self.index
  }
}
impl<T: Default> Default for IndexedOrd<T> {
  fn default() -> Self {
    Self {
      value: T::default(),
      index: None,
    }
  }
}
impl<T: PartialEq> PartialEq for IndexedOrd<T> {
  fn eq(&self, other: &Self) -> bool {
    self.value.eq(&other.value)
  }
}
impl<T: PartialOrd> PartialOrd for IndexedOrd<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.value.partial_cmp(&other.value)
  }
}
impl<T> Deref for IndexedOrd<T> {
  type Target = T;
  fn deref(&self) -> &T {
    &self.value
  }
}
impl<T> DerefMut for IndexedOrd<T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut self.value
  }
}
impl<T: Hash> Hash for IndexedOrd<T> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.value.hash(state);
  }
}