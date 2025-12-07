```rust
pub struct IntervalSet<K: Clone + Ord> {
  map: std::collections::BTreeMap<K, K>,
}
impl<K: Clone + Ord> IntervalSet<K> {
  pub fn new() -> Self {
    Self {
      map: std::collections::BTreeMap::new(),
    }
  }

  /// 区間の個数
  pub fn len(&self) -> usize {
    self.map.len()
  }

  /// 区間のイテレータ
  pub fn intervals(&self) -> std::collections::btree_map::Iter<'_, K, K> {
    self.map.iter()
  }

  /// 区間 [l, r) と交差する区間を `Vec` として返す
  /// `touch`: `true` なら接するだけの場合も取り除く
  pub fn intersecting(&self, l: &K, r: &K, touch: bool) -> Vec<(&K, &K)> {
    let mut result = vec![];
    let mut search_r = r;
    while let Some((l0, r0)) = self.map.range(..= search_r).next_back() {
      if l < r0 || touch && l == r0 {
        result.push((l0, r0));
        search_r = l0;
      } else {
        break;
      }
    }
    result.reverse();
    search_r = r;
    while let Some((l0, r0)) = self.map.range(search_r ..).next() {
      if l0 < r || touch && l0 == r {
        result.push((l0, r0));
        search_r = r0;
      } else {
        break;
      }
    }
    result
  }

  /// 区間 [l, r) に包含される区間を `Vec` として返す
  pub fn contained(&self, l: &K, r: &K) -> Vec<(&K, &K)> {
    let mut result = vec![];
    while let Some((l0, r0)) = self.map.range(l ..= r).next() {
      if r0 <= r {
        result.push((l0, r0));
      } else {
        break;
      }
    }
    result
  }

  /// 区間 [l, r) を包含する区間があれば `Some` として返す
  pub fn contains(&self, l: &K, r: &K) -> Option<(&K, &K)> {
    let (l0, r0) = self.map.range(..= l).next_back()?;
    if r <= r0 {
      Some((l0, r0))
    } else {
      None
    }
  }

  /// 点 x を包含する区間もしくは、点 x よりも左にあり最も近い区間を返す
  pub fn nearest_left_range(&self, x: &K) -> Option<(&K, &K)> {
    self.map.range(..= x).next_back()
  }

  /// 点 x よりも右にあり最も近い区間を返す（点 x を包含する場合は含めない）
  pub fn nearest_right_range(&self, x: &K) -> Option<(&K, &K)> {
    use std::ops::Bound::*;
    self.map.range((Excluded(x), Unbounded)).next()
  }

  /// 区間 [l, r) を追加する
  pub fn insert(&mut self, mut l: K, mut r: K) {
    let intersecting = self.intersecting(&l, &r, true);
    if !intersecting.is_empty() {
      l = l.min(intersecting[0].0.clone());
      r = r.max(intersecting[intersecting.len() - 1].1.clone());
    }
    for l in intersecting.iter().map(|lr| lr.0.clone() ).collect::<Vec<_>>() {
      self.map.remove(&l);
    }
    self.map.insert(l, r);
  }

  /// 区間 [l, r) を削除する
  /// [l, r) と重なる部分はすべて消える
  pub fn remove(&mut self, l: &K, r: &K) {
    for l in self.contained(l, r).iter().map(|lr| lr.0.clone() ).collect::<Vec<_>>() {
      self.map.remove(&l);
    }
    if let Some((_, r0)) = self.map.range_mut(.. l).next_back() {
      if l < r0 {
        *r0 = l.clone();
      }
    }
    if let Some((l0, _)) = self.map.range(l ..= r).next_back() {
      let l0 = l0.clone();
      let r0 = self.map.remove(&l0).unwrap();
      self.map.insert(r.clone(), r0);
    }
  }
}

```
