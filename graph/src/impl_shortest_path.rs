use super::*;
use template::*;
use std::collections::*;
use std::ops::*;

/// ダイクストラ法
/// `g`: グラフ
/// `start`: 始点
/// `heap`: 利用するヒープ（昇順）
/// `cost`: 次のコストを計算する関数
/// 返り値: `(コスト, 最短経路グラフ)`
pub fn dijkstra<E, G: Graph<E>, R: GraphMut<C>, C: Copy + Add<Output = C> + Sub<Output = C> + Default + Ord>(g: &G, start: usize, mut heap: impl Heap<(C, usize)>, mut cost: impl FnMut(&G::Edge, C) -> Option<C>) -> (Vec<Option<C>>, R) {
  let mut dists = vec![None; g.n()];
  let mut tree = R::new_graph(g.n());
  dists[start] = Some(C::default());
  heap.push((C::default(), start));
  while let Some((d_prev, u)) = heap.pop() {
    if dists[u].unwrap() != d_prev { continue }
    g.each_edge_from(u, |e| {
      if let Some(d_next) = (cost)(&e, d_prev) {
        if let Some(d_curr) = dists[e.to()] {
          if d_curr == d_next {
            tree.add_arc(e.to(), u, d_next - d_prev);
          } else if d_curr > d_next {
            tree.clear_edges(e.to());
            tree.add_arc(e.to(), u, d_next - d_prev);
            dists[e.to()] = Some(d_next);
            heap.push((d_next, e.to()));
          }
        } else {
          tree.clear_edges(e.to());
          tree.add_arc(e.to(), u, d_next - d_prev);
          dists[e.to()] = Some(d_next);
          heap.push((d_next, e.to()));
        }
      }
    });
  }
  (dists, tree)
}

/// BFS
pub fn bfs<E, G: Graph<E>>(g: &G, start: impl IntoIterator<Item = usize>) -> Vec<Option<usize>> {
  let mut queue = start.into_iter().collect::<VecDeque<_>>();
  let mut dist = vec![None; g.n()];
  for &u in &queue {
    dist[u] = Some(0);
  }
  while let Some(u) = queue.pop_front() {
    let d1 = dist[u].unwrap();
    g.each_edge_from(u, |e| {
      if dist[e.to()].is_none_or(|&d2| d2 > d1 + 1) {
        dist[e.to()] = Some(d1 + 1);
        queue.push_back(e.to());
      }
    });
  }
  dist
}