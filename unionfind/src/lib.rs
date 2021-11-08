pub struct UnionFind {
  components: Vec<UnionFindComponent>,
  len: usize,
}
impl UnionFind {
  /// 頂点数 $n$ の、辺のないグラフを作成する
  pub fn new(n: usize) -> Self {
    Self {
      components: (0 .. n).map(UnionFindComponent::new).collect::<Vec<_>>(),
      len: n,
    }
  }

  /// 頂点 $i$ を含む連結成分を代表する頂点を返す
  pub fn leader(&mut self, i: usize) -> usize {
    let mut k = self.components[i].root;
    if k != i {
      k = self.leader(k);
      self.components[i].root = k;
    }
    k
  }

  /// 頂点 $i$ と頂点 $j$ を無向辺で結ぶ
  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool {
    i = self.leader(i);
    j = self.leader(j);
    self.components[i].edges += 1;
    if i == j {
      return false;
    }
    self.len -= 1;
    if self.components[i].vertices < self.components[j].vertices {
      std::mem::swap(&mut i, &mut j);
    }
    self.components[i].edges += self.components[j].edges;
    self.components[i].vertices += self.components[j].vertices;
    self.components[j].root = i;
    true
  }

  /// 頂点 $i$ と頂点 $j$ が同じ連結成分に含まれるかを返す
  pub fn is_same(&mut self, mut i: usize, mut j: usize) -> bool {
    i = self.leader(i);
    j = self.leader(j);
    i == j
  }

  /// 連結成分の数を返す
  pub fn len(&self) -> usize {
    self.len
  }

  /// 頂点 $i$ を含む連結成分を返す
  pub fn component(&mut self, mut i: usize) -> &UnionFindComponent {
    i = self.leader(i);
    &self.components[i]
  }

  /// 頂点 $i$ を含む連結成分の頂点数を返す
  pub fn vertex_count(&mut self, i: usize) -> usize {
    self.component(i).vertices
  }

  /// 頂点 $i$ を含む連結成分の辺数を返す
  pub fn edge_count(&mut self, i: usize) -> usize {
    self.component(i).edges
  }

  /// それぞれの連結成分の頂点リストを返す
  pub fn groups(&mut self) -> Vec<Vec<usize>> {
    let n = self.components.len();
    let mut groups = vec![vec![]; n];
    for i in 0 .. n {
      groups[self.leader(i)].push(i);
    }
    groups.into_iter().filter(|group| group.len() > 0).collect::<Vec<_>>()
  }

  /// すべての連結成分を走査する
  pub fn each_component<F: FnMut(&UnionFindComponent)>(&mut self, mut f: F) {
    let n = self.components.len();
    let mut visited = vec![false; n];
    for mut i in 0 .. n {
      i = self.leader(i);
      if !visited[i] {
        visited[i] = true;
        (f)(&self.components[i]);
      }
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub struct UnionFindComponent {
  root: usize,
  vertices: usize,
  edges: usize,
}
impl UnionFindComponent {
  pub fn new(i: usize) -> Self {
    Self {
      root: i,
      vertices: 1,
      edges: 0,
    }
  }

  /// 連結成分を代表する頂点を返す
  pub fn root(&self) -> usize {
    self.root
  }

  /// 連結成分の頂点数を返す
  pub fn vertices(&self) -> usize {
    self.vertices
  }

  /// 連結成分の辺数を返す
  pub fn edges(&self) -> usize {
    self.edges
  }
}

use std::cell::RefCell;
use std::fmt;

#[derive(Clone)]
/// 要素を持つ UnionFind
pub struct UnionFindWithValue<T, F> {
  p: RefCell<Vec<(isize, Option<T>)>>,
  f: F,
  g: usize,
}

impl<T: fmt::Debug, F> fmt::Debug for UnionFindWithValue<T, F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("UnionFind { ")?;
    for i in 0 .. self.len() {
      f.write_fmt(format_args!("{} => ", i))?;
      let x = &self.p.borrow()[i];
      if x.0 < 0 {
        f.write_fmt(format_args!("({}, {:?})", -x.0, x.1.as_ref().unwrap()))?;
      } else {
        f.write_fmt(format_args!("[{}]", x.0))?;
      }
      if i != self.len() - 1 {
        f.write_str(", ")?;
      }
    }
    f.write_str(" }")?;
    Ok(())
  }
}

impl<T, F> UnionFindWithValue<T, F> {
  /// 要素が `x` の UnionFind を作成する
  /// `merge(x: T, y: T) -> T`: `x` と `y` をマージする
  pub fn new(values: Vec<T>, merge: F) -> Self {
    let g = values.len();
    let p = RefCell::new(values.into_iter().map(|x| (-1, Some(x))).collect());
    Self { p, f: merge, g }
  }

