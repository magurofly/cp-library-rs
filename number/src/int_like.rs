use std::convert::TryFrom;
use std::convert::TryInto;
use std::ops::*;
use std::cmp::*;
// use acl_modint::*;

pub trait IntLike: Sized + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + PartialEq {
  fn get0() -> Self { Self::from_usize(0) }
  fn get1() -> Self { Self::from_usize(1) }
  fn add1(self) -> Self { self + Self::get1() }
  fn sub1(self) -> Self { self - Self::get1() }
  fn as_usize(self) -> usize;
  fn from_usize(this: usize) -> Self;
  fn from_i64(this: i64) -> Self;
  fn modulus() -> usize;
}

impl<T: Sized + Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + PartialEq + TryFrom<usize> + TryFrom<i64> + TryInto<usize>> IntLike for T where <T as TryInto<usize>>::Error: std::fmt::Debug, <T as TryFrom<usize>>::Error: std::fmt::Debug, <T as TryFrom<i64>>::Error: std::fmt::Debug {
  fn as_usize(self) -> usize {
    self.try_into().unwrap()
  }

  fn from_usize(this: usize) -> Self {
    Self::try_from(this).unwrap()
  }

  fn from_i64(this: i64) -> Self {
    Self::try_from(this).unwrap()
  }

  fn modulus() -> usize {
    usize::try_from(1000000007i64 * 998244353i64).or(usize::try_from(1000000007i64)).unwrap()
  }
}

// macro_rules! impl_for_int {
//   ($type:ty) => {
//     impl IntLike for $type {
//       fn modulus() -> usize {
//         std::usize::MAX
//       }

//       fn as_usize(self) -> usize {
//         self as usize
//       }

//       fn from_i64(this: i64) -> Self {
//         this as Self
//       }

//       fn from_usize(this: usize) -> Self {
//         this as Self
//       }
      
//       fn add1(self) -> Self {
//         self + 1
//       }

//       fn sub1(self) -> Self {
//         self - 1
//       }
//     }
//   }
// }

// impl_for_int!(i8);
// impl_for_int!(u8);
// impl_for_int!(i16);
// impl_for_int!(u16);
// impl_for_int!(i32);
// impl_for_int!(u32);
// impl_for_int!(i64);
// impl_for_int!(u64);
// impl_for_int!(isize);
// impl_for_int!(usize);