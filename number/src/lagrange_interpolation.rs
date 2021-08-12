use super::*;

pub fn lagrange_interpolation<N: Int>(y: &[N], x: N, m: N) -> N {
  let n = y.len() - 1;
  if x <= n.cast() {
    return y[x.as_usize()] % m;
  }
  let fact = FactorialInvMod::new(n.cast::<N>(), m);

  let mut ret = N::zero();
  let mut dp = vec![N::one(); n + 1];
  let mut pd = vec![N::one(); n + 1];
  for i in 0 .. n {
    dp[i + 1] = dp[i] * (x - i.cast());
  }
  for i in (1 ..= n).rev() {
    pd[i - 1] = pd[i] * (x - i.cast());
  }
  for i in 0 ..= n {
    let t = y[i] * dp[i] * pd[i] * fact.fact_inv(i) * fact.fact_inv(n - i);
    if (n - i).is_odd() {
      ret = ret - t;
    } else {
      ret = ret + t;
    }
  }

  ret
}