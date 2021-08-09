use super::*;
use std::collections::*;
use std::cell::*;

pub struct CompressCoords<T> {
  coords: BTreeSet<T>,
  cache: RefCell<Option<Vec<T>>>,
}
impl<T: Clone + Ord> CompressCoords<T> {
  pub fn new() -> Self {
    Self { coords: BTreeSet::new(), cache: RefCell::new(None) }
  }

  pub fn add(&mut self, x: T) {
    self.coords.insert(x);
    *self.cache.borrow_mut() = None;
  }

  pub fn index_of(&self, x: &T) -> Result<usize, usize> {
    self.cache.borrow_mut()
    .get_or_insert_with(|| self.coords.iter().cloned().to_vec())
    .binary_search(x)
  }

  pub fn get(&self, x: &T) -> usize {
    let r = self.index_of(x);
    r.ok().or(r.err()).unwrap()
  }
}