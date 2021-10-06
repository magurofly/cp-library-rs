use std::{convert::TryFrom, fmt::Debug, ops::*};

/// 非順序対 (x, y) に対して x の総和を返す
/// 時間計算量 O(N)
pub fn sum_pair_first<T: Copy + Default + Add<Output = T>>(xs: &[T]) -> T {
  xs.iter().fold(
    (T::default(), T::default()),
  |(ans, sum), &x| (ans + sum, sum + x)
  ).0
}

/// 非順序対 (x, y) に対して y の総和を返す
/// 時間計算量 O(N)
pub fn sum_pair_second<T: Copy + Default + Add<Output = T>>(xs: &[T]) -> T {
  xs.iter().rev().fold(
    (T::default(), T::default()),
  |(ans, sum), &x| (ans + sum, sum + x)
  ).0
}

/// 非順序対 (x, y) に対して x + y の総和を返す
/// 時間計算量 O(N)
pub fn sum_pair_sum<T: Copy + Default + Add<Output = T>>(xs: &[T]) -> T {
  sum_pair_first(xs) + sum_pair_second(xs)
}

/// 非順序対 (x, y) に対して x - y の総和を返す
/// 時間計算量 O(N)
pub fn sum_pair_diff<T: Copy + Default + Add<Output = T> + Sub<Output = T>>(xs: &[T]) -> T {
  sum_pair_first(xs) - sum_pair_second(xs)
}

/// 非順序対 (x, y) に対して abs(x - y) の総和を返す
/// 時間計算量 O(N)
pub fn sum_pair_abs_diff<T: Copy + Default + Add<Output = T> + Sub<Output = T> + Ord>(xs: &[T]) -> T {
  let mut ys = xs.to_vec();
  ys.sort();
  sum_pair_second(&ys) - sum_pair_first(&ys)
}

/// 非順序対 (x, y) のうち min(x, y) の総和を返す
pub fn sum_pair_min<T: Copy + Default + Add<Output = T> + Ord>(xs: &[T]) -> T {
  let mut ys = xs.to_vec();
  ys.sort();
  sum_pair_first(&ys)
}

/// 非順序対 (x, y) のうち max(x, y) の総和を返す
pub fn sum_pair_max<T: Copy + Default + Add<Output = T> + Ord>(xs: &[T]) -> T {
  let mut ys = xs.to_vec();
  ys.sort();
  sum_pair_second(&ys)
}

/// 非順序対 (x, y) に対して x * y の総和を返す
pub fn sum_pair_mul<T: Copy + Default + Add<Output = T> + Mul<Output = T>>(xs: &[T]) -> T {
  xs.iter().rev().fold(
    (T::default(), T::default()),
    |(ans, sum), &x| (ans + sum * x, sum + x)
  ).0
}

/// 非順序対 (x, y) に対して aa * x * x + bb * y * y + ab * x * y + a * x + b * y + c の総和を返す
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

  fn pairs<T: Copy>(xs: &[T]) -> Vec<(T, T)> {
    (0 .. xs.len()).flat_map(|i| (i + 1 .. xs.len()).map(move |j| (xs[i], xs[j]))).collect()
  }

  #[test]
  fn test_sum_first() {
    for xs in &INT_SEQS {
      let a = pairs(xs).into_iter().map(|(x, _)| x).sum::<i64>();
      let b = sum_pair_first(xs);
      assert_eq!(a, b, "{:?}", xs);
    }
  }

  #[test]
  fn test_sum_second() {
    for xs in &INT_SEQS {
      let a = pairs(xs).into_iter().map(|(_, y)| y).sum::<i64>();
      let b = sum_pair_second(xs);
      assert_eq!(a, b, "{:?}", xs);
    }
  }

  #[test]
  fn test_sum_min() {
    for xs in &INT_SEQS {
      let a = pairs(xs).into_iter().map(|(x, y)| std::cmp::min(x, y)).sum::<i64>();
      let b = sum_pair_min(xs);
      assert_eq!(a, b, "{:?}", xs);
    }
  }

  #[test]
  fn test_sum_max() {
    for xs in &INT_SEQS {
      let a = pairs(xs).into_iter().map(|(x, y)| std::cmp::max(x, y)).sum::<i64>();
      let b = sum_pair_max(xs);
      assert_eq!(a, b, "{:?}", xs);
    }
  }

  #[test]
  fn test_sum_mul() {
    for xs in &INT_SEQS {
      let a = pairs(xs).into_iter().map(|(x, y)| x * y).sum::<i64>();
      let b = sum_pair_mul(xs);
      assert_eq!(a, b, "{:?}", xs);
    }
  }

  #[test]
  fn abc194_c() {
    let testcases: Vec<(Vec<i64>, i64)> = vec![
      (vec![2, 8, 4], 56),
      (vec![-5, 8, 9, -4, -3], 950),
    ];
    for (xs, y) in testcases {
      assert_eq!(y, sum_pair_affine(&xs, 1, 1, -2, 0, 0, 0), "{:?}", &xs);
    }
  }

  #[test]
  fn abc186_d() {
    let testcases: Vec<(Vec<i64>, i64)> = vec![
      (vec![5, 1, 2], 8),
      (vec![31, 41, 59, 26, 53], 176),
    ];
    for (xs, y) in testcases {
      assert_eq!(y, sum_pair_abs_diff(&xs), "{:?}", &xs);
    }
  }
}