//! rust_grammer_sample
//! oooo
mod console_virtual_terminal;
mod win_key_input;

use console_virtual_terminal as cvt;
use std::{thread, time};
use win_key_input as keyboard;
use win_key_input::{KeyKind, KeyState};

// https://crates.io/crates/termion
// https://docs.rs/termion/2.0.1/termion/
// https://qiita.com/hatoo@github/items/905a19a98876e7446edf
fn main() {
    let player1 = "@";
    let mut player1_pos_x = 1;
    let mut player1_pos_y = 1;

    cvt::use_alternate_screen_buffer(); // 代替画面にする。
    cvt::set_window_title("console tetris"); // 代替画面にタイトルを表示する。
    cvt::set_cursor_invisible(); // カーソル表示を消す。
    cvt::set_cursor_position(0, 0); // カーソル位置を0, 0にする。

    let mut esc = keyboard::KeyInput::new(KeyKind::ESC);
    let mut space = keyboard::KeyInput::new(KeyKind::SPACE);
    let mut q = keyboard::KeyInput::new(KeyKind::Q);
    let mut left_arrow = keyboard::KeyInput::new(KeyKind::LEFT_ARROW);
    let mut up_arrow = keyboard::KeyInput::new(KeyKind::UP_ARROW);
    let mut right_arrow = keyboard::KeyInput::new(KeyKind::RIGHT_ARROW);
    let mut down_arrow = keyboard::KeyInput::new(KeyKind::DOWN_ARROW);

    loop {
        if let KeyState::Pressed = left_arrow.get_key_state() {
            if player1_pos_x == 1 {
                player1_pos_x = 1;
            } else {
                player1_pos_x -= 1;
            }
        }

        if let KeyState::Pressed = up_arrow.get_key_state() {
            if player1_pos_y == 1 {
                player1_pos_y = 1;
            } else {
                player1_pos_y -= 1;
            }
        }

        if let KeyState::Pressed = right_arrow.get_key_state() {
            if player1_pos_x == 80 {
                player1_pos_x = 80;
            } else {
                player1_pos_x += 1;
            }
        }

        if let KeyState::Pressed = down_arrow.get_key_state() {
            if player1_pos_y == 30 {
                player1_pos_y = 30;
            } else {
                player1_pos_y += 1;
            }
        }

        // カーソル位置を(x,y) = (1,30) にする。原点は(1,1)。y = 30がWindows 11のターミナルのデフォルト最終行っぽい。
        cvt::set_cursor_position(60, 1);
        print!("esc   : {:9}  ", esc.get_key_state_str());
        cvt::set_cursor_position(60, 2);
        print!("[sp]  : {:9}  ", space.get_key_state_str());
        cvt::set_cursor_position(60, 3);
        print!("q     : {:9}  ", q.get_key_state_str());
        cvt::set_cursor_position(60, 4);
        print!("←     : {:9}  ", left_arrow.get_key_state_str());
        cvt::set_cursor_position(60, 5);
        print!("↑     : {:9}  ", up_arrow.get_key_state_str());
        cvt::set_cursor_position(60, 6);
        print!("→     : {:9}  ", right_arrow.get_key_state_str());
        cvt::set_cursor_position(60, 7);
        print!("↓     : {:9}  ", down_arrow.get_key_state_str());
        cvt::set_cursor_position(60, 8);
        print!("(x, y):({:2}, {:2})", player1_pos_x, player1_pos_y);

        cvt::set_cursor_position(player1_pos_x, player1_pos_y);
        print!("{}", player1);

        if let KeyState::Pressed = esc.get_key_state() {
            cvt::set_cursor_visible(); // カーソルを出す。
            cvt::use_main_screen_buffer(); // メイン画面
            break;
        }

        // 程よくplayerを動かすためにスリープを挟む。
        thread::sleep(time::Duration::from_millis(10));
    }
}
