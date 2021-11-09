use std::cmp::*;
use std::mem::*;

pub struct LeftistHeap<T: Ord> {
  root: Option<Box<LeftistHeapNode<T>>>,
  len: usize,
}

impl<T: Ord> LeftistHeap<T> {
  pub fn new() -> Self {
    Self {
      root: None,
      len: 0,
    }
  }

  pub fn push(&mut self, key: T) {
    self.len += 1;
    let node = LeftistHeapNode::new(key);
    if let Some(root) = self.root.take() {
      self.root = Some(root.meld(node));
    } else {
      self.root = Some(node);
    }
  }

  pub fn pop(&mut self) -> Option<T> {
    let (left, right) = self.root.as_mut()?.split();
    self.len -= 1;
    let root = replace(&mut self.root, Self::meld(left, right));
    root.map(|n| n.key())
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn merge(&mut self, mut other: Self) {
    let root = Self::meld(self.root.take(), other.root.take());
    self.root = root;
    self.len += other.len;
  }

  fn meld(left: Option<Box<LeftistHeapNode<T>>>, right: Option<Box<LeftistHeapNode<T>>>) -> Option<Box<LeftistHeapNode<T>>> {
    if let Some(l) = left {
      if let Some(r) = right {
        Some(l.meld(r))
      } else {
        Some(l)
      }
    } else {
      right
    }
  }
}

impl<T: Ord> std::iter::FromIterator<T> for LeftistHeap<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    let mut heap = Self::new();
    for x in iter {
      heap.push(x);
    }
    heap
  }
}

pub struct LeftistHeapNode<T: Ord> {
  key: T,
  to_leaf: usize,
  children: [Option<Box<LeftistHeapNode<T>>>; 2],
}

impl<T: Ord> LeftistHeapNode<T> {
  pub fn new(key: T) -> Box<Self> {
    Box::new(Self {
      key,
      to_leaf: 0,
      children: [None, None],
    })
  }

  pub fn key(self: Box<Self>) -> T {
    self.key
  }

  pub fn split(self: &mut Box<Self>) -> (Option<Box<Self>>, Option<Box<Self>>) {
    let left = self.children[0].take();
    let right = self.children[1].take();
    (left, right)
  }

  pub fn meld(mut self: Box<Self>, mut other: Box<Self>) -> Box<Self> {
    if self.key.cmp(&other.key) == Ordering::Less {
      swap(&mut self, &mut other);
    }
    if self.children[0].is_none() {
      self.children[0] = Some(other);
    } else {
      if let Some(child) = self.children[1].take() {
        other = child.meld(other);
      }
      self.children[1] = Some(other);
      if Self::to_leaf(&self.children[0]) < Self::to_leaf(&self.children[1]) {
        self.children.swap(0, 1);
      }
      self.to_leaf = Self::to_leaf(&self.children[1]) + 1;
    }
    self
  }

  fn to_leaf(node: &Option<Box<Self>>) -> usize {
    node.as_ref().map(|node| node.to_leaf).unwrap_or(0)
  }
}
