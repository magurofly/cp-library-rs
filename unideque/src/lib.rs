use std::collections::*;
use collection::*;

macro_rules! delegate {
  ($member:ident, $f:ident($($param_name:ident: $param_type:ty),*) -> $ret:ty) => {
    pub fn $f(&self $(, $param_name: $param_type)*) -> $ret {
      self.$member.$f($($param_name),*)
    }
  };
}

pub struct Unideque<T, S: SetMut<T>> {
  deque: VecDeque<T>,
  set: S,
}

impl<T: Clone, S: SetMut<T>> Unideque<T, S> {

  delegate!(deque, len() -> usize);
  delegate!(deque, is_empty() -> bool);
  delegate!(deque, front() -> Option<&T>);
  delegate!(deque, back() -> Option<&T>);
  delegate!(deque, get(index: usize) -> Option<&T>);
  delegate!(deque, iter() -> vec_deque::Iter<'_, T>);
  
  pub fn contains(&self, value: &T) -> bool {
    self.set.has(value)
  }

  pub fn push_front(&mut self, value: T) -> bool {
    if self.set.has(&value) {
      return false;
    }
    self.set.add(value.clone());
    self.deque.push_front(value);
    true
  }

  pub fn push_back(&mut self, value: T) -> bool {
    if self.set.has(&value) {
      return false;
    }
    self.set.add(value.clone());
    self.deque.push_back(value);
    true
  }

  pub fn pop_front(&mut self) -> Option<T> {
    let value = self.deque.pop_front()?;
    self.set.delete(&value);
    Some(value)
  }

  pub fn pop_back(&mut self) -> Option<T> {
    let value = self.deque.pop_back()?;
    self.set.delete(&value);
    Some(value)
  }
}

impl<T, S: SetMut<T> + Default> Default for Unideque<T, S> {
  fn default() -> Self {
    Self {
      deque: VecDeque::new(),
      set: S::default(),
    }
  }
}

impl<T, S: SetMut<T>> From<S> for Unideque<T, S> {
  fn from(set: S) -> Self {
    Self {
      deque: VecDeque::new(),
      set,
    }
  }
}