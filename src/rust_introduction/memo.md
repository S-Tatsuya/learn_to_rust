# 学習中に疑問に思ったことをメモに残す

## chapter17 Closure

### Closureごとの引数の違いってなんなんだろう？
`map => map(|x| x * x)`
`filter => filter(|&x| x % 2 == 1)`
`find => find(|&&x| x == 3)`

### collect()メソッドはどんな機能があるのだろう？
Rubyでは、mapメソッドと同じ機能。([参照](https://www.sejuku.net/blog/71899))
Rustのサンプルコードの動きはmapとは明らかに違う。

入門書の説明は`collectメソッドで適当なベクターに変換します。`
[Reference](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect)を読もう！！

``` Rust
let c: Vec<_> = a.iter().xip(b.iter()).collect();
```
