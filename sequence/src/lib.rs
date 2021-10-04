use std::ops::*;

pub fn sum_of_pair_mul<T: Default + Clone + Add<Output = T> + Mul<Output = T>>(xs: &[T]) -> T {
  xs.iter().rev().fold(
    (T::default(), T::default()),
    |(ans, sum), x| (ans + sum.clone() * x.clone(), sum + x.clone())
  ).0
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