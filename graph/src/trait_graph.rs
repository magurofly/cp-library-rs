use super::*;
use template::*;
use std::ops::*;

pub trait Graph<E>: Sized {
  type Edge: Edge<E>;

  fn n(&self) -> usize;
  fn m(&self) -> usize;

  fn edge_weight(&self, from: usize, to: usize) -> Option<&E>;

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

  /// 多重辺を削除したグラフを作成する
  fn unique_edge<G: GraphMut<E>>(&self) -> G where E: Clone {
    let mut prev = vec![0; self.n()];
    let mut g = G::new_graph(self.n());
    for u in 0 .. self.n() {
      self.each_edge_from(u, |e| {
        if prev[e.to()] != u + 1 {
          prev[e.to()] = u + 1;
          g.add_arc(u, e.to(), e.weight().clone());
        }
      });
    }
    g
  }

  /// 多重辺を合成したグラフを作成する
  fn merge_edge<G: GraphMut<E>>(&self, mut merge: impl FnMut(E, E) -> E) -> G where E: Clone {
    let mut prev = vec![0; self.n()];
    let mut g = G::new_graph(self.n());
    for u in 0 .. self.n() {
      self.each_edge_from(u, |e| {
        if prev[e.to()] != u + 1 {
          prev[e.to()] = u + 1;
          g.add_arc(u, e.to(), e.weight().clone());
        } else {
          if let Some(weight) = g.edge_weight_mut(u, e.to()) {
            *weight = (merge)(weight.clone(), e.weight().clone());
          }
        }
      });
    }
    g
  }

  // algorithms

  fn dijkstra_graph_with_heap_by<R: GraphMut<C>, C: Copy + Add<Output = C> + Sub<Output = C> + Default + Ord>(&self, start: usize, heap: impl Heap<(C, usize)>, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> (Vec<Option<C>>, R) {
    impl_shortest_path::dijkstra(self, start, heap, cost)
  }

  fn dijkstra_graph_by<C: Copy + Add<Output = C> + Sub<Output = C> + Default + Ord, R: GraphMut<C>>(&self, start: usize, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> (Vec<Option<C>>, R) where Self: Sized {
    self.dijkstra_graph_with_heap_by(start, BinaryHeapReversed::new(), cost)
  }

  /// ダイクストラ法で最短路と最短路グラフを求める
  /// O(E log V)
  /// `R`: 最短路グラフの型
  /// * 最短路グラフでは、最短路で直前に通った可能性がある頂点への辺がある
  /// * 終点から始点までたどると復元できる
  /// * いい感じにDPをすると数え上げもできる（面倒かも）
  fn dijkstra_graph<R: GraphMut<E>>(&self, start: usize) -> (Vec<Option<E>>, R) where E: Copy + Add<Output = E> + Sub<Output = E> + Default + Ord, Self: Sized {
    self.dijkstra_graph_with_heap_by(start, BinaryHeapReversed::new(), |edge, dist| Some(dist + *edge.weight()))
  }

  fn dijkstra_with_heap_by<C: Copy + Add<Output = C> + Sub<Output = C> + Default + Ord>(&self, start: usize, heap: impl Heap<(C, usize)>, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
    let (d, _): (Vec<Option<C>>, ()) = impl_shortest_path::dijkstra(self, start, heap, cost);
    d
  }

  /// ダイクストラ法で最短路を求める
  /// O(E log V)
  /// `(cost)(e, d)`: 次の頂点までの距離を計算する関数
  /// * `e`: 辺
  /// * `d`: 現在の距離
  /// * `(cost)(e, d) >= d` となる必要がある
  fn dijkstra_by<C: Copy + Add<Output = C> + Sub<Output = C> + Default + Ord>(&self, start: usize, cost: impl FnMut(&Self::Edge, C) -> Option<C>) -> Vec<Option<C>> {
    self.dijkstra_with_heap_by(start, BinaryHeapReversed::new(), cost)
  }

  /// ダイクストラ法で最短路を求める
  /// O(E log V)
  fn dijkstra(&self, start: usize) -> Vec<Option<E>> where E: Copy + Add<Output = E> + Sub<Output = E> + Default + Ord, Self: Sized {
    self.dijkstra_by(start, |edge, dist| Some(dist + *edge.weight()))
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

  fn mst_by_with_heap<C: Copy + Default + Ord + Add<Output = C>, G: GraphMut<C>>(&self, root: usize, heap: impl Heap<(C, usize, usize)>, cost: impl FnMut(&Self::Edge) -> C) -> (C, G) { impl_mst::mst_prim_by_heap(self, root, heap, cost) }
  fn mst_by<C: Copy + Default + Ord + Add<Output = C>, G: GraphMut<C>>(&self, root: usize, cost: impl FnMut(&Self::Edge) -> C) -> (C, G) { impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), cost) }
  fn mst<G: GraphMut<E>>(&self, root: usize) -> (E, G) where E: Copy + Default + Ord + Add<Output = E> { impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), |e| *e.weight()) }
  fn mst_cost(&self, root: usize) -> E where E: Copy + Default + Ord + Add<Output = E> { let (cost, g) = impl_mst::mst_prim_by_heap(self, root, BinaryHeapReversed::new(), |e| *e.weight()); let _: () = g; cost }
}