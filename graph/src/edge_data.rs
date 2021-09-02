use std::marker::PhantomData;
use super::*;

pub trait EdgeData<E> {
  fn from(&self) -> usize;
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl<E> EdgeData<E> for (usize, usize, E) {
  fn from(&self) -> usize {
    self.0
  }
  
  fn to(&self) -> usize {
    self.1
  }

  fn weight(&self) -> &E {
    &self.2
  }
}

impl EdgeData<()> for (usize, usize) {
  fn from(&self) -> usize {
    self.0
  }
  
  fn to(&self) -> usize {
    self.1
  }

  fn weight(&self) -> &() {
    &()
  }
}

pub struct EdgeInfo<'a, E> {
  from: usize,
  to: usize,
  weight: &'a E,
}

impl<'a, E> EdgeInfo<'a, E> {
  pub fn new(from: usize, to: usize, weight: &'a E) -> Self {
    Self {
      from,
      to,
      weight,
    }
  }
}

impl<'a, E> EdgeData<E> for EdgeInfo<'a, E> {
  fn from(&self) -> usize {
    self.from
  }

  fn to(&self) -> usize {
    self.to
  }

  fn weight(&self) -> &E {
    self.weight
  }
}