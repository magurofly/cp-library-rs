pub struct LCA {
  table: Vec<Vec<usize>>,
  depth: Vec<usize>,
  log: usize,
}
impl LCA {
  pub fn new<T>(graph: &[Vec<(usize, T)>], root: usize) -> Self {
    Self::new_multiple(graph, &[root])
  }

  pub fn new_multiple<T>(graph: &[Vec<(usize, T)>], roots: &[usize]) -> Self {
    let n = graph.len();
    let log = (0usize.leading_zeros() - n.leading_zeros()) as usize;
    let mut table = vec![vec![n; n]];
    let mut depth = vec![0; n];
    let mut stack = roots.to_vec();
    let mut visited = vec![false; n];
    for &root in roots {
      table[0][root] = root;
      visited[root] = true;
    }
    while let Some(u) = stack.pop() {
      for e in &graph[u] {
        if !visited[e.0] {
          visited[e.0] = true;
          table[0][e.0] = u;
          depth[e.0] = depth[u] + 1;
          stack.push(e.0);
        }
      }
    }

    for k in 1 .. log {
      table.push((0 .. n).map(|v| table[k - 1][table[k - 1][v]]).collect());
    }

    Self { table, depth, log }
  }

  pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
    if self.depth[u] < self.depth[v] {
      std::mem::swap(&mut u, &mut v);
    }
    for k in 0 .. self.log {
      if (self.depth[u] - self.depth[v] >> k & 1) != 0 {
        u = self.table[k as usize][u];
      }
    }
    if u == v {
      return u;
    }
    for row in self.table.iter().rev() {
      if row[u] != row[v] {
        u = row[u];
        v = row[v];
      }
    }
    self.table[0][u]
  }
}
