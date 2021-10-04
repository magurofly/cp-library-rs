use super::*;
use unideque::*;

/// http://www.prefield.com/algorithm/graph/strongly_connected_components.html
pub fn scc_tarjan<E, G: Graph<E>>(g: &G) -> Vec<Vec<usize>> {
  #[derive(Default)]
  struct Dfs {
    scc: Vec<Vec<usize>>,
    num: Vec<usize>,
    low: Vec<usize>,
    t: usize,
    stack: Unideque<usize, Vec<bool>>,
  }
  fn dfs<E>(d: &mut Dfs, g: &impl Graph<E>, u: usize) {
    d.t += 1;
    d.low[u] = d.t;
    d.num[u] = d.t;
    d.stack.push_back(u);
    g.each_edge_from(u, |e| {
      let v = e.to();
      if d.num[v] == 0 {
        dfs(d, g, v);
        d.low[u] = d.low[u].min(d.low[v]);
      } else {
        d.low[u] = d.low[u].min(d.num[v]);
      }
    });
    if d.low[u] == d.num[u] {
      let mut component = vec![];
      while let Some(v) = d.stack.pop_back() {
        component.push(v);
        if v == u {
          break;
        }
      }
      d.scc.push(component);
    }
  }

  let mut d = Dfs {
    num: vec![0; g.n()],
    low: vec![0; g.n()],
    ..Dfs::default()
  };
  for u in 0 .. g.n() {
    if d.num[u] == 0 {
      dfs(&mut d, g, u);
    }
  }
  while let Some(v) = d.stack.pop_back() {
    d.scc.push(vec![v]);
  }
  d.scc.reverse();
  d.scc
}

#[cfg(test)]
pub mod test {
  use super::*;

  fn go_to<E>(g: &impl Graph<E>, from: usize, to: usize) -> bool {
    let mut ok = false;
    g.dfs_preorder(from, |v| {
      if v == to {
        ok = true;
      }
    });
    ok
  }

  #[test]
  fn test_scc_tarjan() {
    let mut g = vec![vec![]; 6];
    let edges = vec![(5, 2), (0, 1), (1, 2), (2, 0), (0, 3), (4, 1)];
    for &(u, v) in &edges {
      g[u].push(v);
    }
    let scc = scc_tarjan(&g);
    assert!(scc.windows(2).all(|w| {
      w[1].iter().all(|&u| w[0].iter().all(|&v| !go_to(&g, u, v)))
    }));
  }
}