  pub fn len(&self) -> usize {
    self.p.borrow().len()
  }

  pub fn num_of_groups(&self) -> usize {
    self.g
  }

  pub fn leader(&self, i: usize) -> usize {
    let k = self.p.borrow()[i].0;
    if k >= 0 {
      let j = self.leader(k as usize);
      self.p.borrow_mut()[i].0 = j as isize;
      return j;
    }
    i
  }

  pub fn is_same(&self, i: usize, j: usize) -> bool {
    self.leader(i) == self.leader(j)
  }

  pub fn tap<U>(&self, i: usize, mut f: impl FnMut(&T) -> U) -> U {
    (f)(self.p.borrow()[self.leader(i)].1.as_ref().unwrap())
  }

  pub fn get(&self, i: usize) -> &T {
    let i = self.leader(i);
    unsafe { self.p.as_ptr().as_ref().unwrap()[i].1.as_ref().unwrap() }
  }

  pub fn get_mut(&mut self, i: usize) -> &mut T {
    let i = self.leader(i);
    self.p.get_mut()[i].1.as_mut().unwrap()
  }

  pub fn size(&self, i: usize) -> usize {
    -self.p.borrow()[self.leader(i)].0 as usize
  }

  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool where F: FnMut(T, T) -> T {
    i = self.leader(i);
    j = self.leader(j);
    if i == j {
      return false;
    }

    self.g -= 1;
    
    if -self._p(i) < -self._p(j) {
      std::mem::swap(&mut i, &mut j);
    }

    let size = self._p(i) + self._p(j);
    self.p.borrow_mut()[i].0 = size;
    self.p.borrow_mut()[j].0 = i as isize;
    let value1 = self.p.borrow_mut()[i].1.take().unwrap();
    let value2 = self.p.borrow_mut()[j].1.take().unwrap();
    let value = (self.f)(value1, value2);
    self.p.borrow_mut()[i].1 = Some(value);

    true
  }

  fn _p(&self, i: usize) -> isize {
    self.p.borrow()[i].0
  }

  pub fn groups(&mut self) -> Vec<Vec<usize>> {
    let mut s = vec![vec![]; self.len()];
    for i in 0 .. self.len() {
      s[self.leader(i)].push(i);
    }
    s.into_iter().filter(|g| g.len() > 0 ).collect::<Vec<_>>()
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn with_str() {
    let strs = vec!["a", "b", "c", "d", "e"].into_iter().map(|s| s.to_string()).collect();
    let mut uf = UnionFindWithValue::new(strs, |mut s: String, mut t: String| {
      if s.len() < t.len() {
        std::mem::swap(&mut s, &mut t);
      }
      s.push_str(t.as_str());
      s
    });

    assert_eq!(uf.num_of_groups(), 5);
    assert_eq!(uf.leader(0), 0);
    assert_eq!(uf.leader(1), 1);

    uf.merge(1, 2);
    assert_eq!(uf.num_of_groups(), 4);
    let s = uf.get(1).clone();
    assert!(s == "bc" || s == "cb");

    uf.merge(0, 2);
    assert_eq!(uf.num_of_groups(), 3);
    let t = uf.get(2).clone();
    assert!(t == "a".to_string() + s.as_str() || t == s + "a");
  }
}