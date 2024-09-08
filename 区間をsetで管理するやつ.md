# 区間をsetで管理するやつ

## リファレンス

### `pub fn new() -> Self`
初期化する

### `pub fn insert(&mut self, l: T, r: T)`
半開区間 `l .. r` を追加する
計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）

### `pub fn contains(&self, x: &T) -> bool`
`x` がこれまで追加された区間のうちどれかに含まれているか判定
計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）

### `pub fn containing_range(&self, x: &T) -> Option<(&T, &T)>`
`x` を含む区間を返す（この関数で返される区間は、これまでに追加された区間の論理和）
計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  
### `pub fn nearest_left_range(&self, x: &T) -> Option<(&T, &T)>`
`x` を含む区間もしくは `x` の左側にある区間のうち最も近いものを返す
計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）

### `pub fn nearest_right_range(&self, x: &T) -> Option<(&T, &T)>`
`x` よりも右側にある区間のうち最も近いものを返す（`x` を含む区間は除く）
計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）

## コード
```rs
#[derive(Clone, Default, Debug)]
pub struct RangeSet<T>(std::collections::BTreeMap<T, T>);
impl<T: Clone + Ord> RangeSet<T> {
  pub fn new() -> Self { Self(std::collections::BTreeMap::new()) }

  /// 半開区間 `l .. r` を追加する
  /// 計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  pub fn insert(&mut self, mut l: T, mut r: T) {
    // 前とつなげる
    if let Some((l0, r0)) = self.0.range(.. &l).next_back().filter(|pair| pair.1 >= &l ).map(|(l0, r0)| (l0.clone(), r0.clone()) ) {
      self.0.remove(&l0);
      l = l.min(l0);
      r = r.max(r0);
    }
    // 後とつなげる
    loop {
      if let Some((l0, r0)) = self.0.range(&l ..).next().filter(|pair| pair.0 <= &r ).map(|(l0, r0)| (l0.clone(), r0.clone()) ) {
        self.0.remove(&l0);
        l = l.min(l0);
        r = r.max(r0);
      } else {
        break;
      }
    }
    self.0.insert(l, r);
  }

  /// `x` を含む区間もしくは `x` の左側にある区間のうち最も近いものを返す
  /// 計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  pub fn nearest_left_range(&self, x: &T) -> Option<(&T, &T)> {
    self.0.range(..= x).next_back()
  }

  /// `x` よりも右側にある区間のうち最も近いものを返す（`x` を含む区間は除く）
  /// 計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  pub fn nearest_right_range(&self, x: &T) -> Option<(&T, &T)> {
    use std::ops::Bound::*;
    self.0.range((Excluded(x), Unbounded)).next()
  }

  /// `x` を含む区間を返す（この関数で返される区間は、これまでに追加された区間の論理和）
  /// 計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  pub fn containing_range(&self, x: &T) -> Option<(&T, &T)> {
    self.nearest_left_range(x).filter(|pair| pair.1 > x )
  }

  /// `x` がこれまで追加された区間のうちどれかに含まれているか判定
  /// 計算量: $O(\log n)$ （ただし $n$ はこれまでに追加した要素数）
  pub fn contains(&self, x: &T) -> bool {
    self.containing_range(x).is_some()
  }
}
```
