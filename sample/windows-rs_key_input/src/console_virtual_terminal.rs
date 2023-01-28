//! コンソールの仮想ターミナル シーケンス（Console Virtual Terminal Sequence Code assets）
//!
//! 仮想ターミナルは、出力ストリーム書き込み時にエスケープシーケンスを利用することで
//! カーソル移動・コンソールの色変更・そのほか種々の制御を実行できる。
//!
//! それらのエスケープシーケンスを定数で定義し、このモジュールにまとめる。
//!
//! なお、ここでいうエスケープシーケンスについては以下の説明を参照。
//!
//! > 参照元：[窓辺の小石 第46回 ANSI Terminal Echo](https://www.mapion.co.jp/news/column/cobs2360627-1-all/)
//! >
//! > エスケープシーケンスとは、もともとは端末装置のための制御コードで、
//! > ANSI（American National Standards Institute。米国標準協会）が
//! > ANSI X3.64(1979年)として規格化したことにちなみ、ANSIエスケープシーケンスと呼ばれることがある。
//! > また、この規格に準拠した旧DEC社の端末装置VTシリーズが広く使われたことから
//! > VTエスケープシーケンスとも呼ばれる。
//! > なお、ANSI X3.64とECMA-48（1976年。ヨーロッパの規格）は、ほぼ同じである。
//! > その後、これらの規格は、ISO/IEC 6429として国際規格となった。
//!
//! ESCコードは`\x1b`。また、`\x1b[`を規格ではCSI（Control Sequence Introducer）と称す。
//!
//! 参考リンク：
//!  
//! - <https://learn.microsoft.com/ja-jp/windows/console/console-virtual-terminal-sequences>
//! - <https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences>
//! - <https://www.mapion.co.jp/news/column/cobs2360627-1-all/>

use std::collections::HashMap;

/// コード「ESC M」
/// \nの反転操作を実行し、カーソルを 1 行上に移動し、
/// 水平位置を維持し、必要に応じてバッファーをスクロールする。
pub fn reverse_index() {
    print!("\x1bM");
}

/// コード「ESC 7」
/// カーソル位置をメモリに保存する。
pub fn save_cursor_position_in_memory() {
    print!("\x1b7");
}

/// コード「ESC 8」
/// メモリからカーソル位置を復元する。
pub fn restore_cursor_position_in_memory() {
    print!("\x1b8");
}

/// コード「ESC[\<y\>;\<x\>H」
/// カーソル位置を設定する。
pub fn set_cursor_position(x: u32, y: u32) {
    print!("\x1b[{};{}H", y, x);
}
/// コード「ESC[?25l」
/// カーソルを非表示にする。
pub fn set_cursor_invisible() {
    print!("\x1b[?25l");
}

/// コード「ESC[?25h」
/// カーソルを表示する。
pub fn set_cursor_visible() {
    print!("\x1b[?25h");
}

/// コード「ESC(0」
/// DEC の線描画モードを有効にします。
pub fn enables_dec_line_drawing_mode() {
    print!("\x1b(0");
}

/// コード「ESC(B」
/// ASCIIモードを有効にします（規定）。
pub fn enables_ascii_mode() {
    print!("\x1b(B");
}

/// 全角文字の｜─┼等をDEC Line Drawingへ変換する。
/// ただし、Windowsターミナル上は普通に全角入力した場合と同じ表示になるので、あまりこの関数の意味はない。
/// | Code | ASCII | DEC Line Drawing |
/// | ---  | ---   | ---              |
/// | 0x6a | j     | ┘               |
/// | 0x6b | k     | ┐               |
/// | 0x6c | l     | ┌               |
/// | 0x6d | m     | └               |
/// | 0x6e | n     | ┼               |
/// | 0x71 | q     | ─               |
/// | 0x74 | t     | ├               |
/// | 0x75 | u     | ┤               |
/// | 0x76 | v     | ┴               |
/// | 0x77 | w     | ┬               |
/// | 0x78 | x     | │               |
pub fn draw_dec_line(str: &str) {
    let dec_line_drawing_ascii = vec![
        ("┘", "j"),
        ("┐", "k"),
        ("┌", "l"),
        ("└", "m"),
        ("┼", "n"),
        ("─", "q"),
        ("├", "t"),
        ("┤", "u"),
        ("┴", "v"),
        ("┬", "w"),
        ("│", "x"),
    ]
    .into_iter()
    .collect::<HashMap<&_, &_>>();

    // DEC線描画モードを有効にする。
    enables_dec_line_drawing_mode();

    for c in str.chars() {
        let s: &str = &c.to_string();
        let line = match dec_line_drawing_ascii.get(s) {
            Some(v) => *v,
            None => s,
        };
        print!("{}", line);
    }

    // ASCIIモードを有効にする。
    enables_ascii_mode();
}

/// コード「ESC]0;<string><ST>」（<ST> = \x1b\x5c）
/// 引数で指定した値をウィンドウタイトルに設定する。
pub fn set_window_title(title: &str) {
    print!("\x1b]0;{}\x1b\x5c", title);
}

/// コード「ESC[?1049h」
/// 新しい代替画面バッファーに切り替えます。
pub fn use_alternate_screen_buffer() {
    print!("\x1b[?1049h");
}

/// コード「ESC[?1049l」
/// メイン バッファーに切り替えます。
pub fn use_main_screen_buffer() {
    print!("\x1b[?1049l");
}

/// コード「ESC[2J」
/// 画面をクリアします。
pub fn clear_screen() {
    print!("\x1b[2J");
}
