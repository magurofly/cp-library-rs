use super::*;

pub fn dfs<E, G: Graph<E>>(g: &G, start: usize, f: impl FnMut(DfsOrder, usize)) {
  //TODO: 非再帰にする
  struct Dfs<F: FnMut(DfsOrder, usize)> {
    visited: Vec<bool>,
    f: F
  }
  fn rec<E>(g: &impl Graph<E>, dfs: &mut Dfs<impl FnMut(DfsOrder, usize)>, u: usize) {
    dfs.visited[u] = true;
    (dfs.f)(DfsOrder::Pre, u);
    let mut first = true;
    g.each_edge_from(u, |e| {
      if dfs.visited[e.to()] {
        return;
      }
      if first {
        first = false;
      } else {
        (dfs.f)(DfsOrder::Mid, u);
      }
      rec(g, dfs, e.to());
    });
    (dfs.f)(DfsOrder::Post, u);
  }
  let mut dfs = Dfs { visited: vec![false; g.n()], f };
  rec(g, &mut dfs, start);
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_dfs() {
    let g = vec![vec![1], vec![2], vec![]];
    let mut s = vec![];
    g.dfs_eulertour(0, |v| s.push(v));
    assert_eq!(s, vec![0, 1, 2, 1, 0]);
  }
}