# Implicit Treap

まだ Verify してない

```rs

use implicit_treap::*;
pub mod implicit_treap {
    pub trait ImplicitTreapHelper {
        /// 要素
        type S: Clone;
        /// 作用素
        type F: Clone;
        /// 要素に対する二項演算
        fn op(x: &Self::S, y: &Self::S) -> Self::S;
        /// 二項演算の単位元
        fn e() -> Self::S;
        /// 区間反転したときの結果
        fn rev(x: &Self::S) -> Self::S { x.clone() }
        /// 作用
        fn map(f: &Self::F, x: &Self::S) -> Self::S { (f, x).1.clone() }
        /// 作用素の合成（ `f` の方が先にあった作用）
        fn compose(f: &Self::F, g: &Self::F) -> Self::F { (f, g).1.clone() }
    }
    
    pub struct ImplicitTreap<H: ImplicitTreapHelper> {
        root: Option<Box<Node<H>>>
    }
    
    impl<H: ImplicitTreapHelper> ImplicitTreap<H> {
        pub fn new() -> Self {
            Self { root: None }
        }
        
        pub fn len(&self) -> usize {
            self.root.as_ref().map(|node| node.len() ).unwrap_or(0)
        }
        
        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
        
        pub fn split_off(&mut self, at: usize) -> Self {
            assert!(at <= self.len());
            let [head, tail] = Node::split(self.root.take(), at);
            self.root = head;
            Self { root: tail }
        }
        
        pub fn append(&mut self, other: &mut Self) {
            self.root = Node::merge(self.root.take(), other.root.take());
        }
        
        pub fn insert(&mut self, at: usize, value: H::S) {
            assert!(at <= self.len());
            let [head, tail] = Node::split(self.root.take(), at);
            self.root = Node::merge(Node::merge(head, Some(Node::new(value))), tail);
        }

        pub fn push_front(&mut self, value: H::S) {
            self.root = Node::merge(Some(Node::new(value)), self.root.take());
        }

        pub fn push_back(&mut self, value: H::S) {
            self.root = Node::merge(self.root.take(), Some(Node::new(value)));
        }
        
        pub fn remove(&mut self, index: usize) -> H::S {
            assert!(index < self.len());
            let [head, mid, tail] = Node::split3(self.root.take(), index, index + 1);
            self.root = Node::merge(head, tail);
            mid.unwrap().value
        }

        pub fn pop_front(&mut self) -> Option<H::S> {
            let [head, tail] = Node::split(self.root.take(), 1);
            self.root = tail;
            head.map(|node| node.into_value() )
        }

        pub fn pop_back(&mut self) -> Option<H::S> {
            let len = self.len();
            let [head, tail] = Node::split(self.root.take(), len - 1);
            self.root = head;
            tail.map(|node| node.into_value() )
        }

        /// [l, r) の二項演算の結果を返す
        pub fn prod(&mut self, l: usize, r: usize) -> H::S {
            assert!(l <= r && r <= self.len());
            let [head, mut mid, tail] = Node::split3(self.root.take(), l, r);
            let result = mid.as_mut().unwrap().prod().clone();
            self.root = Node::merge(Node::merge(head, mid), tail);
            result
        }

        /// [l, r) に f を適用する
        pub fn apply(&mut self, l: usize, r: usize, f: &H::F) {
            assert!(l <= r && r <= self.len());
            let [head, mut mid, tail] = Node::split3(self.root.take(), l, r);
            mid.as_mut().unwrap().apply(f);
            self.root = Node::merge(Node::merge(head, mid), tail);
        }

        /// `pred(self.prod(0, r))` が `true` となるような最大の r を返す
        pub fn partition_point(&mut self, mut pred: impl FnMut(&H::S) -> bool) -> usize {
            assert!(pred(&H::e()));
            self.root.as_mut().map(|root| root.partition_point(pred) ).unwrap_or(0)
        }
        
        pub fn reverse(&mut self) {
            if let Some(root) = &mut self.root {
                root.apply_rev();
            }
        }

        /// [0, k) [k, n) に分割し、 [k, n) [0, k) にする
        pub fn rotate(&mut self, k: usize) {
            let mut tail = self.split_off(k);
            self.reverse();
            tail.reverse();
            self.append(&mut tail);
            self.reverse();
        }
    }
    
    struct Node<H: ImplicitTreapHelper> {
        // 左右の子
        children: [Option<Box<Node<H>>>; 2],
        // 優先度
        priority: u32,
        // 部分木の要素数
        len: usize,
        // 区間反転（遅延評価）
        // true である場合、左右の子の入れ替えをよび value, prod への適用は既にされているが子への伝搬がまだであることを表す
        rev: bool,
        // 要素の値
        value: H::S,
        // 部分木の二項演算の結果
        prod: H::S,
        // 区間作用（遅延評価）
        // Some である場合、 value, prod には既に適用されているが子への伝搬がまだであることを表す
        lazy: Option<H::F>,
    }
    
    impl<H: ImplicitTreapHelper> Node<H> {
        pub fn new(value: H::S) -> Box<Self> {
            let prod = value.clone();
            Box::new(Self {
                children: [None, None],
                priority: thread_rng().next_u32(),
                len: 1,
                rev: false,
                value,
                prod,
                lazy: None,
            })
        }
        
        pub fn len(&self) -> usize {
            self.len
        }

        pub fn into_value(self) -> H::S {
            self.value
        }

        pub fn value(&self) -> &H::S {
            &self.value
        }

        pub fn prod(&self) -> &H::S {
            &self.prod
        }

        pub fn partition_point(&mut self, mut pred: impl FnMut(&H::S) -> bool) -> usize {
            self.propagate();
            if pred(&self.prod) {
                return self.len;
            }
            let mut left_len = 0;
            if let Some(left) = &mut self.children[0] {
                left_len += left.len;
                if !pred(&H::op(&left.prod, &self.value)) {
                    return left.partition_point(pred);
                }
            }
            if let Some(right) = &mut self.children[1] {
                return left_len + 1 + right.partition_point(pred);
            }
            0
        }
        
        pub fn split(root: Option<Box<Self>>, at: usize) -> [Option<Box<Self>>; 2] {
            if let Some(mut root) = root {
                root.propagate();
                let left_len = root.children[0].as_ref().map(|node| node.len ).unwrap_or(0);
                if at <= left_len {
                    let [head, tail] = Self::split(root.children[0].take(), at);
                    root.children[0] = tail;
                    root.aggregate();
                    return [head, Some(root)];
                }
                let [head, tail] = Self::split(root.children[1].take(), at - left_len - 1);
                root.children[1] = head;
                root.aggregate();
                [Some(root), tail]
            } else {
                [None, None]
            }
        }
        
        pub fn split3(root: Option<Box<Self>>, at_l: usize, at_r: usize) -> [Option<Box<Self>>; 3] {
            let [rem, tail] = Self::split(root, at_r);
            let [head, mid] = Self::split(rem, at_l);
            [head, mid, tail]
        }
        
        pub fn merge(left: Option<Box<Self>>, right: Option<Box<Self>>) -> Option<Box<Self>> {
            if left.is_some() && right.is_some() {
                let (mut left, mut right) = (left.unwrap(), right.unwrap());
                left.propagate();
                right.propagate();
                let mut root;
                if left.priority > right.priority {
                    left.children[1] = Self::merge(left.children[1].take(), Some(right));
                    root = left;
                } else {
                    right.children[0] = Self::merge(Some(left), right.children[0].take());
                    root = right;
                }
                root.aggregate();
                Some(root)
            } else {
                left.or(right)
            }
        }
        
        pub fn aggregate(&mut self) {
            self.propagate();
            let mut len = 1;
            let mut prod = self.value.clone();
            if let Some(left) = &self.children[0] {
                len += left.len;
                prod = H::op(&left.prod, &prod);
            }
            if let Some(right) = &self.children[1] {
                len += right.len;
                prod = H::op(&prod, &right.prod);
            }
            self.len = len;
            self.prod = prod;
        }
        
        pub fn propagate(&mut self) {
            if std::mem::replace(&mut self.rev, false) {
                for child in self.children.iter_mut().map(Option::as_mut).filter_map(identity) {
                    child.apply_rev();
                }
            }
            if let Some(lazy) = self.lazy.take() {
                for child in self.children.iter_mut().map(Option::as_mut).filter_map(identity) {
                    child.apply(&lazy);
                }
            }
        }
        
        pub fn apply_rev(&mut self) {
            self.children.swap(0, 1);
            self.value = H::rev(&self.value);
            self.prod = H::rev(&self.prod);
        }
        
        pub fn apply(&mut self, lazy: &H::F) {
            self.value = H::map(lazy, &self.value);
            self.prod = H::map(lazy, &self.prod);
            if let Some(orig_lazy) = self.lazy.take() {
                self.lazy = Some(H::compose(&orig_lazy, lazy));
            } else {
                self.lazy = Some(lazy.clone());
            }
        }
    }
    
    use std::convert::identity;
    use rand::prelude::*;
}
```
