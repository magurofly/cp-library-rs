pub struct UnionFind(Vec<isize>, usize);
impl UnionFind {
  pub fn new(n: usize) -> Self { Self(vec![-1; n], n) }
  pub fn leader(&mut self, mut i: usize) -> usize { let k = self.0[i]; if k >= 0 { let j = self.leader(k as usize); self.0[i] = j as isize; i = j; }; i }
  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool { i = self.leader(i); j = self.leader(j); i != j && { if self.0[i] > self.0[j] { let k = i; i = j; j = k; }; self.0[i] += self.0[j]; self.0[j] = i as isize; true } }
  pub fn same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
  pub fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); -self.0[i] as usize }
  pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut s = vec![vec![]; self.1]; for i in 0 .. self.1 { s[self.leader(i)].push(i) }; s.into_iter().filter(|g| g.len() > 0 ).collect::<Vec<_>>() }
}
