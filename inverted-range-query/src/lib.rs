use algebraic_structures::*;

pub struct InvertedRangeQuery<M: Monoid> {
  from_left: Vec<M::S>,
  from_right: Vec<M::S>,
}

impl<M: Monoid> InvertedRangeQuery<M> {
  pub fn prod(&self, lr: usize, rl: usize) -> M::S {
    M::operator(&self.from_left[lr], &self.from_right[rl])
  }

  pub fn without(&self, i: usize) -> M::S {
    self.prod(i, i + 1)
  }
}

impl<M: Monoid> From<&[M::S]> for InvertedRangeQuery<M> {
  fn from(slice: &[M::S]) -> Self {
    let mut from_left = vec![M::identity()];
    let mut from_right = vec![M::identity()];
    for i in 0 .. slice.len() {
      from_left.push(M::operator(&from_left[i], &slice[i]));
      from_right.push(M::operator(&slice[slice.len() - 1 - i], &from_left[i]));
    }
    from_right.reverse();
    InvertedRangeQuery {
      from_left,
      from_right,
    }
  }
}
