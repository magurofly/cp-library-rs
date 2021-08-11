use template::Heap;

#[derive(Debug, Clone)]
pub struct DialHeap<T> {
  dial: Vec<Vec<(usize, T)>>,
  limit: usize,
  current: usize,
  size: usize,
}

impl<T: Clone> DialHeap<T> {
  pub fn new(max: usize) -> Self {
    Self {
      dial: vec![vec![]; max + 1],
      current: 0,
      limit: max + 1,
      size: 0,
    }
  }
}

impl<T> Heap<(usize, T)> for DialHeap<T> {
  /// `push((key, value))`
  /// 前回に `pop()` した値のキーを `x` とすると、 `x <= key <= x + max` が成り立っている必要がある
  fn push(&mut self, pair: (usize, T)) {
    assert!(pair.0 >= self.current);
    self.size += 1;
    self.dial[(pair.0 - self.current) % self.limit].push(pair);
  }

  fn pop(&mut self) -> Option<(usize, T)> {
    if self.size == 0 {
      return None;
    }
    self.size -= 1;
    loop {
      if !self.dial[self.current].is_empty() {
        return self.dial[self.current].pop();
      }
      self.current = (self.current + 1) % self.limit;
    }
  }

  fn is_empty(&self) -> bool {
    self.size == 0
  }
}