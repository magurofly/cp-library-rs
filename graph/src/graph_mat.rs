use super::*;

pub struct MatGraph<E> {
  m: usize,
  mat: Vec<Vec<Option<E>>>,
}

impl<E: Clone> Graph<E> for MatGraph<E> {
  type Edge = (usize, E);

  fn n(&self) -> usize { self.mat.len() }
  
  fn m(&self) -> usize { self.m }

  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
    for v in 0 .. self.mat[from].len() {
      if let Some(weight) = self.mat[from][v].as_ref() {
        (f)(&(v, weight.clone()));
      }
    }
  }
}

impl<E: Clone> GraphMut<E> for MatGraph<E> {
  fn new_graph(n: usize) -> Self {
    Self { m: 0, mat: vec![vec![None; n]; n] }
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self.mat[from][to] = Some(weight);
  }
}