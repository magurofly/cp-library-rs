pub trait Edge<E>: Sized {
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl Edge<usize> for usize {
  fn to(&self) -> usize { *self }
  fn weight(&self) -> &usize { &1 }
}

impl<E> Edge<E> for (usize, E) {
  fn to(&self) -> usize { self.0 }
  fn weight(&self) -> &E { &self.1 }
}

#[derive(Debug, Clone, Copy)]
pub struct EdgeData<'a, E>(usize, &'a E);
impl<E> Edge<E> for EdgeData<'_, E> {
  fn to(&self) -> usize { self.0 }
  fn weight(&self) -> &E { self.1 }
}
