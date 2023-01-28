use windows::Win32::UI::Input::KeyboardAndMouse;
use std::{thread, time};

enum KeyState {
    Pressed,
    Unpressed,
}

fn main() {
    let key_esc = 0x1b; // esc
    let key_space = 0x20; // [space]
    let key_q = 0x51; // q
    let key_left_arrow = 0x25; // ←
    let key_up_arrow = 0x26; // ↑
    let key_right_arrow = 0x27; // →
    let key_down_arrow = 0x28; // ↓

    let player1 = "@";
    let mut player1_pos_x = 1;
    let mut player1_pos_y = 1;

    print!("\x1b[?1049h"); // 代替画面
    print!("\x1b[?25l"); // カーソルを消す
    print!("\x1b[0;0H"); // カーソル位置を0,0にする。
    loop {
        let key_state_esc = get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_esc) });
        let key_state_space =
            get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_space) });
        let key_state_q = get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_q) });
        let key_state_left_arrow =
            get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_left_arrow) });
        let key_state_up_arrow =
            get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_up_arrow) });
        let key_state_right_arrow =
            get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_right_arrow) });
        let key_state_down_arrow =
            get_key_state(unsafe { KeyboardAndMouse::GetAsyncKeyState(key_down_arrow) });

        if let KeyState::Pressed = key_state_left_arrow {
            if player1_pos_x == 1 {
                player1_pos_x = 1;
            } else {
                player1_pos_x -= 1;
            }
        }

        if let KeyState::Pressed = key_state_up_arrow {
            if player1_pos_y == 1 {
                player1_pos_y = 1;
            } else {
                player1_pos_y -= 1;
            }
        }

        if let KeyState::Pressed = key_state_right_arrow {
            if player1_pos_x == 80 {
                player1_pos_x = 80;
            } else {
                player1_pos_x += 1;
            }
        }

        if let KeyState::Pressed = key_state_down_arrow {
            if player1_pos_y == 30 {
                player1_pos_y = 30;
            } else {
                player1_pos_y += 1;
            }
        }

        // カーソル位置を(x,y) = (1,30) にする。原点は(1,1)。y = 30がWindows 11のターミナルのデフォルト最終行っぽい。
        print!("\x1b[30;1H");
        print!("esc: {:3}  ", get_key_state_str(&key_state_esc));
        print!("q: {:3}  ", get_key_state_str(&key_state_q));
        print!("[sp]: {:3}  ", get_key_state_str(&key_state_space));
        print!("← : {:3}  ", get_key_state_str(&key_state_left_arrow));
        print!("↑ : {:3}  ", get_key_state_str(&key_state_up_arrow));
        print!("→ : {:3}  ", get_key_state_str(&key_state_right_arrow));
        print!("↓ : {:3}  ", get_key_state_str(&key_state_down_arrow));
        print!("x:{:3}  ", player1_pos_x);
        print!("y:{:3}  ", player1_pos_y);

        print!("\x1b[{};{}H", player1_pos_y, player1_pos_x);
        print!("{}", player1);

        if let KeyState::Pressed = key_state_esc {
            print!("\x1b[?25h"); // カーソルを出す。
            print!("\x1b[?1049l"); // メイン画面
            break;
        }

        // 程よくplayerを動かすためにスリープを挟む。
        thread::sleep(time::Duration::from_millis(10));
    }
}

// https://liclog.net/getasynckeystate-function-vba-macro-catia-v5/
// https://thom.hateblo.jp/entry/2019/03/10/125326
fn get_key_state(target_key_state: i16) -> KeyState {
    const KEY_PRESSED: i16 = -32768;
    if target_key_state & KEY_PRESSED == KEY_PRESSED {
        KeyState::Pressed
    } else {
        KeyState::Unpressed
    }
}

fn get_key_state_str(key_state: &KeyState) -> &'static str {
    match key_state {
        KeyState::Pressed => "on",
        KeyState::Unpressed => "off",
    }
}
