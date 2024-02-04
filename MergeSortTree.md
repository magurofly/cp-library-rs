# マージソート木
配列を セグ木に乗せて ソートする。（内部実装はマージソート）
一つの要素は $O(log n)$ 個の配列に含まれるため、空間計算量は $O(n \log n)$ となる。

ある区間 $[l, r)$ において、 $[x, y)$ に含まれる値の個数を求めるクエリを処理できる。
区間を $O(\log n)$ 個の重ならないノードで表すことができるため、それぞれのノードにおいて二分探索を行うことで、時間計算量は $O(\log^2 n)$ となる。

# コード
```rs
#[derive(Clone, Debug)]
pub struct MergeSortTree<T> {
    len: usize,
    n: usize,
    data: Vec<Vec<T>>,
}
impl<T: Clone + Ord> MergeSortTree<T> {
    /// time complexity: O(n log n)
    /// space complexity: O(n log n)
    pub fn new(slice: &[T]) -> Self {
        let len = slice.len();
        let n = len.next_power_of_two();
        let mut data = vec![vec![]; 2 * n];
        for i in 0 .. slice.len() {
            data[n + i].push(slice[i].clone());
        }
        for i in (1 .. n).rev() {
            let (dst, src) = data.split_at_mut(i * 2);
            Self::merge(&mut dst[i], &src[0], &src[1]);
        }
        Self { len, n, data }
    }

    /// count items in `index_range` such that contained in `value_range`
    /// time complexity: O(log^2 n)
    pub fn count(&self, index_range: impl std::ops::RangeBounds<usize>, value_range: impl std::ops::RangeBounds<T>) -> usize {
        use std::ops::Bound::*;
        let mut count = self.count_l(&index_range, value_range.end_bound());
        match value_range.start_bound() {
            Included(l) => {
                count -= self.count_l(&index_range, Excluded(l));
            }
            Excluded(l) => {
                count -= self.count_l(&index_range, Included(l));
            }
            Unbounded => {}
        }
        count
    }

    fn count_l(&self, index_range: &impl std::ops::RangeBounds<usize>, a: std::ops::Bound<&T>) -> usize {
        use std::ops::Bound::*;
        let mut l = self.n + match index_range.start_bound() { Included(&l) => l, Excluded(&r) => r + 1, Unbounded => 0 };
        let mut r = self.n + match index_range.end_bound() { Included(&r) => r.saturating_sub(1), Excluded(&r) => r, Unbounded => self.len };
        assert!(r <= self.len);
        let mut count = 0;
        while l < r {
            if l & 1 != 0 {
                count += self.data[l].partition_point(|x| match a { Included(a) => x <= a, Excluded(a) => x < a, Unbounded => true });
                l += 1;
            }
            l >>= 1;
            if r & 1 != 0 {
                r -= 1;
                count += self.data[r].partition_point(|x| match a { Included(a) => x <= a, Excluded(a) => x < a, Unbounded => true });
            }
            r >>= 1;
        }
        count
    }

    fn merge(dst: &mut Vec<T>, src1: &[T], src2: &[T]) {
        dst.reserve_exact(src1.len() + src2.len());
        let mut i = 0;
        let mut j = 0;
        while i < src1.len() && j < src2.len() {
            if src1[i] <= src2[j] {
                dst.push(src1[i].clone());
                i += 1;
            } else {
                dst.push(src2[j].clone());
                j += 1;
            }
        }
        for i in i .. src1.len() {
            dst.push(src1[i].clone());
        }
        for j in j .. src2.len() {
            dst.push(src2[j].clone());
        }
    }
}
```
