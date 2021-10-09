use std::{marker::PhantomData, ops::{Deref, DerefMut}};

use super::*;

pub type GridVecGraph<E> = GridGraph<Vec<Vec<(usize, E)>>, E>;

pub struct GridGraph<G, E> {
  rows: usize,
  cols: usize,
  graph: G,
  phantom: PhantomData<E>,
}

impl<E, G: Graph<E>> GridGraph<G, E> {
  pub fn new(rows: usize, cols: usize) -> Self where G: GraphMut<E> {
    Self {
      rows,
      cols,
      graph: G::new_graph(rows * cols),
      phantom: PhantomData,
    }
  }

  pub fn rows(&self) -> usize {
    self.rows
  }

  pub fn cols(&self) -> usize {
    self.cols
  }

  pub fn id(&self, (i, j): (usize, usize)) -> usize {
    i * self.cols() + j
  }

  pub fn each_vertex(&self, mut f: impl FnMut(usize, usize)) {
    let r = self.rows();
    let c = self.cols();
    for i in 0 .. r {
      for j in 0 .. c {
        (f)(i, j);
      }
    }
  }

  pub fn vertices(&self) -> RectangleIterator {
    RectangleIterator::new(self.rows(), self.cols())
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

impl<E, G: GraphMut<E>> From<&Vec<Vec<char>>> for GridGraph<G, E> {
  fn from(grid: &Vec<Vec<char>>) -> Self {
    let rows = grid.len();
    let cols = grid[0].len();
    Self::new(rows, cols)
  }
}

pub struct RectangleIterator {
  h: usize, w: usize,
  i: usize, j: usize,
}
impl RectangleIterator {
  pub fn new(h: usize, w: usize) -> Self {
    Self {
      h, w,
      i: 0, j: 0,
    }
  }
}
impl Iterator for RectangleIterator {
  type Item = (usize, usize);

  fn next(&mut self) -> Option<Self::Item> {
    if self.i < self.h {
      let res = (self.i, self.j);
      if self.j + 1 < self.w {
        self.j += 1;
      } else {
        self.i += 1;
        self.j = 0;
      }
      Some(res)
    } else {
      None
    }
  }
}

pub fn dbg_grid<G: IntoIterator<Item = H>, H: IntoIterator<Item = T>, T: std::fmt::Debug>(grid: G) {
  eprintln!("{}", fmt_grid(grid));
}

pub fn fmt_grid<G: IntoIterator<Item = H>, H: IntoIterator<Item = T>, T: std::fmt::Debug>(grid: G) -> String {
  let mut width = 0;
  let formatted = grid.into_iter().map(|row| {
    row.into_iter().map(|x| {
      let s = format!("{:?}", x);
      width = width.max(s.len());
      s
    }).collect::<Vec<_>>()
  }).collect::<Vec<_>>();
  let mut result = String::new();
  for row in formatted {
    for x in row {
      let pad = width - x.len() + 1;
      result.push_str(&x);
      result.push_str(&" ".repeat(pad));
    }
    result.push('\n');
  }
  result
}