# kansuji

## 概要

漢数字の解析と変換を行うcrateである。
サポートする漢数字の桁の範囲は垓(10^20)から毛(10^-3)までとする
(<https://homepage45.net/unit/sub.htm>)

なお、大字をどこまでサポートするかは今後決めるものとする。

## 使い方

数字と文字列との間にFromトレイトとTryFromトレイト、ToStringトレイトを元にした相互変換を実現している。
ただし、オーバーフローの関係で漢数字から数字への変換はu128およびf64, f32へのみ対応している。

```rust
use kansuji::Kansuji;

let s = "百二十三兆五百四十万二";
let kansuji = Kansuji::try_from(s).unwrap();
let n: u128 = kansuji.into();
assert_eq!(n, 123000005400002);
let kansuji2 = Kansuji::from(n);
assert_eq!(s.to_string(), kansuji2.to_string());
```
---
[The MIT License](https://github.com/puripuri2100/kansuji-rs/blob/master/LICENSE)

Copyright (c) 2024 Naoki Kaneko (a.k.a. "puripuri2100")

