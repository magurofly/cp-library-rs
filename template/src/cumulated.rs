use super::*;
use std::ops::RangeBounds;

#[derive(Debug, Clone)]
pub struct Cumulated<T, Op, Inv> {
  cumulated: Vec<T>,
  op: Op,
  inv: Inv
}

impl<T: Copy, Op: Fn(T, T) -> T, Inv: Fn(T) -> T> Cumulated<T, Op, Inv> {
  pub fn new(list: impl IntoIterator<Item = T>, init: T, op: Op, inv: Inv) -> Self {
    let mut cumulated = vec![init];
    let mut last = init;
    for x in list {
      last = (op)(last, x);
      cumulated.push(last);
    }
    Self { cumulated, op, inv }
  }

  pub fn fold(&self, range: impl RangeBounds<usize>) -> T {
    (self.op)(self.cumulated[range.end_open_or(self.cumulated.len() - 1)], (self.inv)(self.cumulated[range.start_open_or(0)]))
  }
}