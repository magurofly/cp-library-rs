use std::{marker::PhantomData, ops::{Deref, DerefMut, Index}};

use super::*;

pub type GridVecGraph<E> = GridGraph<Vec<Vec<(usize, E)>>, E>;

pub struct GridGraph<G, E> {
  grid: Vec<Vec<char>>,
  graph: G,
  phantom: PhantomData<E>,
}

impl<E, G: Graph<E>> GridGraph<G, E> {
  pub fn rows(&self) -> usize {
    self.grid.len()
  }

  pub fn cols(&self) -> usize {
    self.grid[0].len()
  }

  pub fn id(&self, (i, j): (usize, usize)) -> usize {
    i * self.cols() + j
  }

  pub fn each_vertex(&self, mut f: impl FnMut(usize, usize, char)) {
    for i in 0 .. self.rows() {
      for j in 0 .. self.cols() {
        (f)(i, j, self.grid[i][j]);
      }
    }
  }
}

impl<E, G: GraphMut<E>> GridGraph<G, E> {
  pub fn connect_4neighbor_if(&mut self, mut f: impl FnMut((usize, usize), (usize, usize)) -> Option<E>) {
    for i in 0 .. self.rows().saturating_sub(1) {
      for j in 0 .. self.cols() {
        for &(u, v) in &[((i, j), (i + 1, j)), ((i + 1, j), (i, j))] {
          if let Some(w) = (f)(u, v) {
            self.graph.add_arc(self.id(u), self.id(v), w);
          }
        }
      }
    }
    for j in 0 .. self.cols().saturating_sub(1) {
      for i in 0 .. self.rows() {
        for &(u, v) in &[((i, j), (i, j + 1)), ((i, j + 1), (i, j))] {
          if let Some(w) = (f)(u, v) {
            self.graph.add_arc(self.id(u), self.id(v), w);
          }
        }
      }
    }
  }
}

impl<E, G> Index<usize> for GridGraph<G, E> {
  type Output = [char];

  fn index(&self, idx: usize) -> &[char] {
    &self.grid[idx]
  }
}

impl<E, G> Index<(usize, usize)> for GridGraph<G, E> {
  type Output = char;

  fn index(&self, idx: (usize, usize)) -> &char {
    &self.grid[idx.0][idx.1]
  }
}

impl<E, G> Deref for GridGraph<G, E> {
  type Target = G;

  fn deref(&self) -> &G {
    &self.graph
  }
}

impl<E, G> DerefMut for GridGraph<G, E> {
  fn deref_mut(&mut self) -> &mut G {
    &mut self.graph
  }
}

impl<E, G: GraphMut<E>> From<Vec<Vec<char>>> for GridGraph<G, E> {
  fn from(grid: Vec<Vec<char>>) -> Self {
    let rows = grid.len();
    let cols = grid[0].len();
    GridGraph { grid, graph: G::new_graph(rows * cols), phantom: PhantomData, }
  }
}