use std::ops::{AddAssign, SubAssign};

pub fn zeta_transform_inplace<T: Clone + AddAssign>(xs: &mut [T]) {
  let n = xs.len().next_power_of_two();
  assert!(xs.len() == n, "len must be power of two");
  let m = n.trailing_zeros();
  for j in 0 .. m {
    let bit = 1 << j;
    for i in 0 .. n {
      if i & bit == 0 {
        xs[i] += xs[i | bit].clone();
      }
    }
  }
}

pub fn zeta_transform<T: Clone + AddAssign>(xs: &[T]) -> Vec<T> {
  let mut xs = xs.to_vec();
  zeta_transform_inplace(&mut xs);
  xs
}

pub fn moebius_transform_inplace<T: Clone + SubAssign>(xs: &mut [T]) {
  let n = xs.len().next_power_of_two();
  assert!(xs.len() == n, "len must be power of two");
  let m = n.trailing_zeros();
  for j in 0 .. m {
    let bit = 1 << j;
    for i in 0 .. n {
      if i & bit == 0 {
        xs[i] -= xs[i | bit].clone();
      }
    }
  }
}

pub fn moebius_transform<T: Clone + SubAssign>(xs: &[T]) -> Vec<T> {
  let mut xs = xs.to_vec();
  moebius_transform_inplace(&mut xs);
  xs
}

#[cfg(test)]
pub mod test {
  use super::*;

  const CASES: [&[i64]; 1] = [
    &[3, 1, 4, 1, 5, 9, 2, 6]
  ];

  #[test]
  fn zeta_moebius() {
    for &xs in &CASES {
      let ys = zeta_transform(&xs);
      let zs = moebius_transform(&ys);
      assert_eq!(xs, &zs[..]);
    }
  }

  #[test]
  fn moebius_zeta() {
    for &xs in &CASES {
      let ys = moebius_transform(&xs);
      let zs = zeta_transform(&ys);
      assert_eq!(xs, &zs[..]);
    }
  }
}