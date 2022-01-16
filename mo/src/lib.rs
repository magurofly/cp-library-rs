pub trait Mo<T: Default> {
  fn add(&mut self, _index: usize) {
    unimplemented!();
  }

  fn remove(&mut self, _index: usize) {
    unimplemented!();
  }

  fn add_left(&mut self, index: usize) {
    self.add(index);
  }

  fn add_right(&mut self, index: usize) {
    self.add(index);
  }

  fn remove_left(&mut self, index: usize) {
    self.remove(index);
  }

  fn remove_right(&mut self, index: usize) {
    self.remove(index);
  }

  fn query(&self) -> T;

  fn mo(&mut self, n: usize, queries: &[(usize, usize)]) -> Vec<T> {
    let q = queries.len();
    let width = (n as f64).sqrt() as usize;
    let mut left = Vec::with_capacity(q);
    let mut right = Vec::with_capacity(q);
    for &(l, r) in queries {
      assert!(l <= r && r <= n);
      left.push(l);
      right.push(r);
    }
    let mut order = (0 .. q).collect::<Vec<_>>();
    order.sort_by(|&i, &j| {
      if left[i] / width == left[j] / width {
        right[i].cmp(&(right[j]))
      } else {
        left[i].cmp(&(left[j]))
      }
    });

    let mut ans = Vec::new();
    let mut l = 0;
    let mut r = 0;
    ans.resize_with(q, T::default);
    for i in order {
      while l > left[i] {
        l -= 1;
        self.add_left(l);
      }
      while r < right[i] {
        self.add_right(r);
        r += 1;
      }
      while l < left[i] {
        self.remove_left(l);
        l += 1;
      }
      while r > right[i] {
        r -= 1;
        self.remove_right(r);
      }
      ans[i] = self.query();
    }
    ans
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn range_set_query() {
    use std::collections::*;

    struct RSQ {
      set: HashMap<usize, usize>,
      colors: Vec<usize>,
    }
    impl Mo<usize> for RSQ {
      fn add(&mut self, index: usize) {
        *self.set.entry(self.colors[index]).or_insert(0) += 1;
      }
      fn remove(&mut self, index: usize) {
        let c = self.colors[index];
        if let Some(count) = self.set.get_mut(&c) {
          *count -= 1;
          if *count == 0 {
            self.set.remove(&c);
          }
        }
      }
      fn query(&self) -> usize {
        self.set.len()
      }
    }

    let mut rsq = RSQ {
      set: HashMap::new(),
      colors: vec![0, 1, 0, 2],
    };
    let ans = rsq.mo(4, &[(0, 3), (1, 4), (2, 3)]);
    assert_eq!(ans, &[2, 3, 1]);
  }
}