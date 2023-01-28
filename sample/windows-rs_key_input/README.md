# windows-rs key input

コンソールでキー入力を受け付けて、入力されたキーをコンソールへ表示する。
以下の情報を参考にした。

windowsクレート

- Rust for Windows
  <https://github.com/microsoft/windows-rs>
- Crate Windows
  <https://microsoft.github.io/windows-docs-rs/doc/windows/index.html>

win32api

- キーボード入力 (はじめに Win32 および C++ を使用)
  <https://learn.microsoft.com/ja-jp/windows/win32/learnwin32/keyboard-input>
- Windows 用 Rust と windows クレート
  <https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/rust-for-windows>
- GetKeyStateとGetAsyncKeyStateの違いは？どう使い分ける？
  <http://pinako.blog33.fc2.com/blog-entry-98.html>

マウス

- GetAsyncKeyState 関数 (winuser.h)
  <https://learn.microsoft.com/ja-jp/windows/win32/api/winuser/nf-winuser-getasynckeystate>
- Function windows::Win32::UI::Input::KeyboardAndMouse::GetAsyncKeyState
  <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/fn.GetAsyncKeyState.html>
- どのキーが押されたのかをVBAで判定する（GetAsyncKeyState）
  <https://vbabeginner.net/getasynckeystate/>

キーボード

- GetKeyState 関数 (winuser.h)
  <https://learn.microsoft.com/ja-jp/windows/win32/api/winuser/nf-winuser-getkeystate>
- Function windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState
  <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Input/KeyboardAndMouse/fn.GetKeyState.html>
- 仮想キー コード
  <https://learn.microsoft.com/ja-jp/windows/win32/inputdev/virtual-key-codes>

rust

- The Rust Reference - External blocks
  <https://doc.rust-lang.org/reference/items/external-blocks.html>
- Rustでコマンドラインアプリでキーが押されたかの判定しつつ、別の処理も進めるには
  <https://abrakatabura.hatenablog.com/entry/2017/09/20/065024>
