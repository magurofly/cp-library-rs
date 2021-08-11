use super::*;
use std::str::FromStr;

macro_rules! impl_to {
  ($name:ident, $type:ty) => {
    fn $name(&self) -> $type {
      <$type>::from_str(&self.to_s()).unwrap()
    }
  }
}

pub trait ToS {
  fn to_s(&self) -> String;

  fn to_chars(&self) -> Vec<char> {
    self.to_s().chars().to_vec()
  }

  impl_to!(to_i8, i8);
  impl_to!(to_i16, i16);
  impl_to!(to_i32, i32);
  impl_to!(to_i64, i64);
  impl_to!(to_i128, i128);
  impl_to!(to_u8, u8);
  impl_to!(to_u16, u16);
  impl_to!(to_u32, u32);
  impl_to!(to_u64, u64);
  impl_to!(to_u128, u128);
  impl_to!(to_f32, f32);
  impl_to!(to_f64, f64);
}

macro_rules! impl_from {
  ($name:ident) => {
    impl ToS for $name {
      fn to_s(&self) -> String {
        self.to_string()
      }
    }
  }
}

impl_from!(char);
impl_from!(i8);
impl_from!(str);
impl_from!(u8);
impl_from!(f32);
impl_from!(f64);
impl_from!(i128);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(u128);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(String);

impl<S: ToS> ToS for [S] {
  fn to_s(&self) -> String {
    self.into_iter().map(S::to_s).to_vec().join("")
  }
}
