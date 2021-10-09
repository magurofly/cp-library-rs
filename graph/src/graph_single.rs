use super::*;

impl Graph<()> for Vec<usize> {
  type Edge = usize;

  fn n(&self) -> usize { self.len() }
  fn m(&self) -> usize { self.len() }
  
  fn edge_weight(&self, from: usize, to: usize) -> Option<&()> {
    if self[from] == to {
      Some(&())
    } else {
      None
    }
  }

  fn each_edge(&self, mut f: impl FnMut(EdgeInfo<()>)) {
    for i in 0 .. self.n() {
      (f)(EdgeInfo::new(i, self[i], &()));
    }
  }

  fn each_edge_from(&self, from: usize, mut f: impl FnMut(&Self::Edge)) {
    (f)(&self[from]);
  }
}