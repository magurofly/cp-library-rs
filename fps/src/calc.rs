use super::*;
use std::ops::*;

impl<T: Clone + From<usize>, C> FPS<T, C> where Self: Clone {
  pub fn diff(&self) -> Self where T: std::ops::Mul<Output = T> {
    let n = self.len();
    let mut ret = Self::new();
    ret.deref_mut().resize(n.saturating_sub(1), T::from(0));
    for i in 1 .. n {
      ret[i - 1] = self[i].clone() * T::from(i);
    }
    ret
  }

  pub fn integral(&self) -> Self where T: std::ops::Div<Output = T> {
    let n = self.len();
    let mut ret = Self::new();
    ret.deref_mut().resize(n + 1, T::from(0));
    for i in 0 .. n {
      ret[i + 1] = self[i].clone() / T::from(i + 1);
    }
    ret
  }
}