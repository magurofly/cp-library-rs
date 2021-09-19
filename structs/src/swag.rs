#[derive(Debug, Clone)]
pub struct SWAG<T, F> {
  op: F,
  front: Vec<(T, T)>,
  back: Vec<T>,
  back_sum: Option<T>,
}

impl<T: Clone, F: Fn(T, T) -> T> SWAG<T, F> {
  pub fn new(op: F) -> Self {
    Self {
      op,
      front: vec![],
      back: vec![],
      back_sum: None,
    }
  }

  /// 後端に要素を追加する
  pub fn push(&mut self, x: T) {
    if self.back.is_empty() {
      self.back_sum = Some(x.clone());
      self.back.push(x);
    } else {
      self.back_sum = Some((self.op)(self.back_sum.take().unwrap(), x.clone()));
      self.back.push(x);
    }
  }

  /// 先頭の要素を取り出す
  pub fn pop(&mut self) -> Option<(T, T)> {
    if self.front.is_empty() {
      while let Some(x) = self.back.pop() {
        if self.front.is_empty() {
          self.front.push((x.clone(), x));
        } else {
          let sum = (self.op)(self.front.last()?.1.clone(), x.clone());
          self.front.push((x, sum));
        }
      }
      self.back_sum = None;
    }
    self.front.pop()
  }

  /// 現在入っている要素の積
  pub fn fold(&self) -> Option<T> where T: std::fmt::Debug {
    eprintln!("front={:?}, back={:?}", &self.front, &self.back);
    if let Some((_, f)) = self.front.last() {
      if let Some(b) = self.back_sum.clone() {
        Some((self.op)(f.clone(), b.clone()))
      } else {
        Some(f.clone())
      }
    } else {
      self.back_sum.clone()
    }
  }
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn slide_min() {
    let mut swag = SWAG::new(|x: i32, y: i32| x.min(y));
    swag.push(1);
    swag.push(7);
    swag.push(7);
    assert_eq!(swag.fold(), Some(1));
    swag.push(4);
    swag.pop();
    assert_eq!(swag.fold(), Some(4));
    swag.pop();
    swag.push(8);
    assert_eq!(swag.fold(), Some(4));
    swag.push(1);
    swag.pop();
    assert_eq!(swag.fold(), Some(1));
    swag.pop();
    swag.push(6);
    assert_eq!(swag.fold(), Some(1));
  }
}