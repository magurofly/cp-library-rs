use super::*;

/// 常に空のグラフ
/// 復元付きの関数でコストだけ求めたいときなどに利用する
pub trait EmptyGraph<E>: Graph<E> {}

impl<E> Graph<E> for () {
  type Edge = (usize, E);

  fn n(&self) -> usize { 0 }
  fn m(&self) -> usize { 0 }

  fn edge_weight(&self, _from: usize, _to: usize) -> Option<&E> { None }

  fn each_edge_from(&self, _from: usize, _f: impl FnMut(&Self::Edge)) {}
}

impl<E> GraphMut<E> for () {
  fn new_graph(_n: usize) -> () { () }
  fn add_arc(&mut self, _from: usize, _to: usize, _weight: E) {}
  fn add_edge(&mut self, _from: usize, _to: usize, _weight: E) {}
  fn edge_weight_mut(&mut self, _from: usize, _to: usize) -> Option<&mut E> { None }
  fn clear_edges(&mut self, _from: usize) {}
}

impl<E> EmptyGraph<E> for () {}

/// 頂点数・辺数だけ数えるグラフ
pub trait CountGraph<E>: Graph<E> {}

impl<E> Graph<E> for (usize, usize) {
  type Edge = (usize, E);

  fn n(&self) -> usize { self.0 }
  fn m(&self) -> usize { self.1 }

  fn edge_weight(&self, _from: usize, _to: usize) -> Option<&E> { None }

  fn each_edge_from(&self, _from: usize, _f: impl FnMut(&Self::Edge)) {} 
}

impl<E> GraphMut<E> for (usize, usize) {
  fn new_graph(n: usize) -> (usize, usize) { (n, 0) }
  fn add_arc(&mut self, _from: usize, _to: usize, _weight: E) { self.1 += 1; }
  fn add_edge(&mut self, _from: usize, _to: usize, _weight: E) { self.1 += 2; }
  fn edge_weight_mut(&mut self, _from: usize, _to: usize) -> Option<&mut E> { None }
  fn clear_edges(&mut self, _from: usize) {}
}