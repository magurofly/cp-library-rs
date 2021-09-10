use succinct_indexable_dictionary::SuccinctIndexableDictionary;

pub struct WaveletMatrixUsize {
  len: usize,
  sigma: usize,
  height: usize,
  dic: Vec<SuccinctIndexableDictionary>,
  mid: Vec<usize>,
}

impl WaveletMatrixUsize {
  pub fn len(&self) -> usize { self.len }
  pub fn sigma(&self) -> usize { self.sigma }
}