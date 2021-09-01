pub trait Edge<E> {
  fn new(to: usize, weight: E) -> Self;
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl<E> Edge<E> for (usize, E) {
  fn new(to: usize, weight: E) -> Self {
    (to, weight)
  }

  fn to(&self) -> usize {
    self.0
  }

  fn weight(&self) -> &E {
    &self.1
  }
}

impl Edge<()> for usize {
  fn new(to: usize, _weight: ()) -> Self {
    to
  }

  fn to(&self) -> usize {
    *self
  }

  fn weight(&self) -> &() {
    &()
  }
}