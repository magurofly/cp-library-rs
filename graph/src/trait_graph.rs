use super::*;
use template::*;

pub trait Graph<E>: Sized {
  type Edge: Edge<E>;

  fn n(&self) -> usize;
  fn m(&self) -> usize;

  fn each_edge_from(&self, from: usize, f: impl FnMut(&Self::Edge));

  fn each_edge(&self, mut f: impl FnMut(EdgeInfo<E>)) {
    for v in 0 .. self.n() {
      self.each_edge_from(v, |e| (f)(EdgeInfo::new(v, e.to(), e.weight())));
    }
  }

  // utils

  /// グラフを別の表現で複製する
  fn dup<G: GraphMut<E>>(&self) -> G where E: Clone {
    let mut g = G::new_graph(self.n());
    self.each_edge(|e| g.add_arc(e.from(), e.to(), e.weight().clone()));
    g
  }

  /// 辺を逆向きにしたグラフを作成する
  fn rev<G: GraphMut<E>>(&self) -> G where E: Clone {
    let mut g = G::new_graph(self.n());
    self.each_edge(|e| g.add_arc(e.to(), e.from(), e.weight().clone()));
    g
  }

  // algorithms

  fn dijkstra_by_with_heap<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, heap: impl Heap<(C, usize)>, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> { impl_shortest_path::dijkstra(self, start, heap, cost) }

  fn dijkstra_by<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
    impl_shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), cost)
  }

  fn dijkstra(&self, start: usize) -> Vec<Option<E>> where E: Copy + std::ops::Add<Output = E> + Default + Ord, Self: Sized {
    impl_shortest_path::dijkstra(self, start, BinaryHeapReversed::new(), |edge, dist| Some(dist + *edge.weight()))
  }

  fn bfs_multistart(&self, start: impl IntoIterator<Item = usize>) -> Vec<Option<usize>> {
    impl_shortest_path::bfs(self, start)
  }

  fn bfs(&self, start: usize) -> Vec<Option<usize>> {
    impl_shortest_path::bfs(self, Some(start))
  }

  /// DFS をする
  /// `f(whence, v)`
  /// whence: `Pre` (先行順) , `Mid` (中間) , `Post` (後行順)
  /// pre, mid, post をすべてするとオイラーツアーになる
  fn dfs(&self, start: usize, f: impl FnMut(DfsOrder, usize)) { impl_dfs::dfs(self, start, f); }

  /// 頂点を先行順で走査する
  fn dfs_preorder(&self, start: usize, mut f: impl FnMut(usize)) { impl_dfs::dfs(self, start, |whence, u| if matches!(whence, DfsOrder::Pre) { (f)(u); }); }

  /// 頂点を後行順で走査する
  fn dfs_postorder(&self, start: usize, mut f: impl FnMut(usize)) { impl_dfs::dfs(self, start, |whence, u| if matches!(whence, DfsOrder::Post) { (f)(u); }); }

  /// 頂点をオイラーツアー順で走査する
  fn dfs_eulertour(&self, start: usize, mut f: impl FnMut(usize)) {
    let mut last = self.n();
    impl_dfs::dfs(self, start, |_, u| if last != u { last = u; (f)(u); });
  }

  fn mst_by_with_heap<C: Copy + Default + Ord + std::ops::Add<Output = C>, G: GraphMut<C>>(&self, root: usize, heap: impl Heap<(C, usize, usize)>, cost: impl FnMut(&Self::Edge) -> C) -> (C, G) { impl_mst::mst_prim_by_heap(self, root, heap, cost) }
  fn mst_by<C: Copy + Default + Ord + std::ops::Add<Output = C>, G: GraphMut<C>>(&self, root: usize, cost: impl FnMut(&Self::Edge) -> C) -> (C, G) { impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), cost) }
  fn mst<G: GraphMut<E>>(&self, root: usize) -> (E, G) where E: Copy + Default + Ord + std::ops::Add<Output = E> { impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), |e| *e.weight()) }
  fn mst_cost(&self, root: usize) -> E where E: Copy + Default + Ord + std::ops::Add<Output = E> { let (cost, g) = impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), |e| *e.weight()); let _: () = g; cost }
}