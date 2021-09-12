use succinct_indexable_dictionary::SuccinctIndexableDictionary;
use bits::*;

pub struct WaveletMatrixUsize {
  len: usize,
  sigma: usize,
  height: usize,
  dic: Vec<SuccinctIndexableDictionary<u32>>,
  mid: Vec<usize>,
}

impl WaveletMatrixUsize {
  pub fn len(&self) -> usize { self.len }
  pub fn sigma(&self) -> usize { self.sigma }

  pub fn new(mut a: Vec<usize>, sigma: usize) -> Self {
    let len = a.len();
    let height = sigma.bit_length();
    let mut dic = Vec::with_capacity(height);
    let mut mid = Vec::with_capacity(height);
    for level in (0 .. height).rev() {
      let mut sid = SuccinctIndexableDictionary::new(len + 1);
      for i in 0 .. len {
        sid.set(i, a[i].bit_at(level));
      }
      dic.push(sid);
      
      // level ビット目が 0 のものを左に、 1 のものを右に移動させる
      partition(&mut a, &|&x| !x.bit_at(level));
      mid.push(a.partition_point(|&x| !x.bit_at(level)));
    }

    // // 逆順にpushしたので
    dic.reverse();
    mid.reverse();

    Self {
      len,
      sigma,
      height,
      dic,
      mid
    }
  }

  /// idx 番目の要素を返す
  /// O(log sigma)
  pub fn get(&self, idx: usize) -> usize {
    self.access(idx)
  }

  /// idx 番目の要素を返す
  /// O(log sigma)
  pub fn access(&self, mut idx: usize) -> usize {
    let mut ret = 0;
    for level in (0 .. self.height).rev() {
      let f = self.dic[level].get(idx);
      ret.bit_set(level, f);
      idx = self.rnk(f, idx, level);
    }
    ret
  }

  /// [0, r) にある x の数を返す
  /// O(log sigma)
  pub fn rank(&self, x: usize, mut r: usize) -> usize {
    let mut l = 0;
    for level in (0 .. self.height).rev() {
      l = self.rnk(x.bit_at(level), l, level);
      r = self.rnk(x.bit_at(level), r, level);
    }
    r - l
  }

  /// num 番目の x の位置を返す
  /// O(log N log sigma)
  pub fn select(&self, x: usize, num: usize) -> Option<usize> {
    if num == 0 {
      return Some(0);
    }
    if self.rank(x, self.len()) < num {
      return None;
    }
    let mut wa = 0;
    let mut ac = self.len();
    while wa + 1 < ac {
      let wj = (ac + wa) / 2;
      *(if self.rank(x, wj) <= num { &mut ac } else { &mut wa }) = wj;
    }
    Some(ac)
  }

  /// [l, r) における k 番目に小さい要素を返す
  /// O(log sigma)
  pub fn kth_smallest(&self, mut l: usize, mut r: usize, mut k: usize) -> usize {
    assert!(l <= r);
    assert!(k < r - l);
    let mut ret = 0;
    for level in (0 .. self.height).rev() {
      let cnt = self.dic[level].rank0_in(l, r);
      let f = cnt <= k;
      if f {
        ret.bit_on(level);
        k -= cnt;
      }
      l = self.rnk(f, l, level);
      r = self.rnk(f, r, level);
    }
    ret
  }

  /// [l, r) における k 番目に大きい要素を返す
  /// O(log sigma)
  pub fn kth_largest(&self, l: usize, r: usize, k: usize) -> usize {
    self.kth_smallest(l, r, r - l - k - 1)
  }

  /// [l, r) における x 未満の要素の個数を返す
  /// O(log sigma)
  pub fn range_freq(&self, mut l: usize, mut r: usize, x: usize) -> usize {
    let mut ret = 0;
    for level in (0 .. self.height).rev() {
      let f = x.bit_at(level);
      if f {
        ret += self.dic[level].rank0_in(l, r);
      }
      l = self.rnk(f, l, level);
      r = self.rnk(f, r, level);
    }
    ret
  }

  /// [l, r) における low 以上 high 未満の要素の個数を返す
  /// O(log sigma)
  pub fn range_freq_between(&self, l: usize, r: usize, low: usize, high: usize) -> usize {
    self.range_freq(l, r, high) - self.range_freq(l, r, low)
  }

  /// [l, r) における high 未満の最大値を返す
  /// O(log sigma)
  pub fn prev_value(&self, l: usize, r: usize, high: usize) -> Option<usize> {
    let cnt = self.range_freq(l, r, high);
    if cnt == 0 {
      None
    } else {
      Some(self.kth_smallest(l, r, cnt - 1))
    }
  }

  /// [l, r) における low 以上の最小値を返す
  /// O(log sigma)
  pub fn next_value(&self, l: usize, r: usize, low: usize) -> Option<usize> {
    let cnt = self.range_freq(l, r, low);
    if cnt == r - l {
      None
    } else {
      Some(self.kth_smallest(l, r, cnt))
    }
  }

  fn rnk(&self, f: bool, i: usize, level: usize) -> usize {
    if f {
      self.dic[level].rank1(i) + self.mid[level]
    } else {
      self.dic[level].rank0(i)
    }
  }
}

/// https://gist.github.com/alphaWizard/e821c4717b73b37fb34a926518dd6a8f
/// f が真になる要素を前に持ってくる
fn partition<T>(a: &mut [T], f: &impl Fn(&T) -> bool) {
  if a.len() <= 1 {
    return;
  }

  let half = a.len() / 2;
  partition(&mut a[.. half], f);
  partition(&mut a[half ..], f);

  let l = (0 .. half).find(|&i| !(f)(&a[i])).unwrap_or(half);
  let r = (half .. a.len()).find(|&i| !(f)(&a[i])).unwrap_or(a.len());
  a[l .. half].reverse();
  a[half .. r].reverse();
  a[l .. r].reverse();
}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn access() {
    // let a = vec![3, 1, 4, 1, 5, 9, 2, 6, 4, 8];
    let a = vec![11, 0, 15, 6, 5, 2, 7, 12, 11, 0, 12, 12, 13, 4, 6, 13, 1, 11, 6, 1, 7, 10, 2, 7, 14, 11, 1, 7, 5, 4, 14, 6];
    let wm = WaveletMatrixUsize::new(a.clone(), a.iter().copied().max().unwrap() + 1);
    let b = (0 .. a.len()).map(|i| wm.access(i)).collect::<Vec<_>>();
    assert_eq!(a, b);
  }
}