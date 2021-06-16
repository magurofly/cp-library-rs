pub struct UnionFind(Vec<isize>, usize);
impl UnionFind {
  fn new(n: usize) -> Self { Self(vec![-1; n], n) }
  fn leader(&mut self, mut i: usize) -> usize { let k = self.0[i]; if k >= 0 { let j = self.leader(k as usize); self.0[i] = j as isize; i = j; }; i }
  fn merge(&mut self, mut i: usize, mut j: usize) -> bool { i = self.leader(i); j = self.leader(j); if i == j { return false }; if self.0[i] > self.0[j] { std::mem::swap(&mut i, &mut j) }; self.0[i] += self.0[j]; self.0[j] = i as isize; true }
  fn same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
  fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); -self.0[i] as usize }
  fn groups(&mut self) -> Vec<Vec<usize>> { let mut gs = vec![vec![]; self.1]; for i in 0 .. self.1 { gs[self.leader(i)].push(i) }; gs.into_iter().filter(|g| g.len() > 0 ).collect::<Vec<_>>() }
}
