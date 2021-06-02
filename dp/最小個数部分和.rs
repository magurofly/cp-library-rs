// 01-最小個数部分和を計算する
// 計算量: 時間 O(|A|ΣA), 空間 O(ΣA)
// @param A 非負整数の集合
// @param INF 無限とみなす値
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
