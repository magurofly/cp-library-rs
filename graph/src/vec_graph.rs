use std::ops::{Deref};
use std::collections::*;

use template::*;
use super::*;

pub trait VecGraph<E, Ed: Edge<E>>: Graph<E, Edge = Ed> + Deref<Target = [Vec<Ed>]> {
  fn bfs_multistart(&self, start: impl IntoIterator<Item = usize>) -> Vec<Option<usize>> {
    let mut queue = start.into_iter().collect::<VecDeque<_>>();
    let mut dist = vec![None; self.n()];
    for &u in &queue {
      dist[u] = Some(0);
    }
    while let Some(u) = queue.pop_front() {
      let d1 = dist[u].unwrap();
      for e in &self[u] {
        if dist[e.to()].is_none_or(|&d2| d2 > d1 + 1) {
          dist[e.to()] = Some(d1 + 1);
          queue.push_back(e.to());
        }
      }
    }
    dist
  }

  fn bfs(&self, start: usize) -> Vec<Option<usize>> {
    self.bfs_multistart(Some(start))
  }

  /// DFS をする
  /// `f(whence, v)`
  /// whence: `Pre` (先行順) , `Mid` (中間) , `Post` (後行順)
  /// pre, mid, post をすべてするとオイラーツアーになる
  fn dfs(&self, start: usize, f: impl FnMut(Whence, usize)) where Self: Sized {
    //TODO: 非再帰にする
    struct Dfs<F: FnMut(Whence, usize)> {
      visited: Vec<bool>,
      f: F
    }
    fn rec<E, Ed: Edge<E>>(graph: &impl VecGraph<E, Ed>, dfs: &mut Dfs<impl FnMut(Whence, usize)>, u: usize) {
      dfs.visited[u] = true;
      (dfs.f)(Whence::Pre, u);
      let mut first = true;
      for e in &graph[u] {
        if dfs.visited[e.to()] {
          continue;
        }
        if first {
          first = false;
        } else {
          (dfs.f)(Whence::Mid, u);
        }
        rec(graph, dfs, e.to());
      }
      (dfs.f)(Whence::Post, u);
    }
    let mut dfs = Dfs { visited: vec![false; self.n()], f };
    rec(self, &mut dfs, start);
  }

  fn dfs_preorder(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    self.dfs(start, |whence, u| {
      match whence {
        Whence::Pre => { (f)(u); }
        _ => {}
      }
    });
  }

  fn dfs_postorder(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    self.dfs(start, |whence, u| {
      match whence {
        Whence::Post => { (f)(u); }
        _ => {}
      }
    });
  }

  fn dfs_eulertour(&self, start: usize, mut f: impl FnMut(usize)) where Self: Sized {
    let mut last = self.n();
    self.dfs(start, |_, u| {
      if last != u {
        last = u;
        (f)(u);
      }
    });
  }
}

impl<E, Ed: Edge<E>> Graph<E> for Vec<Vec<Ed>> {
  type Edge = Ed;

  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&Ed)) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

  fn n(&self) -> usize {
    self.len()
  }

  fn m(&self) -> usize {
    self.iter().map(|edges| edges.len()).sum()
  }
}

impl<E, Ed: Edge<E>> GraphMut<E> for Vec<Vec<Ed>> {
  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push(Ed::new(to, weight));
  }
}

pub enum Whence {
  Pre,
  Mid,
  Post,
}