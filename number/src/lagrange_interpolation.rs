use super::*;

pub fn lagrange_interpolation_mod<N: Int>(y: &[N], x: N, m: N) -> N {
  let n = y.len() - 1;
  if x <= n.cast() {
    return y[x.as_usize()] % m;
  }
  let fact = FactorialInvMod::new(n.cast::<N>(), m);

  let mut dp = vec![N::one(); n + 1];
  let mut pd = vec![N::one(); n + 1];
  for i in 0 .. n {
    dp[i + 1] = dp[i] * (x - i.cast()) % m;
  }
  for i in (1 ..= n).rev() {
    pd[i - 1] = pd[i] * (x - i.cast()) % m;
  }
  let mut ret = N::zero();
  for i in 0 ..= n {
    let t = y[i] * dp[i] % m * pd[i] % m * fact.fact_inv(i) % m * fact.fact_inv(n - i) % m;
    if (n - i).is_odd() {
      ret = ret - t;
    } else {
      ret = ret + t;
    }
    ret = ret % m;
  }

  ret
}