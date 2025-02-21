# Rustの学習ノート（イディオム編）

## 型変換

```rust
aaaa.to_string().parse::<i32>().unwrap();
println!("[{}{}], file!(), line!());
```

## 処理時間の計測

```rust
let now = std::time::Instant::now;
println!("[{}:{}] elapsed time: {:?}", file!(), line!(), now.elapsed());
```

## ファイル操作

### カレントディレクトリのパスを取得する

```rust
std::env;
emv::current_di().unwrap().display();
```

### ファイル読み書き

```rust
use std::fs::File;
use::io::{Read, Write};
use::path::{Path, PathBuf};
use std::process::Command;

let mut input_file = File::open(&path).expect("can't open input file.");
let file_length = input_file.metadata().unwrap().len() as usize;  // ファイルサイズを取得する方法。
let mut buf = vec![0;input_file_length];
input_file.read(&mut buf).expect("\x1b[31mBuffer over flow!\x1b[0m]"); // bufへファイルの内容を書き込む方法。

let mut output_file = File::create(&output_file_path).expect("xxxxx");
output_file.write_all(&buf[0...100]);
output_file.write_all(&buf[100..])

// 読み込んでストリームに渡す例。
let mut source_file = fs::File::open(&source_path).unwrap();
let mut source_bytes = Vec::new();
source_file.read_to_end(&mut source_bytes).unwrap();

let mut stream = Cursor::new(source.bytes.to_vec());
let _ = builder.set_thumbnail(format_mime, &mut stream);

// コマンドを実行する方法。C言語のsystem()やVBAのexecやrunに相当する。
let output = Command::new("openssl").args["sha256", -"binary", target_path]).output().expect("faild to run openssl sha256");
let mut f = File::create(path).expect("Can't open file");
f.write_all(&output.stdout).expect("Can't write file.");
```

## Rust FFI

```rust
std::ffi::{c_void, CString}

#[repr(C)]
struct RawAsset {
    value: *mut c_void, // 配列・文字列リテラルの先頭の要素へのポインタ。
    length: usize       // 配列・文字列リテラルの長さ
}

pub struct RawaaData {
    file_path: RawAsset,
}

// 普通の数値はunsafeブロックで囲まなくてやり取り可能。

let safe_file_path = String::from_utf8_lossy(unsafe{
    let obj = & aaa.filepash;
    std::slice::from_raw_parts(obj.value as *const u8, obj.length)
});
```
