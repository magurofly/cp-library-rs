use super::*;

pub struct MatGraph<E> {
  m: usize,
  mat: Vec<Vec<Option<E>>>,
}

impl<E: Clone> MatGraph<E> {
  pub fn new(n: usize) -> Self {
    Self::new_graph(n)
  }

  pub fn new_from<T>(mat: &Vec<Vec<T>>, mut f: impl FnMut(&T) -> Option<E>) -> Self {
    let mut this = Self::new(mat.len());
    for (i, row) in mat.iter().enumerate() {
      for (j, x) in row.iter().enumerate() {
        this.mat[i][j] = (f)(x);
      }
    }
    this
  }
}

impl<E: Clone> Graph<E> for MatGraph<E> {
  type Edge = (usize, E);

  fn n(&self) -> usize { self.mat.len() }
  
  fn m(&self) -> usize { self.m }

  /// O(1)
  fn edge_weight(&self, from: usize, to: usize) -> Option<&E> { self.mat[from][to].as_ref() }

  /// O(V)
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
    for v in 0 .. self.mat[from].len() {
      if let Some(weight) = self.mat[from][v].as_ref() {
        (f)(&(v, weight.clone()));
      }
    }
  }
}

impl<E: Clone> GraphMut<E> for MatGraph<E> {
  fn new_graph(n: usize) -> Self {
    Self { m: 0, mat: vec![vec![None; n]; n] }
  }

  fn add_arc(&mut self, from: usize, to: usize, weight: E) {
    self.mat[from][to] = Some(weight);
  }

  /// O(1)
  fn edge_weight_mut(&mut self, from: usize, to: usize) -> Option<&mut E> { self.mat[from][to].as_mut() }

  /// O(V)
  fn clear_edges(&mut self, from: usize) {
    for to in 0 .. self.n() {
      self.mat[from][to] = None;
    }
  }
}

impl<T> From<Vec<Vec<Option<T>>>> for MatGraph<T> {
  fn from(mat: Vec<Vec<Option<T>>>) -> Self {
    let mut m = 0;
    for row in mat.iter() {
      m += row.iter().filter(|x| x.is_some()).count();
    }
    Self { m, mat }
  }
}