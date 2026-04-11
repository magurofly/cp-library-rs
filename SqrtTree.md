# Sqrt Tree
静的な列の区間和クエリを計算する。

## コード
```rs
#[derive(Clone)]
pub struct SqrtTree<M: ac_library::Monoid> {
    len: usize,
    root: SqrtTreeNode<M>,
}

impl<M: ac_library::Monoid> SqrtTree<M> {
    pub fn new(slice: &[M::S]) -> Self {
        Self {
            len: slice.len(),
            root: SqrtTreeNode::new(slice),
        }
    }

    pub fn prod(&self, range: impl std::ops::RangeBounds<usize>) -> M::S {
        use std::ops::Bound::*;
        let l = match range.start_bound() { Included(&l) => l, Excluded(&l) => l + 1, Unbounded => 0 };
        let mut r = match range.end_bound() { Included(&r) => r + 1, Excluded(&r) => r, Unbounded => self.len };
        r = r.min(self.len);
        self.root.prod(l, r)
    }
}

#[derive(Clone)]
struct SqrtTreeNode<M: ac_library::Monoid> {
    prefix: Vec<M::S>,
    suffix: Vec<M::S>,
    block_size: usize,
    children: Vec<Box<Self>>,
    between: Vec<Vec<M::S>>,
}

impl<M: ac_library::Monoid> SqrtTreeNode<M> {
    fn new(slice: &[M::S]) -> Self {
        assert!(!slice.is_empty());

        let mut prefix = vec![M::identity(); slice.len()];
        prefix[0] = slice[0].clone();
        for i in 1 .. slice.len() {
            prefix[i] = M::binary_operation(&prefix[i - 1], &slice[i]);
        }
        let mut suffix = vec![M::identity(); slice.len()];
        suffix[slice.len() - 1] = slice[slice.len() - 1].clone();
        for i in (0 .. slice.len() - 1).rev() {
            suffix[i] = M::binary_operation(&slice[i], &suffix[i + 1]);
        }
        let block_size = (slice.len() as f64).sqrt().floor() as usize; // 0 -> 1, 1 -> 1, 2 -> 1
        let block_count = if slice.len() <= 1 { 0 } else { (slice.len() + block_size - 1) / block_size };

        let children = (0 .. block_count).map(|b| {
            let l = block_size * b;
            let r = slice.len().min(l + block_size);
            Box::new(Self::new(&slice[l .. r]))
        }).collect::<Vec<_>>();
        let mut between = vec![];
        for bl in 0 .. block_count {
            let mut x = M::identity();
            between.push((bl .. block_count).map(|b| {
                x = M::binary_operation(&x, &children[b].suffix[0]);
                x.clone()
            }).collect::<Vec<_>>());
        }
            
        Self {
            prefix,
            suffix,
            block_size,
            children,
            between,
        }
    }

    fn prod(&self, l: usize, r: usize) -> M::S {
        if l >= r {
            return M::identity();
        }
        if l == 0 {
            return self.prefix[r - 1].clone();
        }
        if r >= self.prefix.len() {
            return self.suffix[l].clone();
        }
        let bl = l / self.block_size;
        let br = (r - 1) / self.block_size;
        let mut result;
        if bl < br {
            result = self.children[bl].prod(l - bl * self.block_size, self.block_size);
            if bl + 1 < br {
                result = M::binary_operation(&result, &self.between[bl + 1][(br - 1) - (bl + 1)]);
            }
            result = M::binary_operation(&result, &self.children[br].prefix[(r - 1) - br * self.block_size]);
        } else {
            result = self.children[bl].prod(l - bl * self.block_size, r - bl * self.block_size);
        }
        result
    }
}

```
