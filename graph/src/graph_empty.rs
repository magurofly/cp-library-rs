use super::*;

/// 常に空のグラフ
/// 復元付きの関数でコストだけ求めたいときなどに利用する
pub trait EmptyGraph<E>: Graph<E> {}

impl<E> Graph<E> for () {
  type Edge = (usize, E);

  fn n(&self) -> usize { 0 }
  fn m(&self) -> usize { 0 }

  fn each_edge_from(&self, _from: usize, _f: impl FnMut(&Self::Edge)) {}
}

impl<E> GraphMut<E> for () {
  fn new_graph(_n: usize) -> () { () }
  fn add_arc(&mut self, _from: usize, _to: usize, _weight: E) {}
  fn add_edge(&mut self, _from: usize, _to: usize, _weight: E) {}
}

impl<E> EmptyGraph<E> for () {}