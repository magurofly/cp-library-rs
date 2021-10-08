use std::collections::HashMap;

pub struct Cycle {
  path: Vec<usize>,
  first: HashMap<usize, usize>,
  tail: usize,
}

impl Cycle {
  pub fn new(path: Vec<usize>, first: HashMap<usize, usize>, tail: usize) -> Self {
    assert!(tail <= path.len());
    Self { path, first, tail }
  }

  pub fn tail(&self) -> usize {
    self.tail
  }

  pub fn cycle(&self) -> usize {
    self.path.len() - self.tail
  }

  pub fn has_cycle(&self) -> bool {
    self.cycle() != 0
  }

  pub fn index_of(&self, v: usize) -> Option<usize> {
    self.first.get(&v).copied()
  }

  pub fn get(&self, idx: usize) -> Option<usize> {
    if idx < self.tail() {
      Some(self.path[idx])
    } else if self.has_cycle() {
      Some(self.path[self.tail() + (idx - self.tail()) % self.cycle()])
    } else {
      None
    }
  }
}