# 最大流（Dinic + 容量スケーリング）

- 計算量 `O(VE log U)`

## 参考

- https://ei1333.github.io/library/graph/flow/dinic-capacity-scaling.hpp.html
- https://misawa.github.io/others/flow/library_design.html

```rs
//UPDATE: 2022-08-06 02:59
pub struct MaxFlowGraph {
  graph: Vec<Vec<usize>>,
  arcs: Vec<MaxFlowArc>,
  max_cap: i64,
  cap_sum: i64,
}
pub struct MaxFlowArc {
  pub from: usize,
  pub to: usize,
  pub capacity: i64,
  pub flow: i64,
}
impl MaxFlowArc {
  pub fn residual_cap(&self) -> i64 { self.capacity - self.flow }
}
impl MaxFlowGraph {
  /// 頂点のないグラフを初期化する
  pub fn new() -> Self { Self { graph: vec![], arcs: vec![], max_cap: 0, cap_sum: 0 } }
  /// 頂点数
  pub fn number_of_vertices(&self) -> usize { self.graph.len() }
  /// 辺数
  pub fn number_of_arcs(&self) -> usize { self.arcs.len() / 2 }
  /// 辺番号によって辺を取得
  pub fn arc(&self, index: usize) -> &MaxFlowArc { assert!(index < self.number_of_arcs()); &self.arcs[index * 2] }
  /// 頂点を追加し、頂点番号を返す
  pub fn add_vertex(&mut self) -> usize { let v = self.number_of_vertices(); self.graph.push(vec![]); v }
  /// 頂点を複数追加し、頂点番号を配列で返す
  pub fn add_vertices(&mut self, n: usize) -> Vec<usize> { (0 .. n).map(|_| self.add_vertex()).collect() }
  /// 辺を追加し、辺番号を返す
  pub fn add_arc(&mut self, from: usize, to: usize, capacity: i64) -> usize { assert!(from < self.number_of_vertices()); assert!(to < self.number_of_vertices()); assert!(capacity >= 0); let e = self.arcs.len(); self.arcs.push(MaxFlowArc { from, to, capacity, flow: 0 }); self.arcs.push(MaxFlowArc { from: to, to: from, capacity, flow: capacity }); self.graph[from].push(e); self.graph[to].push(e + 1); if self.max_cap < capacity { self.max_cap = capacity; } self.cap_sum += capacity; e / 2}
  /// `source` から `sink` へフローを流せるだけ流す
  pub fn flow(&mut self, source: usize, sink: usize) -> i64 { self.flow_limited(source, sink, self.cap_sum) }
  /// `source` から `sink` へフローを最大 `limit` まで流す
  pub fn flow_limited(&mut self, source: usize, sink: usize, limit: i64) -> i64 {
    let (graph, mut arcs, &mut max_cap, _) = self.split_mut();
    if max_cap == 0 { return 0; }
    let mut flow = 0;
    for bit in (0 ..= 0_u64.leading_zeros() - (max_cap as u64).leading_zeros()).rev() {
      let flow_cur = 1_i64 << bit;
      loop {
        let mut level = vec![graph.len(); graph.len()];
        if !Self::build_augment_path(&graph, &arcs, &mut level, source, sink, flow_cur) {
          break;
        }
        let mut indices = vec![0; graph.len()];
        flow += Self::find_augment_path(&graph, &level, &mut arcs, &mut indices, source, sink, flow_cur, limit - flow);
      }
    }
    flow
  }
  /// 現在の残余ネットワークで `source` からの到達可能判定
  /// 最小カットを求める場合、 `flow` を先に呼んでおく
  pub fn cut(&self, source: usize) -> Vec<bool> {
    assert!(source < self.graph.len());
    let mut visited = vec![false; self.graph.len()];
    visited[source] = true;
    let mut stack = vec![source];
    while let Some(u) = stack.pop() {
      for &e in &self.graph[u] {
        let v = self.arcs[e].to;
        if !visited[v] && self.arcs[e].residual_cap() > 0 {
          visited[v] = true;
          stack.push(v);
        }
      }
    }
    visited
  }
  fn build_augment_path(graph: &[Vec<usize>], arcs: &[MaxFlowArc], level: &mut [usize], s: usize, t: usize, base: i64) -> bool {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(s);
    level[s] = 0;
    while let Some(u) = queue.pop_front() {
      for &e in &graph[u] {
        let arc = &arcs[e];
        if arc.capacity - arc.flow >= base && level[arc.to] > level[u] + 1 {
          level[arc.to] = level[u] + 1;
          queue.push_back(arc.to);
        }
      }
    }
    level[t] < graph.len()
  }
  fn find_augment_path(graph: &[Vec<usize>], level: &[usize], arcs: &mut [MaxFlowArc], indices: &mut [usize], u: usize, sink: usize, base: i64, flow: i64) -> i64 {
    if u == sink { return flow; }
    let mut sum = 0;
    while indices[u] < graph[u].len() {
      let e = graph[u][indices[u]];
      let delta1 = arcs[e].residual_cap();
      if level[u] < level[arcs[e].to] && delta1 >= base {
        let delta2 = Self::find_augment_path(graph, level, arcs, indices, arcs[e].to, sink, base, delta1.min(flow - sum));
        if delta2 > 0 {
          arcs[e ^ 0].flow += delta2;
          arcs[e ^ 1].flow -= delta2;
          sum += delta2;
          if flow - sum < base {
            break;
          }
        }
      }
      indices[u] += 1;
    }
    sum
  }
  fn split_mut(&mut self) -> (&mut Vec<Vec<usize>>, &mut Vec<MaxFlowArc>, &mut i64, &mut i64) { unsafe { (&mut *(&mut self.graph as *mut _), &mut *(&mut self.arcs as *mut _), &mut *(&mut self.max_cap as *mut _), &mut *(&mut self.cap_sum as *mut _)) } }
}
```
