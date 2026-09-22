# 要素のON/OFF可能なMergeSortTree

$i$ 番目の要素が存在する/しないを切り替えることができる。

可換モノイドを乗せることができる。

- `ToggleableMergeSortTree::new(slice)`: `slice` から構築する（初期状態ではどの要素も存在しない）
- `ToggleableMergeSortTree::with_state(slice, states)`: `slice` と初期状態を指定して構築する
- `set(&mut self, i, state)`: `i` 番目の要素の状態を `state` に変更する
- `prod(&self, index_range, value_range) -> M::S`: 位置と値の区間を指定し、モノイド積を得る
- `max_right(&self, l, value_range, predicate)`, `min_left(&self, r, value_range, predicate)`: 二分探索する（ACL セグメント木の `max_right`, `min_left` と同様の機能）

## コード

```rs
use ac_library::{Segtree, Monoid};

#[derive(Clone)]
/// 要素の ON/OFF 可能な MergeSortTree (位置の区間と値の区間を両方指定してモノイド積を得られるデータ構造)
/// 任意の $i$ について、 $i$ 番目の要素が存在する/しないを切り替えることができる
pub struct ToggleableMergeSortTree<M: Monoid> {
    len: usize,
    n: usize,
    values: Vec<M::S>,
    indices: Vec<Vec<u32>>,
    tree: Vec<Segtree<M>>,
    position: Vec<Vec<(u32, u32)>>,
}
impl<M: Monoid> ToggleableMergeSortTree<M> where M::S: Clone + Ord {
    /// 要素 `values` と初期状態 `states` を指定して構築する
    pub fn with_states(values: &[M::S], states: &[bool]) -> Self {
        assert!(values.len() == states.len());
        let values = values.to_vec();
        let len = values.len();
        let n = len.next_power_of_two();
        let mut indices = vec![vec![]; 2 * n];
        for i in 0 .. values.len() {
            indices[n + i].push(i as u32);
        }
        for i in (1 .. n).rev() {
            let (dst, src) = indices.split_at_mut(i * 2);
            Self::merge(&values, &mut dst[i], &src[0], &src[1]);
        }
        let mut position = vec![vec![]; len];
        for a in 0 .. 2 * n {
            for j in 0 .. indices[a].len() {
                position[indices[a][j] as usize].push((a as u32, j as u32));
            }
        }
        let tree = (0 .. 2 * n).map(|i| {
            let v = indices[i].iter().map(|&j| {
                if states[j as usize] {
                    values[j as usize].clone()
                } else {
                    M::identity()
                }
            }).collect::<Vec<_>>();
            Segtree::<M>::from(v)
        }).collect::<Vec<_>>();
        Self { len, n, values, position, indices, tree }
    }

    /// `values` から構築する（初期状態: すべて存在しない）
    pub fn new(values: &[M::S]) -> Self {
        Self::with_states(values, &vec![false; values.len()])
    }

    fn merge(values: &[M::S], dst: &mut Vec<u32>, src1: &[u32], src2: &[u32]) {
        dst.reserve_exact(src1.len() + src2.len());
        let mut i = 0;
        let mut j = 0;
        while i < src1.len() && j < src2.len() {
            if values[src1[i] as usize] <= values[src2[j] as usize] {
                dst.push(src1[i]);
                i += 1;
            } else {
                dst.push(src2[j]);
                j += 1;
            }
        }
        dst.extend_from_slice(&src1[i ..]);
        dst.extend_from_slice(&src2[j ..]);
    }

    /// `i` 番目の要素の状態を `state` にする
    pub fn set(&mut self, i: usize, state: bool) {
        assert!(i < self.len);
        if state {
            for &(a, j) in &self.position[i] {
                self.tree[a as usize].set(j as usize, self.values[i].clone());
            }
        } else {
            for &(a, j) in &self.position[i] {
                self.tree[a as usize].set(j as usize, M::identity());
            }
        }
    }

    fn prod_sub(&self, k: usize, value_range: &impl std::ops::RangeBounds<M::S>) -> M::S {
        use std::ops::Bound::*;
        let i0 = match value_range.start_bound() {
            Included(a) => self.indices[k].partition_point(|&i| &self.values[i as usize] < a ),
            Excluded(a) => self.indices[k].partition_point(|&i| &self.values[i as usize] <= a ),
            Unbounded => 0,
        };
        let i1 = match value_range.end_bound() {
            Included(a) => self.indices[k].partition_point(|&i| &self.values[i as usize] <= a ),
            Excluded(a) => self.indices[k].partition_point(|&i| &self.values[i as usize] < a ),
            Unbounded => self.indices[k].len(),
        };
        self.tree[k].prod(i0 .. i1)
    }

    /// 位置が `index_range` に含まれ、値が `value_range` に含まれる要素のモノイド積を得る
    pub fn prod(&self, index_range: impl std::ops::RangeBounds<usize>, value_range: impl std::ops::RangeBounds<M::S>) -> M::S {
        use std::ops::Bound::*;
        let mut l = match index_range.start_bound() { Included(&l) => l, Excluded(&r) => r + 1, Unbounded => 0 };
        let mut r = match index_range.end_bound() { Included(&r) => r.saturating_sub(1), Excluded(&r) => r, Unbounded => self.len };
        assert!(r <= self.len);
        l += self.n;
        r += self.n;
        let mut prod_l = M::identity();
        let mut prod_r = M::identity();
        while l < r {
            if l & 1 != 0 {
                prod_l = M::binary_operation(&prod_l, &self.prod_sub(l, &value_range));
                l += 1;
            }
            l >>= 1;
            if r & 1 != 0 {
                r -= 1;
                prod_r = M::binary_operation(&self.prod_sub(r, &value_range), &prod_r);
            }
            r >>= 1;
        }
        M::binary_operation(&prod_l, &prod_r)
    }

    /// 位置が `l..r` に含まれ、値が `value_range` に含まれる要素のモノイド積が `predicate` を満たすような最大の `r` を得る
    pub fn max_right(&self, mut l: usize, value_range: impl std::ops::RangeBounds<M::S>, mut predicate: impl FnMut(&M::S) -> bool) -> usize {
        assert!(l <= self.len);
        assert!(predicate(&M::identity()));
        if l == self.len { return self.len; }
        l += self.n;
        let mut x = M::identity();
        loop {
            l >>= l.trailing_zeros();
            let y = M::binary_operation(&x, &self.prod_sub(l, &value_range));
            if !predicate(&y) {
                while l < self.n {
                    l *= 2;
                    let y = M::binary_operation(&x, &self.prod_sub(l, &value_range));
                    if predicate(&y) {
                        x = y;
                        l += 1;
                    }
                }
                return l - self.n;
            }
            x = y;
            l += 1;
            if l.is_power_of_two() {
                break;
            }
        }
        self.len
    }

    /// 位置が `l..r` に含まれ、値が `value_range` に含まれる要素のモノイド積が `predicate` を満たすような最小の `l` を得る
    pub fn min_left(&self, mut r: usize, value_range: impl std::ops::RangeBounds<M::S>, mut predicate: impl FnMut(&M::S) -> bool) -> usize {
        assert!(r <= self.len);
        assert!(predicate(&M::identity()));
        if r == 0 { return 0; }
        r += self.n;
        let mut x = M::identity();
        loop {
            r -= 1;
            r >>= r.trailing_ones();
            let y = M::binary_operation(&self.prod_sub(r, &value_range), &x);
            if !predicate(&y) {
                while r < self.n {
                    r = 2 * r + 1;
                    let y = M::binary_operation(&self.prod_sub(r, &value_range), &x);
                    if predicate(&y) {
                        x = y;
                        r -= 1;
                    }
                }
                return r + 1 - self.n;
            }
            x = y;
            if r.is_power_of_two() {
                break;
            }
        }
        0
    }
}

```
