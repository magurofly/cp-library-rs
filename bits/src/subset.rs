use super::*;

pub struct SubSets<T: Bits> {
  superset: T,
  bits: T,
  finished: bool,
}

impl<T: Bits> SubSets<T> {
  pub fn new(bits: T) -> Self {
    Self {
      superset: bits,
      bits,
      finished: false,
    }
  }
}

impl<T: Bits> Iterator for SubSets<T> {
  type Item = T;
  fn next(&mut self) -> Option<T> {
    if self.finished {
      return None;
    }
    let value = self.bits;
    if self.bits == T::none() {
      self.finished = true;
    } else {
      self.bits = (self.bits - T::top(0)) & self.superset;
    }
    Some(value)
  }
}