use super::*;

pub trait GraphMut<E>: Graph<E> {
  fn new_graph(n: usize) -> Self;
  
  fn add_arc(&mut self, from: usize, to: usize, weight: E);

  fn add_edge(&mut self, from: usize, to: usize, weight: E) where E: Clone {
    self.add_arc(from, to, weight.clone());
    self.add_arc(to, from, weight);
  }

  fn connect(&mut self, u: usize, v: usize) where E: Default {
    self.add_arc(u, v, E::default());
    self.add_arc(v, u, E::default());
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

  fn edge_weight_mut(&mut self, from: usize, to: usize) -> Option<&mut E>;

  fn clear_edges(&mut self, from: usize);
}