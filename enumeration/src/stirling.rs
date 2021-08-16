use number::*;

pub fn stirling2_table_mod<N: Int>(n: usize, m: N) -> Vec<Vec<N>> {
  let mut dp = vec![vec![N::one()]];
  dp.reserve(n);
  for i in 1 ..= n {
    let mut row = vec![N::zero()];
    row.reserve(i);
    for j in 1 ..= i {
      row.push((dp[i - 1][j - 1] + j.cast::<N>() * dp[i - 1][j]) % m);
    }
    dp.push(row);
  }
  dp
}