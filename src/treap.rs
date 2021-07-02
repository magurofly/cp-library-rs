pub mod treap {
  // Last Update: 2021-07-03 01:37
  // https://www.slideshare.net/iwiwi/2-12188757
  pub enum Treap<K, V, Op> {
    NIL,
    NODE {
      key: K,
      value: V,
      sum: V,
      op: Rc<Op>,
      priority: usize,
      len: usize,
      child: Box<[Treap<K, V, Op>; 2]>,
    }
  }

  use Treap::*;

  impl<K: Copy + Ord, V: Copy + Default, Op: Fn(V, V) -> V> Treap<K, V, Op> {
    pub fn new(key: K, value: V, op: Rc<Op>, priority: usize) -> Self {
      NODE { key, value, sum: value, op, priority, len: 1, child: Box::new([NIL, NIL]) }
    }

    pub fn is_node(&self) -> bool { !matches!(self, NIL) }
    pub fn len(&self) -> usize { match self { NIL => 0, NODE { len, .. } => *len } }
    pub fn value(&self) -> V { match self { NIL => V::default(), NODE { value, .. } => *value } }
    pub fn sum(&self) -> V { match self { NIL => V::default(), NODE { sum, .. } => *sum } }
    pub fn priority(&self) -> usize { match self { NIL => 0, NODE { priority, .. } => *priority } }
    pub fn map_child(self, which: usize, f: impl FnOnce(Self) -> Self) -> Self {
      if let NODE { key, value, sum, op, priority, len, mut child } = self {
        child[which] = (f)(take(&mut child[which]));
        return NODE { key, value, sum, op, priority, len, child };
      }
      NIL
    }

    pub fn update(self) -> Self {
      match self {
        NIL => NIL,
        NODE { key, value, sum, op, priority, len, child } => {
          let len = child[0].len() + child[1].len() + 1;
          let sum = (op)((op)(child[0].sum(), child[1].sum()), value);
          NODE { key, value, sum, op, priority, len, child }
        }
      }
    }

    pub fn merge(self, other: Self) -> Self {
      if self.is_node() {
        if other.is_node() {
          if self.priority() > other.priority() {
            self.map_child(1, |right| right.merge(other) ).update()
          } else {
            other.map_child(0, |left| self.merge(left) ).update()
          }
        } else {
          self
        }
      } else {
        other
      }
    }
    
    pub fn split(self, k: usize) -> (Self, Self) {
      match self {
        NIL => (NIL, NIL),
        NODE { key, value, sum, op, priority, len, mut child } => {
          if k <= child[0].len() {
            let (s, t) = take(&mut child[0]).split(k);
            child[0] = t;
            (s, NODE { key, value, sum, op, priority, len, child }.update())
          } else {
            let (s, t) = take(&mut child[1]).split(k - child[0].len() - 1);
            child[1] = s;
            (NODE { key, value, sum, op, priority, len, child }.update(), t)
          }
        },
      }
    }

    //TODO: insert, erase実装
  }

  impl<K, V, Op> Default for Treap<K, V, Op> {
    fn default() -> Self { Treap::NIL }
  }

  use std::rc::Rc;
  use std::mem::take;
}
