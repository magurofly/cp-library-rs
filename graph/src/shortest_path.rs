use super::*;
use template::*;
use std::collections::*;

/// ダイクストラ法
/// `g`: グラフ
/// `start`: 始点
/// `heap`: 利用するヒープ（昇順）
/// `cost`: 次のコストを計算する関数
pub fn dijkstra<E, G: Graph<E>, C: Copy + std::ops::Add<Output = C> + Default + Ord>(g: &G, start: usize, mut heap: impl Heap<(C, usize)>, mut cost: impl FnMut(&G::Edge, C) -> Option<C>) -> Vec<Option<C>> {
  let mut dists = vec![None; g.n()];
  dists[start] = Some(C::default());
  heap.push((C::default(), start));
  while let Some((d1, u)) = heap.pop() {
    if dists[u].unwrap() != d1 { continue }
    g.each_edge_from(u, |e| {
      if let Some(d2) = (cost)(e, d1) {
        if dists[e.to()].map(|d3| d3 > d2 ).unwrap_or(true) {
          dists[e.to()] = Some(d2);
          heap.push((d2, e.to()));
        }
      }
    });
  }
  dists
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