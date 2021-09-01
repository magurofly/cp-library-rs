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

pub struct EdgeInfo<'a, E, Ed: Edge<E>> {
  from: usize,
  edge: &'a Ed,
  phantom: PhantomData<E>,
}

impl<'a, E, Ed: Edge<E>> EdgeInfo<'a, E, Ed> {
  pub fn new(from: usize, edge: &'a Ed) -> Self {
    Self {
      from,
      edge,
      phantom: PhantomData,
    }
  }
}

impl<'a, E, Ed: Edge<E>> EdgeData<E> for EdgeInfo<'a, E, Ed> {
  fn from(&self) -> usize {
    self.from
  }

  fn to(&self) -> usize {
    self.edge.to()
  }

  fn weight(&self) -> &E {
    self.edge.weight()
  }
}