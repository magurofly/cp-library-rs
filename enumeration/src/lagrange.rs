use acl_modint::ModIntBase;
use super::*;
use number::*;

pub fn lagrange_polynomial<T: ModIntBase>(en: &Enumeration<T>, y: &[T], t: T) -> T {
  let n = y.len() - 1;
  let _t = t.val().as_usize();
  if _t <= n {
    return y[_t];
  }
  let mut ret = T::from(0);
  let mut dp = vec![T::from(1); n + 1];
  let mut pd = vec![T::from(1); n + 1];
  for i in 0 .. n {
    dp[i + 1] = dp[i] * T::from(_t - i);
  }
  for i in (1 ..= n).rev() {
    pd[i - 1] = pd[i] * T::from(_t - i);
  }
  for i in 0 ..= n {
    let tmp = y[i] * dp[i] * pd[i] * en.fact_inv(i) * en.fact_inv(n - i);
    if (n - i).is_odd() {
      ret -= tmp;
    } else {
      ret += tmp;
    }
  }
  ret
}

#[cfg(test)]
pub mod test {
  use acl_modint::*;
  use super::*;

  fn conv<T, U: From<T>>(from: Vec<T>) -> Vec<U> {
    from.into_iter().map(U::from).collect()
  }

  #[test]
  fn test_lagrange() {
    type M = ModInt1000000007;
    let en = Enumeration::<M>::new();
    assert_eq!(M::from(13), lagrange_polynomial(&en, &conv(vec![1, 3, 7]), M::from(3)));
    assert_eq!(M::from(999984471), lagrange_polynomial(&en, &conv(vec![4, 16, 106, 484, 1624, 4384]), M::from(1000000000)));
  }
}