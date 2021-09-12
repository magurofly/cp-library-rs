pub trait Edge<E> {
  fn new_edge(to: usize, weight: E) -> Self;
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

pub trait EdgeMut<E>: Edge<E> {
  fn weight_mut(&mut self) -> &mut E;
}

impl<E> Edge<E> for (usize, E) {
  fn new_edge(to: usize, weight: E) -> Self {
    (to, weight)
  }

  fn to(&self) -> usize {
    self.0
  }

  fn weight(&self) -> &E {
    &self.1
  }
}

impl<E> EdgeMut<E> for (usize, E) {
  fn weight_mut(&mut self) -> &mut E {
    &mut self.1
  }
}

impl Edge<()> for usize {
  fn new_edge(to: usize, _weight: ()) -> Self {
    to
  }

  fn to(&self) -> usize {
    *self
  }

  fn weight(&self) -> &() {
    &()
  }
}
