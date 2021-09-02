use super::*;

pub trait GraphMut<E>: Graph<E> {
  fn new_graph(n: usize) -> Self;
  
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