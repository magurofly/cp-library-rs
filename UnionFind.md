# [UnionFind](https://github.com/magurofly/cp-library-rs/blob/main/src/unionfind.rs)

使い方はACLとほぼ同じです。

* `UnionFind::new(n)`: `n` 頂点の素集合森を作成する
* `leader(i)`: `i` が属する集合の代表元
* `size(i)`: `i` が属する集合の大きさ
* `merge(i, j)`: `i` が属する集合と `j` が属する集合を結合する
* `same(i, j)`: `i` と `j` が同じ集合に属するか判定
* `groups()`: すべての集合を返す
