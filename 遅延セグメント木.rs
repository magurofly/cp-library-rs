use segtree::*;
pub mod segtree {
  pub trait SegtreeHelper {
    /// 要素の型
    type S;
    /// 要素の二項演算
    fn op(x: &Self::S, y: &Self::S) -> Self::S { Self::e() }
    /// 要素の単位元
    fn e() -> Self::S;
    /// 作用の型（使わない場合は `()` とする）
    type F;
    /// 要素に作用させる
    fn map(f: &Self::F, x: &Self::S) -> Self::S { Self::e() }
    /// 作用の単位元（使わない場合は `()` とする）
    fn id() -> Self::F;
    /// 作用の合成
    fn compose(f: &Self::F, g: &Self::F) -> Self::F { Self::id() }
    /// 再計算が必要か
    fn is_failed(x: &Self::S) -> bool { false }
  }

  pub struct Segtree<H: SegtreeHelper> {
    len: usize,
    size: usize,
    data: UnsafeCell<Vec<H::S>>,
    lazy: UnsafeCell<Vec<H::F>>,
  }

  impl<H: SegtreeHelper> Segtree {
    pub fn new(len: usize) -> Self {
      assert!(len > 0);
      let size = len.next_power_of_two();
      Self {
        len, size,
        data: UnsafeCell::new((0 .. size * 2).map(H::e).collect()),
        lazy: UnsafeCell::new((0 .. size * 2).map(H::id).collect()),
      }
    }

    fn update(&self, k: usize) {
      let z = H::op(&self.data()[k * 2], &self.data()[k * 2 + 1]);
      self.data_mut()[k] = z;
    }

    fn all_apply(&self, k: usize, f: &H::F) {
      let y = H::map(f, &self.data()[k]);
      self.data_mut()[k] = y;
      if k < self.size {
        let h = H::compose(f, &self.lazy()[k]);
        self.lazy_mut()[k] = h;
      }
    }

    fn push(&self, k: usize) {
      self.all_apply(2 * k, &self.lazy()[k]);
      self.all_apply(2 * k + 1, &self.lazy()[k]);
      self.lazy_mut()[k] = H::id();
    }

    fn data(&self) -> &Vec<H::S> { unsafe { &*self.data.get() } }
    fn lazy(&self) -> &Vec<H::F> { unsafe { &*self.lazy.get() } }
    fn data_mut(&self) -> &mut Vec<H::S> { unsafe { &mut *self.data.get() } }
    fn lazy_mut(&self) -> &mut Vec<H::F> { unsafe { &mut *self.lazy.get() } }
  }

  impl<H: SegtreeHelper> From<Vec<H::S>> for Segtree {
    fn from(mut xs: Vec<H::S>) -> Self {
      let size = xs.len().next_power_of_two();
      let mut data = (0 .. size).map(H::e).collect();
      data.append(&mut xs);
      data.resize_with(size * 2, H::e);
      let this = Self {
        len, size,
        data: UnsafeCell::new(data),
        lazy: UnsafeCell::new((0 .. size * 2).map(H::id)),
      };
      for i in 1 .. size {
        this.update(i);
      }
      this
    }
  }

  use self::cell::*;
}