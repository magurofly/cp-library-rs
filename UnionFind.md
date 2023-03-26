# [UnionFind](https://github.com/magurofly/cp-library-rs/blob/main/src/unionfind.rs)

使い方はACLとほぼ同じです。

* `UnionFind::new(n)`: `n` 頂点の素集合森を作成する
* `leader(i)`: `i` が属する集合の代表元
* `size(i)`: `i` が属する集合の大きさ
* `merge(i, j)`: `i` が属する集合と `j` が属する集合を結合する
* `same(i, j)`: `i` と `j` が同じ集合に属するか判定
* `groups()`: すべての集合を返す

## 半群が乗るバージョン

[ABC183 F - Confluence でテスト済](https://atcoder.jp/contests/abc183/submissions/40080663)

```rust
pub struct UnionFindWithSemigroup<T, Op, Id> {
  n: usize,
  p: Vec<isize>,
  x: Vec<T>,
  op: Op,
  id: Id,
}
impl<T, Op: FnMut(T, T) -> T, Id: FnMut() -> T> UnionFindWithSemigroup<T, Op, Id> {
  pub fn new(n: usize, op: Op, mut id: Id) -> Self { let mut x = vec![]; x.resize_with(n, || id() ); Self { n, p: vec![-1; n], x, op, id } }
  pub fn leader(&mut self, mut i: usize) -> usize { let k = self.p[i]; if k >= 0 { let j = self.leader(k as usize); self.p[i] = j as isize; i = j; }; i }
  pub fn get(&mut self, mut i: usize) -> &T { i = self.leader(i); &self.x[i] }
  pub fn get_mut(&mut self, mut i: usize) -> &mut T { i = self.leader(i); &mut self.x[i] }
  pub fn set(&mut self, mut i: usize, x: T) { i = self.leader(i); self.x[i] = x; }
  pub fn merge(&mut self, mut i: usize, mut j: usize) -> bool { i = self.leader(i); j = self.leader(j); i != j && { if i > j { let k = i; i = j; j = k; }; let a = std::mem::replace(&mut self.x[i], (self.id)()); let b = std::mem::replace(&mut self.x[j], (self.id)()); self.x[i] = (self.op)(a, b); self.p[i] += self.p[j]; self.p[j] = i as isize; true } }
  pub fn same(&mut self, i: usize, j: usize) -> bool { self.leader(i) == self.leader(j) }
  pub fn size(&mut self, mut i: usize) -> usize { i = self.leader(i); -self.p[i] as usize }
  pub fn groups(&mut self) -> Vec<Vec<usize>> { let mut s = vec![vec![]; self.n]; for i in 0 .. self.n { s[self.leader(i)].push(i) }; s.into_iter().filter(|g| g.len() > 0 ).collect() }
}
```
