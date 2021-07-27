pub mod binary_trie {
  use num_traits::*;

  fn mask<N: PrimInt>(x: N, y: N) -> N {
    if (x & y).is_zero() {
      N::zero()
    } else {
      N::one()
    }
  }

  #[derive(Debug, Clone, Default)]
  struct Node {
    zero: Option<Box<Node>>,
    one: Option<Box<Node>>,
    size: usize,
  }
  impl Node {
    pub fn child<N: PrimInt>(&mut self, i: N) -> &mut Option<Box<Node>> {
      assert!(i.is_zero() || i.is_one());
      if i.is_zero() {
        &mut self.zero
      } else {
        &mut self.one
      }
    }
  }

  #[derive(Debug, Clone)]
  pub struct BinaryTrie<N> {
    root: Box<Node>,
    bit_start: N,
    xor_mask: N,
  }
  impl<N: PrimInt> BinaryTrie<N> {
    pub fn new(depth: usize) -> Self {
      Self {
        root: Box::new(Node::default()),
        bit_start: N::one() << (depth - 1),
        xor_mask: N::zero(),
      }
    }

    pub fn insert(&mut self, x: N) {
      let mut b = self.bit_start;
      let mut node = &mut self.root;
      node.size += 1;
      while !b.is_zero() {
        let child = node.child(mask(x, b)).get_or_insert_with(|| Box::new(Node::default()) );
        child.size += 1;
        node = child;
        b = b >> 1;
      }
    }

    pub fn pop_min(&mut self) -> N {
      #![allow(unused_assignments)]
      let mut b = self.bit_start;
      let mut node = &mut self.root;
      let mut ret = N::zero();
      let mut tmp = None;
      node.size -= 1;
      while !b.is_zero() {
        let mut i = mask(self.xor_mask, b);
        if node.child(i).is_none() {
          i = i ^ N::one();
        }
        ret = ret << 1 | i;
        if node.child(i).as_ref().map(|child| child.size ).unwrap() > 1 {
          node.child(i).as_mut().unwrap().size -= 1;
          node = node.child(i).as_mut().unwrap();
        } else {
          tmp = node.child(i).take();
          node = tmp.as_mut().unwrap();
        }
        b = b >> 1;
      }
      ret
    }
  }
}
