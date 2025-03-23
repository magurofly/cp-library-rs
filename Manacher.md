# Manacher
文字列の各位置について、そこを中心とする回文長を求める。

本来は回文「半径」を求めるアルゴリズムだが、回文「長」のほうが偶数長・奇数長の回文を統一して扱えるなどの利点があるため回文長としている。

## コード
```rust
/// 列の各位置に対して、そこを中心としてできる最長回文の長さを求める
/// 列の長さが `n` とするとき、長さ `2 * n - 1` の Vec を返す
/// - `2 * i`: `i` 番目の文字を中心とする奇数長の回文
/// - `2 * i + 1`: `i` 番目の文字の直後を中心とする偶数長の回文
fn manacher<T: PartialEq>(slice: &[T]) -> Vec<usize> {
    // slice の各文字の間にセパレータを入れた文字列を考える（例： "hello" -> "h.e.l.l.o"）
    let n = slice.len() * 2 - 1;
    let mut rad = vec![0; n];
    let mut c = 0;
    let mut r = 0;
    while c < n {
        // c を中心とする最長回文半径を愚直に求める
        while r <= c && c + r < n && ((c + r) % 2 == 1 || slice[(c - r) / 2] == slice[(c + r) / 2]) {
            r += 1;
        }
        rad[c] = r;
        // 最長回文の左半分に含まれる回文を右半分にコピー
        let mut k = 1;
        while k <= c && k + rad[c - k] < r {
            rad[c + k] = rad[c - k];
            k += 1;
        }
        c += k;
        r -= k;
    }
    // セパレータが端になる分をキャンセル
    let mut len = rad;
    for i in 1 .. n.saturating_sub(1) {
        len[i] -= 1;
    }
    len
}
```

## 応用
`PartialEq` を独自実装にすることでワイルドカード文字を扱うことができる。

```rust
fn main() {
    let pattern = "h?llo".chars().map(|c| {
        match c {
            '?' => WildCardPattern::WildCard,
            _ => WildCardPattern::Letter(c)
        }
    }).collect::<Vec<_>>();
    println!("{:?}", manacher(&pattern));
    //=> [1, 2, 1, 2, 3, 4, 3, 2, 1]
}

enum WildCardPattern<T> {
    Letter(T),
    WildCard,
}
impl<T: PartialEq> PartialEq for WildCardPattern<T> {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Self::WildCard, _) | (_, &Self::WildCard) => true,
            (Self::Letter(x), Self::Letter(y)) => x.eq(y),
        }
    }
}
```