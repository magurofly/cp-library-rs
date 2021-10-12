use super::*;
use std::cell::RefCell;

pub struct UnionFind {
  p: RefCell<Vec<isize>>,
  e: Vec<(usize, usize)>,
}

impl UnionFind {
  pub fn new(n: usize) -> Self {
    Self {
      p: RefCell::new(vec![-1; n]),
      e: vec![],
    }
  }

  pub fn leader(&self, mut i: usize) -> usize {
    let k = self.p.borrow()[i];
    if k >= 0 {
      let j = self.leader(k as usize);
      self.p.borrow_mut()[i] = j as isize;
      i = j
    }
    i
  }

  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool {
    i = self.leader(i);
    j = self.leader(j);
    if i == j {
      return false;
    }
    if -self.p.borrow()[i] < -self.p.borrow()[j] {
      std::mem::swap(&mut i, &mut j);
    }
    let s = self.p.borrow()[j];
    self.p.borrow_mut()[i] += s;
    self.p.borrow_mut()[j] = i as isize;
    true
  }

  pub fn is_same(&self, i: usize, j: usize) -> bool {
    self.leader(i) == self.leader(j)
  }

  pub fn size(&self, i: usize) -> usize {
    -self.p.borrow()[self.leader(i)] as usize
  }
}

impl Graph<()> for UnionFind {
  type Edge = usize;

  fn n(&self) -> usize { self.p.borrow().len() }
  fn m(&self) -> usize { self.e.len() }

  /// 計算量: O(M)
  fn edge_weight(&self, from: usize, to: usize) -> Option<&()> {
    self.e.iter().find(|&&(u, v)| (u == from && v == to) || (v == from && u == to) ).map(|_| &())
  }

  /// O(M)
  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&usize)) {
    for &(u, v) in &self.e {
      if u == from {
        (f)(&v);
      } else if v == from {
        (f)(&u);
      }
    }
  }

  fn connected_components(&self) -> Vec<Vec<usize>> {
    let mut components = vec![vec![]; self.n()];
    for i in 0 .. self.n() {
      components[self.leader(i)].push(i);
    }
    components.retain(|component| component.len() != 0);
    components
  }
}


impl GraphMut<()> for UnionFind {
  fn new_graph(n: usize) -> Self {
    Self::new(n)
  }

  fn add_arc(&mut self, from: usize, to: usize, _: ()) {
    self.merge(from, to);
  }

  fn add_edge(&mut self, from: usize, to: usize, _: ()) {
    self.merge(from, to);
  }

  fn connect(&mut self, from: usize, to: usize) {
    self.merge(from, to);
  }

  /// O(1)
  fn edge_weight_mut(&mut self, _: usize, _: usize) -> Option<&mut ()> { 
    unimplemented!()
  }

  /// O(V)
  fn clear_edges(&mut self, _: usize) {
    unimplemented!()
  }
}

#[cfg(test)]
pub mod tests {
    use super::*;

  #[test]
  fn atc001_a() {
    let mut uf = UnionFind::new(8);
    uf.connect(1, 2);
    uf.connect(3, 2);
    assert!(uf.is_connected(1, 3));
    assert!(!uf.is_connected(1, 4));
    uf.connect(2, 4);
    assert!(uf.is_connected(4, 1));
    uf.connect(4, 2);
    uf.connect(0, 0);
    assert!(uf.is_connected(0, 0));
  }
}