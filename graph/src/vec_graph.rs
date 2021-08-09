use std::ops::{Deref};

use super::Graph;

pub trait VecGraph<E, Ed: Edge<E>>: Graph<E> + Deref<Target = [Vec<Ed>]> {
  fn each_edge_from(&self, from: usize, f: impl FnMut(&Ed));
  
  fn add_arc(&mut self, from: usize, to: usize, weight: E);
  fn add_edge(&mut self, from: usize, to: usize, weight: E) where E: Clone {
    self.add_arc(from, to, weight.clone());
    self.add_arc(to, from, weight);
  }

  fn add_arcs<D: EdgeData<E>>(&mut self, arcs: impl IntoIterator<Item = D>) where E: Clone {
    for arc in arcs {
      self.add_arc(arc.from(), arc.to(), arc.weight().clone());
    }
  }

  fn add_edges<D: EdgeData<E>>(&mut self, edges: impl IntoIterator<Item = D>) where E: Clone {
    for edge in edges {
      self.add_edge(edge.from(), edge.to(), edge.weight().clone());
    }
  }
}

impl<E, Ed: Edge<E>> Graph<E> for Vec<Vec<Ed>> {
  fn n(&self) -> usize {
    self.len()
  }

  fn m(&self) -> usize {
    self.iter().map(|edges| edges.len()).sum()
  }
}

impl<E> VecGraph<E, (usize, E)> for Vec<Vec<(usize, E)>> {
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&(usize, E))) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push((to, weight));
  }
}

impl VecGraph<(), usize> for Vec<Vec<usize>> {
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&usize)) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

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