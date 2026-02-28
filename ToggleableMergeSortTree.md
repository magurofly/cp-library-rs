# 要素のON/OFF可能なMergeSortTree

`.set(i, state)` で $i$ 番目の要素が存在する/しないを切り替えることができる。

初期状態ではどの要素も存在しない。

- `ToggleableMergeSortTree::new(slice)`
- `set(&mut self, i: usize, state: bool)`
- `count(&self, index_range: impl RangeBounds<usize>, value_range: impl RangeBounds<T>) -> usize`

## コード

```rs
#[derive(Clone)]
pub struct ToggleableMergeSortTree<T> {
    len: usize,
    n: usize,
    data: Vec<Vec<(T, usize)>>,
    count: Vec<FenwicktreeSet>,
    position: Vec<Vec<(usize, usize)>>,
}
impl<T: Clone + Ord> ToggleableMergeSortTree<T> {
    pub fn new(slice: &[T]) -> Self {
        let len = slice.len();
        let n = len.next_power_of_two();
        let mut data = vec![vec![]; 2 * n];
        for i in 0 .. slice.len() {
            data[n + i].push((slice[i].clone(), i));
        }
        for i in (1 .. n).rev() {
            let (dst, src) = data.split_at_mut(i * 2);
            Self::merge(&mut dst[i], &src[0], &src[1]);
        }
        let mut position = vec![vec![]; len];
        for a in 0 .. 2 * n {
            for j in 0 .. data[a].len() {
                position[data[a][j].1].push((a, j));
            }
        }
        let count = (0 .. 2 * n).map(|i| FenwicktreeSet::new(data[i].len()) ).collect::<Vec<_>>();
        Self { len, n, data, position, count }
    }

    fn merge(dst_value: &mut Vec<(T, usize)>, src1_value: &[(T, usize)], src2_value: &[(T, usize)]) {
        dst_value.reserve_exact(src1_value.len() + src2_value.len());
        let mut i = 0;
        let mut j = 0;
        while i < src1_value.len() && j < src2_value.len() {
            if src1_value[i].0 <= src2_value[j].0 {
                dst_value.push(src1_value[i].clone());
                i += 1;
            } else {
                dst_value.push(src2_value[j].clone());
                j += 1;
            }
        }
        for i in i .. src1_value.len() {
            dst_value.push(src1_value[i].clone());
        }
        for j in j .. src2_value.len() {
            dst_value.push(src2_value[j].clone());
        }
    }

    pub fn set(&mut self, i: usize, state: bool) {
        assert!(i < self.len);
        for &(a, j) in &self.position[i] {
            self.count[a].set(j, state);
        }
    }

    fn count_l(&self, index_range: &impl std::ops::RangeBounds<usize>, a: std::ops::Bound<&T>) -> usize {
        use std::ops::Bound::*;
        let mut l = match index_range.start_bound() { Included(&l) => l, Excluded(&r) => r + 1, Unbounded => 0 };
        let mut r = match index_range.end_bound() { Included(&r) => r.saturating_sub(1), Excluded(&r) => r, Unbounded => self.len };
        assert!(r <= self.len);
        l += self.n;
        r += self.n;
        let mut count = 0;
        while l < r {
            if l & 1 != 0 {
                let j = self.data[l].partition_point(|x| match a { Included(a) => &x.0 <= a, Excluded(a) => &x.0 < a, Unbounded => true });
                count += self.count[l].count(.. j);
                l += 1;
            }
            l >>= 1;
            if r & 1 != 0 {
                r -= 1;
                let j = self.data[r].partition_point(|x| match a { Included(a) => &x.0 <= a, Excluded(a) => &x.0 < a, Unbounded => true });
                count += self.count[r].count(.. j);
            }
            r >>= 1;
        }
        count
    }
    
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
}


#[derive(Clone, Debug)]
pub struct FenwicktreeSet {
    len: usize,
    low_bitset: Vec<u64>,
    high_bit: Vec<i32>,
}
impl From<Vec<bool>> for FenwicktreeSet {
    fn from(value: Vec<bool>) -> Self {
        let mut set = Self::new(value.len());
        for i in 0 .. value.len() {
            set.low_bitset[i >> 6] |= (value[i] as u64) << (i & 63);
        }
        for i in 0 .. set.low_bitset.len() {
            set.high_bit[i + 1] = set.low_bitset[i].count_ones() as i32;
        }
        for i in 1 .. set.low_bitset.len() {
            set.high_bit[i + (1 << i.trailing_zeros())] += set.high_bit[i];
        }
        set
    }
}
impl FenwicktreeSet {
    pub fn new(len: usize) -> Self {
        let low_bitset = vec![0; len + 63 >> 6];
        let high_bit = vec![0; low_bitset.len() + 1];
        Self { len, low_bitset, high_bit }
    }

    pub fn set(&mut self, p: usize, x: bool) {
        assert!(p < self.len);
        let p_low = p & 63;
        let mut p_high = p >> 6;

        let bs  = &mut self.low_bitset[p_high];
        let mask = (*bs & (1 << p_low)) ^ ((x as u64) << p_low);
        if mask == 0 {
            return;
        }
        let add = if *bs & 1 << p_low == 0 { 1 } else { -1 };
        *bs ^= mask;

        p_high += 1;
        self.high_bit[p_high] = bs.count_ones() as i32;
        while p_high <= self.low_bitset.len() {
            self.high_bit[p_high] += add;
            p_high += p_high & p_high.wrapping_neg();
        }
    }

    pub fn get(&self, p: usize) -> bool {
        self.low_bitset[p >> 6] >> (p & 63) & 1 != 0
    }

    #[inline]
    fn sum(&self, mut r: usize) -> usize {
        let mut count = 0;

        if r & 63 != 0 {
            count += (self.low_bitset[r - 1 >> 6] & u64::MAX >> (63 - (r - 1 & 63))).count_ones() as usize;
        }

        r >>= 6;
        while r > 0 {
            count += self.high_bit[r] as usize;
            r -= r & r.wrapping_neg();
        }

        count
    }

    pub fn count(&self, range: impl std::ops::RangeBounds<usize>) -> usize {
        use std::ops::Bound::*;
        let l = match range.start_bound() { Included(&p) => p, Excluded(&p) => p + 1, Unbounded => 0 };
        let r = match range.end_bound() { Included(&p) => p + 1, Excluded(&p) => p, Unbounded => self.len };
        assert!(r <= self.len);
        self.sum(r) - self.sum(l)
    }
}

```
