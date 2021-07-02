# [ModInt](https://github.com/magurofly/cp-library-rs/blob/main/src/modint.rs)

有限体のライブラリ。

## 機能

* 四則演算（ `+`, `-`, `*`, `/`, `+=`, `-=`, `*=`, `/=` ）
* 任意 mod での逆元の計算 `.inv()`
* 冪乗 `.pow(n)`
* `FromStr`, `Display`

## 使い方

### デフォルトの mod を設定

```rust
use modint::ModInt;

// mod を設定
ModInt::set_mod(998244353);

// 入力
proconio::input! {
  k: ModInt<i64>,
}

// 出力
println!("{}", k * 2 + 1);
```

### 異なる mod を使う

```rust
let x = ModInt::with_mod(100, 998244353);
let y = ModInt::with_mod(200, 998244853);
```

## Verify

まだ
