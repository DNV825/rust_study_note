# 仮想ターミナルの学習ノート

## コンソールの仮想ターミナルシーケンス

<https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences>
<https://learn.microsoft.com/ja-jp/windows/console/console-virtual-terminal-sequences>
<https://www.mapion.co.jp/news/column/cobs2360627-1-all/>

```rust
fn main() {
    println!("Hello, world!");
    println!("\x1b[AHello, vt world!");
    
    let esc = "\x1b";
    println!("Hello, World2!");
    println!("{}[AHello, vt world2!",esc);
}
```

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\rust_grammer_sample.exe`
Hello, vt world!
Hello, vt world2!
```
