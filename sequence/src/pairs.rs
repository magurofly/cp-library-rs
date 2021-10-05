use std::{convert::TryFrom, fmt::Debug, ops::*};

pub fn sum_pair_mul<T: Copy + Default + Add<Output = T> + Mul<Output = T>>(xs: &[T]) -> T {
  xs.iter().rev().fold(
    (T::default(), T::default()),
    |(ans, sum), &x| (ans + sum * x, sum + x)
  ).0
}

pub fn sum_pair_affine<T: Copy + Add<Output = T> + Mul<Output = T> + TryFrom<usize>>(xs: &[T], aa: T, bb: T, ab: T, a: T, b: T, c: T) -> T where T::Error: Debug {
  let n = xs.len();
  let mut ans = cast(0);
  let mut sum = cast(0);
  for i in (0 .. n).rev() {
    ans = ans + (aa * cast(n - i - 1) + bb * cast(i)) * xs[i] * xs[i] + cast::<_, T>(n - i - 1) * (a * xs[i] + c) + (ab * xs[i] + b) * sum;
    sum = sum + xs[i];
  }
  ans
}

fn cast<T, U: TryFrom<T>>(x: T) -> U where U::Error: Debug {
  U::try_from(x).unwrap()
}

#[cfg(test)]
pub mod test {
  use super::*;
  // use acl_modint::*;

  const INT_SEQS: [&[i64]; 6] = [
    &[1, 2, 3, 4, 5],
    &[1, 0, -4, 1, 2],
    &[],
    &[0],
    &[1],
    &[-1]
  ];

  #[test]
  fn test_sum_mul() {
    for xs in &INT_SEQS {
      let a = (0 .. xs.len()).flat_map(|i| (i + 1 .. xs.len()).map(|j| xs[i] * xs[j]).collect::<Vec<_>>()).sum::<i64>();
      let b = sum_pair_mul(xs);
      assert_eq!(a, b);
    }
  }

  #[test]
  fn abc194_c() {
    let testcases: Vec<(Vec<i64>, i64)> = vec![
      (vec![2, 8, 4], 56),
      (vec![-5, 8, 9, -4, -3], 950),
    ];
    for (xs, y) in testcases {
      assert_eq!(y, sum_pair_affine(&xs, 1, 1, -2, 0, 0, 0));
    }
  }
}