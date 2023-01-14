# BIT Multiset

## コード

```rust
use bit_multiset::BITMultiset;
pub mod bit_multiset {
  pub struct BITMultiset {
    n: usize,
    tree: Vec<usize>,
  }
  impl BITMultiset {
    pub fn new(n: usize) -> Self {
      assert!(n <= 100_000_000, "n={} は大きすぎます。値を圧縮してください。", n);
      Self {
        n,
        tree: vec![0; n + 1],
      }
    }
    pub fn count_all(&self) -> usize {
      self.count_lt(self.n)
    }
    pub fn count(&self, x: usize) -> usize {
      self.count_lt(x + 1) - self.count_lt(x)
    }
    pub fn count_le(&self, x: usize) -> usize {
      self.count_lt(x + 1)
    }
    pub fn count_lt(&self, x: usize) -> usize {
      self.sum(x)
    }
    pub fn count_gt(&self, x: usize) -> usize {
      self.count_all() - self.count_le(x)
    }
    pub fn count_ge(&self, x: usize) -> usize {
      self.count_all() - self.count_lt(x)
    }
    pub fn insert_multiple(&mut self, x: usize, count: usize) {
      self.add(x, count as isize)
    }
    pub fn insert(&mut self, x: usize) {
      self.add(x, 1)
    }
    pub fn remove_multiple(&mut self, x: usize, count: usize) -> usize {
      let actual = self.count(x).min(count);
      self.add(x, actual as isize);
      actual
    }
    pub fn remove(&mut self, x: usize) -> bool {
      self.remove_multiple(x, 1) == 1
    }
    pub fn remove_all(&mut self, x: usize) -> usize {
      self.remove_multiple(x, self.count_all())
    }
    pub fn kth_min(&self, k: usize) -> Option<usize> {
      if self.count_all() <= k {
        return None;
      }
      Some(self.lower_bound(k + 1))
    }
    pub fn kth_max(&self, k: usize) -> Option<usize> {
      if self.count_all() >= k + 1 {
        self.kth_min(self.count_all() - k - 1)
      } else {
        None
      }
    }
    pub fn min(&self) -> Option<usize> {
      self.kth_min(0)
    }
    pub fn max(&self) -> Option<usize> {
      self.kth_max(0)
    }
    pub fn gt_min(&self, x: usize) -> Option<usize> {
      self.kth_min(self.count_le(x))
    }
    pub fn lt_max(&self, x: usize) -> Option<usize> {
      self.kth_max(self.count_ge(x))
    }
    pub fn into_vec(self) -> Vec<usize> {
      let Self { n, mut tree } = self;
      for x in (1 .. n).rev() {
        tree[x + (1 << x.trailing_zeros())] -= tree[x];
      }
      tree
    }
    fn sum(&self, mut x: usize) -> usize {
      let mut sum = 0;
      while x > 0 {
        sum += self.tree[x];
        x -= 1 << x.trailing_zeros();
      }
      sum
    }
    fn add(&mut self, mut x: usize, count: isize) {
      x += 1;
      while x <= self.n {
        self.tree[x] = (self.tree[x] as isize + count) as usize;
        x += 1 << x.trailing_zeros();
      }
    }
    // 参考: http://hos.ac/slides/20140319_bit.pdf
    fn lower_bound(&self, mut count: usize) -> usize {
      let mut x = 0;
      let mut k = (self.n + 1).next_power_of_two();
      while k > 0 {
        if x + k < self.n && self.tree[x + k] < count {
          count -= self.tree[x + k];
          x += k;
        }
        k >>= 1;
      }
      x
    }
  }
  impl std::iter::FromIterator<usize> for BITMultiset {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
      let mut tree = Some(0).into_iter().chain(iter).collect::<Vec<_>>();
      let n = tree.len() - 1;
      for x in 1 .. n {
        tree[x + (1 << x.trailing_zeros())] += tree[x];
      }
      Self { n, tree }
    }
  }
  impl std::iter::IntoIterator for BITMultiset {
    type IntoIter = std::iter::Filter<std::iter::Enumerate<std::vec::IntoIter<usize>>, fn(&(usize, usize)) -> bool>;
    type Item = (usize, usize);
    fn into_iter(self) -> Self::IntoIter {
      fn nonzero(&(_, count): &(usize, usize)) -> bool { count != 0 }
      self.into_vec().into_iter().enumerate().filter(nonzero)
    }
  }
  impl std::fmt::Debug for BITMultiset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_str("BITMultiset {")?;
      let mut separator = "";
      for x in 0 .. self.n {
        let count = self.count(x);
        if count != 0 {
          f.write_fmt(format_args!("{}{}: {}", separator, x, count))?;
          separator = ", ";
        }
      }
      f.write_str("}")?;
      Ok(())
    }
  }
}
```
