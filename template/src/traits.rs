use std::collections::*;
use std::cmp::*;

pub trait Heap<T> {
  fn push(&mut self, item: T);
  fn pop(&mut self) -> Option<T>;
  fn is_empty(&self) -> bool;
}

impl<T: Ord> Heap<T> for BinaryHeap<T> {
  fn push(&mut self, item: T) {
    BinaryHeap::push(self, item);
  }

  fn pop(&mut self) -> Option<T> {
    BinaryHeap::pop(self)
  }

  fn is_empty(&self) -> bool {
    BinaryHeap::is_empty(self)
  }
}

pub struct BinaryHeapReversed<T>(BinaryHeap<Reverse<T>>);

impl<T: Ord> BinaryHeapReversed<T> {
  pub fn new() -> Self {
    Self(BinaryHeap::new())
  }
}

impl<T: Ord> Heap<T> for BinaryHeapReversed<T> {
  fn push(&mut self, item: T) {
    self.0.push(Reverse(item));
  }

  fn pop(&mut self) -> Option<T> {
    self.0.pop().map(|r| r.0)
  }

  fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
}