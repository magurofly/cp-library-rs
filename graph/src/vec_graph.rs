use std::ops::{Deref};

use super::*;

pub trait VecGraph<E, Ed: Edge<E>>: Graph<E, Edge = Ed> + Deref<Target = [Vec<Ed>]> {
}

impl<E, Ed: Edge<E>> Graph<E> for Vec<Vec<Ed>> {
  type Edge = Ed;

  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&Ed)) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

  fn n(&self) -> usize {
    self.len()
  }

  fn m(&self) -> usize {
    self.iter().map(|edges| edges.len()).sum()
  }
}

impl<E, Ed: Edge<E>> GraphMut<E> for Vec<Vec<Ed>> {
  fn new_graph(n: usize) -> Self {
    let mut g = Vec::with_capacity(n);
    g.resize_with(n, Vec::new);
    g
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push(Ed::new(to, weight));
  }
}
