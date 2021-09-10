use std::cell::RefCell;

/// 完備辞書（簡潔ビットベクトル）
/// ref: https://misteer.hatenablog.com/entry/bit-vector
pub struct SuccinctIndexableDictionary {
  len: usize,
  bit: Vec<u8>,
  helper: RefCell<SuccinctIndexableDictionaryHelper>,
}

impl SuccinctIndexableDictionary {
  pub fn chunk_size() -> usize { 256 }
  pub fn block_size() -> usize { 8 }

  pub fn from_vec(len: usize, mut bit: Vec<u8>) -> Self {
    let num_chunks = (len + Self::chunk_size() - 1) / Self::chunk_size();
    let num_blocks = Self::chunk_size() / Self::block_size();
    bit.resize(num_chunks * num_blocks, 0);
    Self {
      len,
      bit,
      helper: RefCell::new(SuccinctIndexableDictionaryHelper {
        num_chunks,
        num_blocks,
        chunks: vec![0; num_chunks + 1],
        blocks: vec![vec![0; num_blocks]; num_chunks],
        changed: false,
      }),
    }
  }

  pub fn new(len: usize) -> Self {
    Self::from_vec(len, vec![])
  }

  pub fn len(&self) -> usize {
    self.len
  }

  pub fn set(&mut self, idx: usize, value: bool) {
    self.helper.borrow_mut().changed = true;
    let bi = idx / Self::block_size();
    let bo = idx % Self::block_size();
    match value {
      true => {
        self.bit[bi] |= 1 << bo;
      }
      false => {
        self.bit[bi] &= !(1 << bo);
      }
    }
  }

  pub fn get(&self, idx: usize) -> bool {
    let bi = idx / Self::block_size();
    let bo = idx % Self::block_size();
    ((self.bit[bi] >> bo) & 1) == 1
  }

  // popCount(num) = num.count_ones()

  /// [0, idx) にある 1 の数を返す
  pub fn rank(&self, idx: usize) -> usize {
    self.check();
    let helper = self.helper.borrow();
    let ci = idx / Self::chunk_size();
    let bi = idx % Self::chunk_size() / Self::block_size();
    let bo = idx % Self::block_size();
    let masked = self.bit[ci * helper.num_blocks + bi] & ((1 << bo) - 1);
    helper.chunks[ci] as usize + helper.blocks[ci][bi] as usize + masked.count_ones() as usize
  }

  /// rank(idx) = num となる最小の idx を返す
  pub fn select(&self, num: usize) -> Option<usize> {
    if num == 0 {
      return Some(0);
    }
    if self.rank(self.len()) < num {
      return None;
    }
    let mut wa = 0;
    let mut ac = self.len();
    while wa + 1 < ac {
      let wj = (ac + wa) >> 1;
      *(if self.rank(wj) >= num { &mut ac } else { &mut wa }) = wj;
    }
    Some(ac)
  }

  /// 変更する
  fn build(&self) {
    let mut helper = self.helper.borrow_mut();
    helper.changed = false;
    helper.chunks[0] = 0;
    for i in 0 .. helper.num_chunks {
      helper.blocks[i][0] = 0;
      for j in 0 .. helper.num_blocks - 1 {
        helper.blocks[i][j + 1] = helper.blocks[i][j] + self.bit[i * helper.num_blocks + j].count_ones() as u8;
      }
      helper.chunks[i + 1] = helper.chunks[i] + helper.blocks[i][helper.num_blocks - 1] as u16 + self.bit[(i + 1) * helper.num_blocks - 1].count_ones() as u16;
    }
  }

  fn check(&self) {
    if self.helper.borrow().changed {
      self.build();
    }
  }
}

impl From<Vec<bool>> for SuccinctIndexableDictionary {
  fn from(array: Vec<bool>) -> Self {
    let mut dic = SuccinctIndexableDictionary::new(array.len());
    for i in 0 .. array.len() {
      dic.set(i, array[i]);
    }
    dic
  }
}

struct SuccinctIndexableDictionaryHelper {
  pub num_chunks: usize,
  pub num_blocks: usize,
  pub chunks: Vec<u16>,
  pub blocks: Vec<Vec<u8>>,
  pub changed: bool,
}

#[cfg(test)]
pub mod test {
    use crate::SuccinctIndexableDictionary;


  #[test]
  fn access() {
    let bits = vec![false, true, true, false, false];
    let dic = SuccinctIndexableDictionary::from(bits.clone());
    for i in 0 .. bits.len() {
      assert_eq!(bits[i], dic.get(i));
    }
  }

  #[test]
  fn rank() {
    let bits = vec![false, true, true, false, false, true];
    let dic = SuccinctIndexableDictionary::from(bits.clone());
    for i in 0 .. bits.len() {
      let count = (0 .. i).filter(|&j| bits[j]).count();
      assert_eq!(count, dic.rank(i));
    }
  }

  #[test]
  fn select() {
    let bits = vec![false, true, true, false, false, true];
    let dic = SuccinctIndexableDictionary::from(bits.clone());
    for i in 0 .. bits.len() {
      let idx = (0 ..= bits.len()).find(|&j| dic.rank(j) == i);
      assert_eq!(idx, dic.select(i));
    }
  }
}