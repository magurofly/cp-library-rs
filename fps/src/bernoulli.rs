use super::*;
use enumeration::Enumeration;
use acl_modint::*;
use fft::*;

pub fn bernoulli<T: ModIntBase, C: Clone + Convolution<T>>(n: usize) -> Vec<T> {
  let mut x = FPS::<T, C>::with_deg(n + 1);
  let f = Enumeration::<T>::new();
  //TODO: replace with thread-local Enumeration struct
  for i in 0 ..= n {
    x[i] = f.fact_inv(i + 1);
  }

  let mut y = x.inv();
  for i in 0 ..= n {
    y[i] *= f.fact(i);
  }

  let y: Vec<_> = y.into();
  y[..= n].to_vec()
}

#[cfg(test)]
pub mod test {
    use acl_modint::*;
    use super::*;

  #[test]
  fn test_bernoulli() {
    let b = bernoulli::<ModInt998244353, ConvolutionStatic<Mod998244353>>(10);
    let b = b.into_iter().map(|x| x.val()).collect::<Vec<_>>();
    assert_eq!(b, vec![1, 499122176, 166374059, 0, 565671800, 0, 308980395, 0, 565671800, 0, 892369952]);
  }
}