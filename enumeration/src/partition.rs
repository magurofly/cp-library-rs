use number::*;

/// 分割数テーブルを作成する O(nk)
pub fn partition_table<N: IntLike>(n: usize, k: usize) -> Vec<Vec<N>> {
  let mut dp = vec![vec![N::get0(); k + 1]; n + 1];
  dp[0][0] = N::get1();

  for i in 0 ..= n {
    for j in 1 ..= k {
      dp[i][j] = dp[i][j - 1];
      if i >= j {
        dp[i][j] = dp[i][j] + dp[i - j][j];
      }
    }
  }

  dp
}

pub struct PartitionTableMod<N> {
  dp: Vec<Vec<N>>,
  n: usize,
  k: usize,
  modulus: N,
}
impl<N: Int> PartitionTableMod<N> {
  pub fn empty(modulus: N) -> Self {
    Self {
      dp: vec![vec![N::get1()]],
      n: 0,
      k: 0,
      modulus,
    }
  }

  pub fn new(modulus: N, n: usize, k: usize) -> Self {
    let mut p = Self::empty(modulus);
    p.ensure(n, k);
    p
  }

  pub fn ensure(&mut self, n: usize, k: usize) {
    // expand right
    if self.k < k {
      for i in 0 ..= self.n {
        self.dp[i].resize(k + 1, N::get0());
        for j in self.k + 1 ..= k {
          self.dp[i][j] = self.dp[i][j - 1];
          if i >= j {
            self.dp[i][j] = (self.dp[i][j] + self.dp[i - j][j]) % self.modulus;
          }
        }
      }
      self.k = k;
    }

    // expand down
    if self.n < n {
      for i in self.n + 1 ..= n {
        let mut row = vec![N::get0(); self.k + 1];
        for j in 1 ..= self.k {
          row[j] = row[j - 1];
          if i >= j {
            row[j] = (row[j] + self.dp[i - j][j]) % self.modulus;
          }
        }
        self.dp.push(row);
      }
      self.n = n;
    }
  }

  pub fn partition(&self, n: usize, k: usize) -> N {
    assert!(n <= self.n && k <= self.k);
    self.dp[n][k]
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_partition_table() {
    let mut p = PartitionTableMod::empty(1000000007i64);

    p.ensure(5, 3);
    assert_eq!(5, p.partition(5, 3));

    p.ensure(10, 5);
    assert_eq!(30, p.partition(10, 5));

    p.ensure(100, 100);
    assert_eq!(190569292, p.partition(100, 100));
  }
}