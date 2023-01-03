# 概要

Mo's Algorithm を用いて、静的な配列に対する区間クエリを、現在見ている区間の伸縮によって処理する。

## 実装するべき関数
- `add` or (`add_left` and `add_right`)
- `remove` or (`remove_left` and `remove_right`)
- `query`

## 実装ヒント
- 構造体を作り、 `impl Mo` する
- 構造体に配列のデータを持たせる

# コード

```Rust
pub trait Mo<T: Default> {
  /// `i` 番目の要素を追加する
  fn add(&mut self, i: usize) {
    unimplemented!();
  }

  /// `i` 番目の要素を削除する
  fn remove(&mut self, i: usize) {
    unimplemented!();
  }

  /// 区間の左端を伸ばして `i` 番目の要素を追加する
  fn add_left(&mut self, i: usize) {
    self.add(i);
  }

  /// 区間の右端を伸ばして `i` 番目の要素を追加する
  fn add_right(&mut self, i: usize) {
    self.add(i);
  }

  /// 区間の左端を縮めて `i` 番目の要素を削除する
  fn remove_left(&mut self, i: usize) {
    self.remove(i);
  }

  /// 区間の右端を縮めて `i` 番目の要素を削除する
  fn remove_right(&mut self, i: usize) {
    self.remove(i);
  }

  /// 現在見ている区間に対するクエリに答える
  fn query(&self) -> T;

  /// 長さ `n` に対する右半開区間クエリ `queries` を Mo's Algorithm によって処理する
  fn mo(&mut self, n: usize, queries: &[(usize, usize)]) -> Vec<T> {
    let q = queries.len();
    let width = (n as f64).sqrt() as usize;
    let mut left = Vec::with_capacity(q);
    let mut right = Vec::with_capacity(q);
    for &(l, r) in queries {
      assert!(l <= r && r <= n);
      left.push(l);
      right.push(r);
    }
    let mut order = (0 .. q).collect::<Vec<_>>();
    order.sort_by(|&i, &j| {
      if left[i] / width == left[j] / width {
        right[i].cmp(&(right[j]))
      } else {
        left[i].cmp(&(left[j]))
      }
    });

    let mut ans = Vec::new();
    let mut l = 0;
    let mut r = 0;
    ans.resize_with(q, T::default);
    for i in order {
      while l > left[i] {
        l -= 1;
        self.add_left(l);
      }
      while r < right[i] {
        self.add_right(r);
        r += 1;
      }
      while l < left[i] {
        self.remove_left(l);
        l += 1;
      }
      while r > right[i] {
        r -= 1;
        self.remove_right(r);
      }
      ans[i] = self.query();
    }
    ans
  }
}
```
