use acl_modint::ModIntBase;
use fft::Convolution;
use super::*;

// verifyまだ

impl<T: ModIntBase, C: Clone + Convolution<T>> FPS<T, C> {
  /// df/dx = g(f) mod x^deg となる f を求める
  pub fn differential_equation(g: impl Fn(&Self, usize) -> Self, g_diff: impl Fn(&Self, usize) -> Self, f0: T, deg: usize) -> Self {
    let mut f = Self::from(vec![f0]);
    let mut i = 1;
    while i < deg {
      let r = (-(g_diff)(&f, i << 1)).integral().exp_at(i << 1);
      let mut h = ((g)(&f, i << 1) - (g_diff)(&f, i << 1) * &f) * &r;
      h.truncate(i << 1);
      h = h.integral();
      f = (h + f0) * r.inv_at(i << 1);
      f.truncate(i << 1);
      i <<= 1;
    }
    f.truncate(deg);
    f
  }
}