use super::*;
use template::*;

pub trait Graph<E> {
  type Edge: Edge<E>;

  fn n(&self) -> usize;
  fn m(&self) -> usize;

  fn each_edge_from(&self, from: usize, f: impl FnMut(&Self::Edge));

  // algorithms

  fn dijkstra_by_with_heap<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, heap: impl Heap<(C, usize)>, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> where Self: Sized {
    shortest_path::dijkstra(self, start, heap, cost)
  }

  fn dijkstra_by<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> where Self: Sized {
    shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), cost)
  }

  fn dijkstra(&self, start: usize) -> Vec<Option<E>> where E: Copy + std::ops::Add<Output = E> + Default + Ord, Self: Sized {
    shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), |edge, dist| Some(dist + *edge.weight()))
  }
}

pub trait GraphMut<E>: Graph<E> {
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