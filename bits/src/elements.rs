use super::*;

pub struct Elements<T: Bits> {
  bits: T,
  index: usize,
}

impl<T: Bits> Elements<T> {
  pub fn new(bits: T) -> Self {
    Self {
      bits,
      index: 0,
    }
  }
}

impl<T: Bits> Iterator for Elements<T> {
  type Item = usize;

  fn next(&mut self) -> Option<usize> {
    if self.bits == T::none() {
      return None;
    }
    while !self.bits.bit_at(0) {
      self.index += 1;
      self.bits = self.bits >> 1;
    }
    Some(self.index)
  }
}