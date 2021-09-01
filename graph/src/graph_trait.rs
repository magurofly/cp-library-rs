use super::*;
use template::*;

pub trait Graph<E>: Sized {
  type Edge: Edge<E>;

  fn n(&self) -> usize;
  fn m(&self) -> usize;

  fn each_edge_from(&self, from: usize, f: impl FnMut(&Self::Edge));

  fn each_edge(&self, mut f: impl FnMut(&EdgeInfo<E, Self::Edge>)) {
    for v in 0 .. self.n() {
      self.each_edge_from(v, |e| (f)(&EdgeInfo::new(v, e)));
    }
  }

  // utils

  fn rev<G: GraphMut<E>>(&self) -> G where E: Clone {
    let mut g = G::new_graph(self.n());
    self.each_edge(|e| {
      g.add_arc(e.to(), e.from(), e.weight().clone());
    });
    g
  }

  // algorithms

  fn dijkstra_by_with_heap<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, heap: impl Heap<(C, usize)>, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
    shortest_path::dijkstra(self, start, heap, cost)
  }

  fn dijkstra_by<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
    shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), cost)
  }

  fn dijkstra(&self, start: usize) -> Vec<Option<E>> where E: Copy + std::ops::Add<Output = E> + Default + Ord, Self: Sized {
    shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), |edge, dist| Some(dist + *edge.weight()))
  }

  fn bfs_multistart(&self, start: impl IntoIterator<Item = usize>) -> Vec<Option<usize>> {
    shortest_path::bfs(self, start)
  }

  fn bfs(&self, start: usize) -> Vec<Option<usize>> {
    shortest_path::bfs(self, Some(start))
  }

  /// DFS をする
  /// `f(whence, v)`
  /// whence: `Pre` (先行順) , `Mid` (中間) , `Post` (後行順)
  /// pre, mid, post をすべてするとオイラーツアーになる
  fn dfs(&self, start: usize, f: impl FnMut(DfsOrder, usize)) where Self: Sized {
    dfs_impl::dfs(self, start, f);
  }

  fn dfs_preorder(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    dfs_impl::dfs(self, start, |whence, u| {
      match whence {
        DfsOrder::Pre => { (f)(u); }
        _ => {}
      }
    });
  }

  fn dfs_postorder(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    dfs_impl::dfs(self, start, |whence, u| {
      match whence {
        DfsOrder::Post => { (f)(u); }
        _ => {}
      }
    });
  }

  fn dfs_eulertour(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    let mut last = self.n();
    dfs_impl::dfs(self, start, |_, u| {
      if last != u {
        last = u;
        (f)(u);
      }
    });
  }
}