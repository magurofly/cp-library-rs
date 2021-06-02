fn subset_sum_min_count(A: &Vec<usize>, INF: usize) -> Vec<Option<usize>> {
  let sum = A.iter().sum::<usize>();
  let mut dp = vec![INF; sum];
  for &x in A {
    for y in x ..= sum {
      let c = dp[y - x] + 1;
      if dp[y] > c {
        dp[y] = c;
      }
    }
  }
  dp
}
