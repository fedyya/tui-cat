use crossterm::event::{read, Event, KeyCode, KeyEventKind};

#[derive(Debug)]
pub enum Key {
    ///上　↑もしくはw
    Up,
    ///下　↓もしくはs
    Down,
    ///次に進む　dもしくは→
    Next,
    /// Enter
    Enter,
    ///戻る　←
    Back,
    ///入力欄切り替え
    Change,
    ///終了 q
    Exit,
    ///移動して終了
    ExitMove,
    /// プロパティモード切り替え
    PropertyMode,
    /// 行数を表すモード
    LineMode,
    /// 最終行に移動する
    MoveLastLine,
    /// 先頭に移動する
    MoveFirstLine,
    ///例外
    None,
}

impl Key {
    pub const fn find(f: KeyCode) -> Key {
        match f {
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Right => Key::Next,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Back,
            KeyCode::Char(c) => match c {
                'a' => Key::Back,
                'd' => Key::Next,
                'q' => Key::Exit,
                'Q' => Key::ExitMove,
                'w' => Key::Up,
                's' => Key::Down,
                'e' => Key::Change,
                'p' => Key::PropertyMode,
                'l' => Key::LineMode,
                'g' => Key::MoveFirstLine,
                'G' => Key::MoveLastLine,
                _ => Key::None,
            },
            _ => Key::None,
        }
    }
}

/// キー入力から値を取得する。
/// # Exsample
/// ``` rust
/// use crate::ui::events;
/// let key: Key = event::input();
///
/// // wキーを押す
/// print!("{:?}", key);
/// // Key::Up
/// ```
#[inline]
pub fn input() -> Key {
    //! 標準入力
    //!
    //! 一文字づつ値を読み込み[Key]を返す
    if let Event::Key(f) = read().unwrap() {
        match f.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => Key::find(f.code),
            _ => Key::None,
        }
    } else {
        Key::None
    }
}
