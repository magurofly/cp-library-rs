use acl_modint::*;

type T = ModInt998244353;

pub fn ntt(x: &[T]) -> Vec<T> {
  let bit = x.len().next_power_of_two().trailing_zeros() + 1;
  ntt_internal(x, bit, false)
}

pub fn ntt_inv(x: &[T]) -> Vec<T> {
  let bit = x.len().next_power_of_two().trailing_zeros() + 1;
  ntt_internal(x, bit, true)
}

/// https://judge.yosupo.jp/submission/63348
fn ntt_internal(x: &[T], bit: u32, inverse: bool) -> Vec<T> {
  let n = x.len();
  assert!(n == 1 << bit);
  assert!(bit <= 23);

  let mask1 = n - 1;

  let mut pv_zeta = T::from(3).pow(119 << 23 - bit);
  if !inverse {
    pv_zeta = pv_zeta.inv();
  }
  let mut zeta = vec![T::from(1); n];
  for i in 0 .. n {
    zeta[i] = zeta[i - 1] * pv_zeta;
  }
  let mut tmp = vec![];
  let mut x = x;
  for i in 0 .. bit {
    let mask2 = mask1 >> i + 1;
    tmp = (0 .. n).map(|j| {
      let lower = j & mask2;
      let upper = j ^ lower;
      let shifted = upper << 1 & mask1;
      x[shifted | lower] + zeta[upper] * x[shifted | mask2 + 1 | lower]
    }).collect::<Vec<_>>();
    x = &mut tmp;
  }
  tmp
}