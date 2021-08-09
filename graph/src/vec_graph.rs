use std::ops::{Deref};
use std::collections::*;

use template::*;
use super::Graph;

pub trait VecGraph<E, Ed: Edge<E>>: Graph<E> + Deref<Target = [Vec<Ed>]> {
  fn each_edge_from(&self, from: usize, f: impl FnMut(&Ed));
  
  fn add_arc(&mut self, from: usize, to: usize, weight: E);
  fn add_edge(&mut self, from: usize, to: usize, weight: E) where E: Clone {
    self.add_arc(from, to, weight.clone());
    self.add_arc(to, from, weight);
  }

  fn add_arcs<D: EdgeData<E>>(&mut self, arcs: impl IntoIterator<Item = D>) where E: Clone {
    for arc in arcs {
      self.add_arc(arc.from(), arc.to(), arc.weight().clone());
    }
  }

  fn add_edges<D: EdgeData<E>>(&mut self, edges: impl IntoIterator<Item = D>) where E: Clone {
    for edge in edges {
      self.add_edge(edge.from(), edge.to(), edge.weight().clone());
    }
  }

  fn dijkstra_by<C: Copy + std::ops::Add<Output = C> + Default + Ord>(&self, start: usize, mut cost: impl FnMut(&Ed, C) -> Option<C>) -> Vec<Option<C>> {
    let mut dists = vec![None; self.n()];
    dists[start] = Some(C::default());
    let mut pq = BinaryHeap::new();
    pq.push((std::cmp::Reverse(C::default()), start));
    while let Some((std::cmp::Reverse(d1), u)) = pq.pop() {
      if dists[u].unwrap() != d1 { continue }
      for e in &self[u] {
        if let Some(d2) = (cost)(e, d1) {
          if dists[e.to()].map(|d3| d3 > d2 ).unwrap_or(true) {
            dists[e.to()] = Some(d2);
            pq.push((std::cmp::Reverse(d2), e.to()));
          }
        }
      }
    }
    dists
  }

  fn dijkstra(&self, start: usize) -> Vec<Option<E>> where E: Copy + std::ops::Add<Output = E> + Default + Ord {
    self.dijkstra_by(start, |edge, dist| Some(dist + *edge.weight()))
  }

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
  fn n(&self) -> usize {
    self.len()
  }

  fn m(&self) -> usize {
    self.iter().map(|edges| edges.len()).sum()
  }
}

impl<E> VecGraph<E, (usize, E)> for Vec<Vec<(usize, E)>> {
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&(usize, E))) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self[from].push((to, weight));
  }
}

impl VecGraph<(), usize> for Vec<Vec<usize>> {
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&usize)) {
    for edge in &self[from] {
      (f)(edge);
    }
  }

  fn add_arc(&mut self, from: usize, to: usize, _weight: ()) {
    self[from].push(to);
  }
}

pub trait Edge<E> {
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl<E> Edge<E> for (usize, E) {
  fn to(&self) -> usize {
    self.0
  }

  fn weight(&self) -> &E {
    &self.1
  }
}

impl Edge<()> for usize {
  fn to(&self) -> usize {
    *self
  }

  fn weight(&self) -> &() {
    &()
  }
}

pub trait EdgeData<E> {
  fn from(&self) -> usize;
  fn to(&self) -> usize;
  fn weight(&self) -> &E;
}

impl<E> EdgeData<E> for (usize, usize, E) {
  fn from(&self) -> usize {
    self.0
  }
  
  fn to(&self) -> usize {
    self.1
  }

  fn weight(&self) -> &E {
    &self.2
  }
}

impl EdgeData<()> for (usize, usize) {
  fn from(&self) -> usize {
    self.0
  }
  
  fn to(&self) -> usize {
    self.1
  }

  fn weight(&self) -> &() {
    &()
  }
}

pub enum Whence {
  Pre,
  Mid,
  Post,
}