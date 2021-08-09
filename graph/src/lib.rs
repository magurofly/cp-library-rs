pub mod vec_graph;

pub trait Graph<E> {
  fn n(&self) -> usize;
  fn m(&self) -> usize;
}

pub use vec_graph::VecGraph;