pub struct UnionFind(Vec<isize>, usize);
impl UnionFind {
  pub fn new(n: usize) -> Self { Self(vec![-1; n], n) }
  pub fn leader(&mut self, mut i: usize) -> usize { let k = self.0[i]; if k >= 0 { let j = self.leader(k as usize); self.0[i] = j as isize; i = j; }; i }
  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool { i = self.leader(i); j = self.leader(j); i != j && { if self.0[i] > self.0[j] { let k = i; i = j; j = k; }; self.0[i] += self.0[j]; self.0[j] = i as isize; true } }
  pub fn same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
  pub fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); -self.0[i] as usize }
  pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut s = vec![vec![]; self.1]; for i in 0 .. self.1 { s[self.leader(i)].push(i) }; s.into_iter().filter(|g| g.len() > 0 ).collect::<Vec<_>>() }
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