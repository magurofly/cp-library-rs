use std::ops::*;

pub fn sum_of_pair_mul<T: Default + Clone + AddAssign + Mul<Output = T>>(xs: &[T]) -> T {
  let mut ans = T::default();
  let mut sum = T::default();
  for i in (0 .. xs.len().saturating_sub(1)).rev() {
    sum += xs[i + 1].clone();
    ans += sum.clone() * xs[i].clone();
  }
  ans
}

#[cfg(test)]
pub mod test {
  use super::*;

  const INT_SEQS: [&[i64]; 6] = [
    &[1, 2, 3, 4, 5],
    &[1, 0, -4, 1, 2],
    &[],
    &[0],
    &[1],
    &[-1]
  ];

  #[test]
  fn test_sum_of_pair_mul() {
    for xs in &INT_SEQS {
      let a = (0 .. xs.len()).flat_map(|i| (i + 1 .. xs.len()).map(|j| xs[i] * xs[j]).collect::<Vec<_>>()).sum::<i64>();
      let b = sum_of_pair_mul(xs);
      assert_eq!(a, b);
    }
  }
}