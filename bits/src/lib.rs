use std::ops::*;

pub trait Bits: Sized + Clone + Copy + PartialEq + Eq + Default + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Shl<u32, Output = Self> + Shr<u32, Output = Self> {
  fn width() -> usize;
  fn top(idx: usize) -> Self;
  fn full(count: usize) -> Self;
  fn none() -> Self;
  fn bit_length(self) -> usize;
  fn clz(self) -> usize;
  fn ctz(self) -> usize;
  fn popcount(self) -> usize;

  fn bit_at(self, idx: usize) -> bool {
    assert!(idx < Self::width());
    (self & Self::top(idx)) != Self::none()
  }
  fn bit_on(&mut self, idx: usize) { *self = *self | Self::top(idx); }
  fn bit_off(&mut self, idx: usize) { *self = *self & !Self::top(idx); }
  fn bit_flip(&mut self, idx: usize) { *self = *self ^ Self::top(idx); }
  fn bit_set(&mut self, idx: usize, val: bool) { if val { self.bit_on(idx); } else { self.bit_off(idx); } }
}

macro_rules! impl_int {
  ($t:ty, $w:expr) => {
    impl Bits for $t {
      fn width() -> usize { $w }
      fn top(idx: usize) -> Self { 1 << idx as u32 }
      fn full(count: usize) -> Self { (1 << count) - 1 }
      fn none() -> Self { 0 }
      fn clz(self) -> usize { self.leading_zeros() as usize }
      fn ctz(self) -> usize { self.trailing_zeros() as usize }
      fn popcount(self) -> usize { self.count_ones() as usize }
      fn bit_length(self) -> usize { if self == 0 { 0 } else { $w - (self - 1).clz() } }
    }
  };
}

impl_int!(u8, 8);
impl_int!(i8, 8);
impl_int!(u16, 16);
impl_int!(i16, 16);
impl_int!(u32, 32);
impl_int!(i32, 32);
impl_int!(u64, 64);
impl_int!(i64, 64);
impl_int!(u128, 128);
impl_int!(i128, 128);