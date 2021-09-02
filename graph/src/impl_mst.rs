use super::*;
use template::*;

pub fn mst_prim_by_heap<E, C: Copy + Default + Ord + std::ops::Add<Output = C>, G: Graph<E>, H: GraphMut<C>>(g: &G, root: usize, mut heap: impl Heap<(C, usize, usize)>, mut cost: impl FnMut(&G::Edge) -> C) -> (C, H) {
  assert!(root < g.n());
  let mut h = H::new_graph(g.n());
  let mut cost_sum = C::default();
  let mut used = vec![false; g.n()];
  used[root] = true;
  g.each_edge_from(root, |e| heap.push(((cost)(e), root, e.to())));
  while let Some((c, from, u)) = heap.pop() {
    if used[u] { continue; }
    used[u] = true;
    cost_sum = cost_sum + c;
    h.add_edge(from, u, c);
    g.each_edge_from(u, |e| {
      if used[e.to()] { return; }
      heap.push(((cost)(e), u, e.to()));
    });
  }
  (cost_sum, h)
}