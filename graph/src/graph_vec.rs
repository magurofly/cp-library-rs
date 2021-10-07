use std::ops::{Deref};

use super::*;

pub type UnweightedGraph = Vec<Vec<usize>>;
pub type WeightedGraph<T> = Vec<Vec<(usize, T)>>;

pub trait VecGraph<E, Ed: Edge<E>>: Graph<E, Edge = Ed> + GraphMut<E> + Deref<Target = [Vec<Ed>]> {
}

impl<E, Ed: Edge<E>> Graph<E> for Vec<Vec<Ed>> {
  type Edge = Ed;

  /// amortized O(E/V)
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

  /// amortized O(E/V)
  fn edge_weight(&self, from: usize, to: usize) -> Option<&E> {
    let mut r = None;
    for e in &self[from] {
      if e.to() == to {
        r = Some(e.weight());
      }
    }
    r
  }
}

impl<E, Ed: EdgeMut<E>> GraphMut<E> for Vec<Vec<Ed>> {
  fn new_graph(n: usize) -> Self {
    let mut g = Vec::with_capacity(n);
    g.resize_with(n, Vec::new);
    g
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push(Ed::new_edge(to, weight));
  }

  /// amortized O(E/V)
  fn edge_weight_mut(&mut self, from: usize, to: usize) -> Option<&mut E> {
    let mut r = None;
    for e in &mut self[from] {
      if e.to() == to {
        r = Some(e.weight_mut());
      }
    }
    r
  }

  fn clear_edges(&mut self, from: usize) {
    self[from].clear();
  }
}


#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_vec() {
    let mut g: Vec<Vec<(usize, i64)>> = vec![vec![]; 4];
    for &(u, v) in &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)] {
      g.add_edge(u, v, 1);
    }

    assert_eq!(g.dijkstra(0), vec![Some(0), Some(1), Some(1), Some(1)]);
  }
}