use std::*;
use collections::*;
use iter::*;

pub fn say(x: impl fmt::Display) { println!("{}", x); }

pub fn yesno(c: bool) { println!("{}", if c { "Yes" } else { "No" }); }
pub fn yes() { yesno(true); }
pub fn no() { yesno(false); }

pub struct CompressCoords<'a, T> {
  coords: BTreeSet<T>,
  cache: RefCell<Option<Vec<&'a T>>>,
}
impl<'a, T: Clone + Ord> CompressCoords<'a, T> {
  pub fn new() -> Self {
    Self { coords: BTreeSet::new(), cache: RefCell::new(None) }
  }

  pub fn add(&mut self, x: T) {
    self.coords.insert(x);
    *self.cache.borrow_mut() = None;
  }

  pub fn index_of(&self, x: &T) -> Result<usize, usize> {
    self.cache.borrow_mut()
    .get_or_insert_with(|| self.coords.iter().cloned().collect())
    .binary_search(x)
  }

  pub fn get(&self, x: &T) -> usize {
    let r = self.index_of(x);
    r.ok().or(r.err()).unwrap()
  }
}

pub trait MyItertools : Iterator {
  fn to_vec(self) -> Vec<Self::Item> where Self: Sized { self.collect::<Vec<_>>() }
  fn to_vec_reversed(self) -> Vec<Self::Item> where Self: Sized { let mut v = self.collect::<Vec<_>>(); v.reverse(); v }
  fn to_vec_of<T: From<Self::Item>>(self) -> Vec<T> where Self: Sized { self.map(|x| x.into()).to_vec() }
  fn try_to_vec_of<T: std::convert::TryFrom<Self::Item>>(self) -> Result<Vec<T>, T::Error> where Self: Sized { let mut xs = Vec::with_capacity(self.size_hint().0); for x in self { xs.push(T::try_from(x)?); } Ok(xs) }
  fn tally(self) -> HashMap<Self::Item, usize> where Self: Sized, Self::Item: Copy + Eq + hash::Hash { let mut counts = HashMap::new(); self.for_each(|item| *counts.entry(item).or_default() += 1 ); counts }
  // fn cumprod<F: Fn(Self::Item, Self::Item) -> Self::Item>(self, init: Self::Item, f: F) -> CumProd<Self, Self::Item, F> where Self: Sized, Self::Item: Copy { CumProd { prod: init, f, iter: self } }
}
impl<T: ?Sized> MyItertools for T where T: Iterator {}

pub trait MyIntoIter : IntoIterator where Self: Sized {
  fn convert<U, V: FromIterator<U>>(self, f: impl FnMut(Self::Item) -> U) -> V { self.into_iter().map(f).collect() }
  fn implode(self, sep: &str) -> String where Self::Item: fmt::Display { self.into_iter().map(|x| format!("{}", x)).to_vec().join(sep) }
}
impl<T> MyIntoIter for T where T: IntoIterator {}

pub trait MyOrd : PartialOrd + Sized {
  fn max(self, other: Self) -> Self { if &self < &other { other } else { self } }
  fn min(self, other: Self) -> Self { if &self > &other { other } else { self } }
  fn chmax(&mut self, mut rhs: Self) -> &mut Self { if self < &mut rhs { *self = rhs; }; self }
  fn chmin(&mut self, mut rhs: Self) -> &mut Self { if self > &mut rhs { *self = rhs; }; self }
}
impl<T: Sized + PartialOrd> MyOrd for T {}