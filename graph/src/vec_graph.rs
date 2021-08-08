use std::ops::IndexMut;

pub trait VecGraph<E>: IndexMut<usize> {
  fn add_arc(&mut self, from: usize, to: usize, weight: E);
  fn add_edge(&mut self, from: usize, to: usize, weight: E) where E: Clone {
    self.add_arc(from, to, weight.clone());
    self.add_arc(to, from, weight);
  }

  fn add_arcs<Ed: EdgeData<E>>(&mut self, arcs: impl IntoIterator<Item = Ed>) where E: Clone {
    for arc in arcs {
      self.add_arc(arc.from(), arc.to(), arc.weight().clone());
    }
  }

  fn add_edges<Ed: EdgeData<E>>(&mut self, edges: impl IntoIterator<Item = Ed>) where E: Clone {
    for edge in edges {
      self.add_edge(edge.from(), edge.to(), edge.weight().clone());
    }
  }
}

impl<E> VecGraph<E> for Vec<Vec<(usize, E)>> {
  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push((to, weight));
  }
}

impl VecGraph<()> for Vec<Vec<usize>> {
  fn add_arc(&mut self, from: usize, to: usize, _weight: ()) {
    self[from].push(to);
  }
}

pub trait Edge<E> {
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl<E> Edge<E> for (usize, E) {
  fn to(&self) -> usize {
    self.0
  }

  fn weight(&self) -> &E {
    &self.1
  }
}

impl Edge<()> for usize {
  fn to(&self) -> usize {
    *self
  }

  fn weight(&self) -> &() {
    &()
  }
}

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