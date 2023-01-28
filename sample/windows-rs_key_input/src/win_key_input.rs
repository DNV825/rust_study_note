use std::{thread, time};
use windows::Win32::UI::Input::KeyboardAndMouse;

/// キーが押下されているかを表す列挙体。
#[derive(Debug)]
pub enum KeyState {
    /// キーが押された。
    Pressed,

    /// キーが押しっぱなし。
    Pressing,

    /// 押されていない（離された）。
    Unpressed,
}

/// キーの種別の定数を持つ構造体。
///
/// 参考：
/// https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
#[derive(Debug)]
pub struct KeyKind {}

impl KeyKind {
    pub const ESC: i32 = 0x1b;
    pub const SPACE: i32 = 0x20;
    pub const Q: i32 = 0x51;
    pub const LEFT_ARROW: i32 = 0x25;
    pub const UP_ARROW: i32 = 0x26;
    pub const RIGHT_ARROW: i32 = 0x27;
    pub const DOWN_ARROW: i32 = 0x28;
}

/// あるキーの押下状態を要素に持つ構造体。
#[derive(Debug)]
pub struct KeyInput {
    /// キーの種別を指定する。指定する値は列挙体KeyKindの定数とする。
    key_kind: i32,

    /// キーの押下状態を指定する。
    key_state: KeyState,
}

impl KeyInput {
    pub fn new(key_kind: i32) -> KeyInput {
        KeyInput {
            key_kind: key_kind,
            key_state: KeyState::Unpressed,
        }
    }

    /// キーの状態を取得する。
    ///
    /// 参考：
    /// https://liclog.net/getasynckeystate-function-vba-macro-catia-v5/
    /// https://thom.hateblo.jp/entry/2019/03/10/125326
    pub fn get_key_state(&mut self) -> &mut KeyState {
        unsafe {
            const KEY_PRESSED: i16 = -32768;
            if KeyboardAndMouse::GetAsyncKeyState(self.key_kind) & KEY_PRESSED == KEY_PRESSED {
                self.key_state = match self.key_state {
                    KeyState::Unpressed => KeyState::Pressed,
                    _ => KeyState::Pressing,
                }
            } else {
                self.key_state = KeyState::Unpressed;
            }
            // print!("{:?}", self.key_state);
            &mut self.key_state
        }
    }

    pub fn get_key_state_str(&self) -> &'static str {
        match self.key_state {
            KeyState::Pressed => "pressed",
            KeyState::Pressing => "pressing",
            KeyState::Unpressed => "",
        }
    }
}
