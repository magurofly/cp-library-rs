# ポテンシャル付き UnionFind
重み付き UnionFind とも。

## コード

```rust
pub struct PotentializedUnionFind {
    p: Vec<isize>,
    w: Vec<i64>,
}
impl PotentializedUnionFind {
    /// n 頂点の重み付き UnionFind 木を初期化する
    pub fn new(n: usize) -> Self {
        Self {
            p: vec![-1; n],
            w: vec![0; n],
        }
    }

    /// p(v) - p(u) を求める
    pub fn diff(&self, u: usize, v: usize) -> i64 {
        self.w[v] - self.w[u]
    }

    /// 代表元を求める
    pub fn leader(&mut self, k: usize) -> usize {
        if self.p[k] < 0 {
            return k;
        }
        let p = self.p[k] as usize;
        let q = self.leader(p);
        self.w[k] += self.w[p];
        self.p[k] = q as isize;
        q
    }

    /// p(u) + d = p(v) という情報を追加する
    /// 矛盾している場合は None 、無意味な場合は Some(false) 、意味があった場合は Some(true)
    pub fn merge(&mut self, u: usize, v: usize, mut d: i64) -> Option<bool> {
        let mut x = self.leader(u);
        let mut y = self.leader(v);
        if x == y {
            if self.w[u] + d != self.w[v] {
                return None;
            }
            return Some(false);
        }
        d += self.w[u];
        d -= self.w[v];
        if self.p[x] > self.p[y] {
            std::mem::swap(&mut x, &mut y);
            d *= -1;
        }
        self.p[x] += self.p[y];
        self.p[y] = x as isize;
        self.w[y] = d;
        Some(true)
    }
}
```

## 参考
- https://ei1333.github.io/luzhiled/snippets/structure/union-find.html
