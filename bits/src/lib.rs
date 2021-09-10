use std::ops::*;

pub trait Bits: Sized + Clone + Copy + PartialEq + Eq + Default + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Not<Output = Self> + Shl<u32, Output = Self> + Shr<u32, Output = Self> {
  fn width() -> usize;
  fn flag(idx: usize) -> Self;
  fn none() -> Self;
  fn bit_length(self) -> usize;

  fn bit_at(self, idx: usize) -> bool {
    assert!(idx < Self::width());
    (self & Self::flag(idx)) != Self::none()
  }
  fn bit_on(self, idx: usize) -> Self { self | Self::flag(idx) }
  fn bit_off(self, idx: usize) -> Self { self & !Self::flag(idx) }
  fn bit_flip(self, idx: usize) -> Self { self ^ Self::flag(idx) }
}

macro_rules! impl_int {
  ($t:ty, $w:expr) => {
    impl Bits for $t {
      fn width() -> usize { $w }
      fn flag(idx: usize) -> Self { 1 << idx as u32 }
      fn none() -> Self { 0 }
      fn bit_length(self) -> usize { if self == 0 { 0 } else { $w - (self - 1).leading_zeros() as usize } }
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