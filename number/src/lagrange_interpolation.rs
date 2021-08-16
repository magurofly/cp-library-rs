use super::*;
use acl_modint::*;

// pub fn lagrange_interpolation<M: ModIntBase>(x: &[M], y: &[M], t: M) -> M {
//   let k = x.len() - 1;
//   let mut ret = M::from(0);
//   for i in 0 ..= k {
//     let mut m = M::from(1);
//     let mut d = M::from(1);
//     for j in 0 ..= k {
//       if i != j {
//         m *= t - x[j];
//         d *= x[i] - x[j];
//       }
//     }
//     ret += y[i] * m / d;
//   }
//   ret
// }

pub fn lagrange_polynomial<N: ModIntBase>(y: &[N], t: N) -> N {
  let deg = y.len() - 1;
  let f = FactorialInvMod::new(deg as i64, N::modulus() as i64);
  if t.val() as usize <= deg {
    return y[t.val() as usize];
  }
  let mut ret = N::from(0);
  let mut dp = vec![N::from(1); deg + 1];
  let mut pd = vec![N::from(1); deg + 1];
  for i in 0 .. deg {
    dp[i + 1] = dp[i] * (t - N::from(i));
  }
  for i in (1 .. deg).rev() {
    pd[i - 1] = pd[i] * (t - N::from(i));
  }
  for i in 0 ..= deg {
    let tmp = y[i] * dp[i] * pd[i] * N::from(f.fact_inv(i)) * N::from(f.fact_inv(deg - i));
    if (deg - i).is_even() {
      ret -= tmp;
    } else {
      ret += tmp;
    }
  }
  ret
}

#[cfg(test)]
pub mod test {
  use super::*;
  
  fn m(n: u32) -> ModInt1000000007 {
    ModInt1000000007::from(n)
  }

  #[test]
  fn test_polynomial() {
    // assert_eq!(lagrange_polynomial(&[m(1), m(3), m(7)], m(3)), m(13))
  }
}