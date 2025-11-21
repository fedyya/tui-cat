use crate::components::check_property::Property;

use crossterm::terminal;
use ratatui::{
    text::{Span, Text},
    widgets::ListState,
};
use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{read_to_string, ReadDir},
    path::PathBuf,
};

use crate::ui::syntax;
use crate::Key;

pub struct Events<'a> {
    ///ディレクトリ内のファイル・フォルダを取得
    /// [フォルダ, ファイル]の順
    pub items: [Vec<OsString>; 2],
    ///パス
    pub path: PathBuf,
    ///選択肢の保存場所
    pub state: ListState,
    ///サブ切り替え時の選択肢の仮保存場所
    /// - true  <br>サブモードon
    /// - false <br>サブモードoff
    pub submode: bool,
    ///サブ時の値保存場所
    pub substate: (u16, u16),
    ///キーボードで入力された文字列
    pub key: Key,
    ///ファイルの中身
    pub data: Text<'a>,
    /// プロパティ
    pub property: Option<Property>,
    /// プロパティモード
    pub property_mode: bool,
    /// 行数を表すモード
    pub line_mode: bool,
}

impl<'a> Events<'_> {
    pub fn new() -> Events<'a> {
        //カウントディレクトリを取得
        let dir = env::current_dir().unwrap().read_dir().unwrap();

        let mut eve = Events {
            items: search_directory(dir),
            path: env::current_dir().unwrap(),
            state: ListState::default(),
            submode: false,
            substate: (0, 0),
            key: Key::None,
            data: Text::raw(""),
            property: None,
            property_mode: false,
            line_mode: false,
        };

        eve.property = Property::new(eve.path.as_path());
        eve
    }

    ///選択を一つ次に進める
    pub fn next(&mut self) {
        match self.submode {
            true => {
                // submode時の下限サイズ設定
                if (self.substate.0 as usize) < self.limit_down_size() {
                    self.substate.0 = self.substate.0.saturating_add(1);
                }
            }
            false => {
                if !self.items.is_empty() {
                    let i = match self.state.selected() {
                        Some(i) => {
                            if i >= self.items.concat().len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    self.state.select(Some(i));
                }
            }
        }
    }

    ///選択を一つ前に戻す
    #[inline]
    pub fn back(&mut self) {
        match self.submode {
            true => {
                self.substate.0 = self.substate.0.saturating_sub(1);
            }
            false => {
                if !self.items.is_empty() {
                    let i = match self.state.selected() {
                        Some(i) => {
                            if i == 0 {
                                self.items.concat().len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    self.state.select(Some(i));
                }
            }
        }
    }

    /// substateの値を1増やす
    #[inline]
    pub fn subnext(&mut self) {
        self.substate.1 = self.substate.1.saturating_add(1);
    }

    /// substateの値を1減らす
    #[inline]
    pub fn subback(&mut self) {
        self.substate.1 = self.substate.1.saturating_sub(1);
    }

    ///入力欄切り替え
    #[inline]
    pub fn change(&mut self) {
        self.submode = !self.submode;
    }

    ///選択したファイル・フォルダを開く
    #[inline]
    pub fn open_file(&mut self) {
        //フォルダの中身チェック
        if !self.items.concat().is_empty() {
            let next = self.state.selected().unwrap();
            let file = &self.items.concat()[next];

            // 開いているパスが同じフォルダの場合早期リターン
            if self.path.iter().last() == Some(file) {
                return;
            }

            if self.path.is_file() {
                self.path.pop();
            }

            self.path.push(file);

            //開いたものがフォルダの場合<ReadDir>を返す
            if self.path.is_dir() {
                let dir = match self.path.as_path().read_dir() {
                    Ok(f) => f,
                    Err(err) => {
                        self.data = Text::from(err.to_string());
                        self.path.pop();
                        self.path.as_path().read_dir().unwrap()
                    }
                };

                self.items = search_directory(dir);
                self.reset_state();
            } else {
                //開いたものがファイルの場合
                //開けるか確認
                let text = read_to_string(self.path.as_path())
                    .unwrap_or("ファイルが開けませんでした".to_string());

                // ファイルの拡張子を取得
                let extension = self
                    .path
                    .extension()
                    .unwrap_or(OsStr::new("txt"))
                    .to_str()
                    .unwrap_or("txt");
                self.data = syntax::hylight(text, extension);
                self.property = Property::new(&self.path);

                // linemodeを初期化
                self.line_mode = false;
            }
            self.reset_substate();
        }
    }

    ///現在開いているパスの一つ前のフォルダに戻る
    #[inline]
    pub fn back_file(&mut self) {
        //! # Example
        //! ``` rust
        //! // パスが以下の通りなら
        //! // C
        //! // |--folder_01
        //! //    |--folder_02
        //! //       |--folder_03 <- 現在値1
        //! //           |--file.txt <- 現在値2
        //!
        //! let mut event: Events = search_dir::Events::new();
        //! event.back_file();
        //!
        //! // C
        //! // |--folder_01
        //! //    |--folder_02 <- 現在値1, 現在値2
        //! ```

        // 開いているものがファイルの場合はもう一階層戻る
        if self.path.is_file() {
            self.path.pop();
        }

        let this_folder: OsString = self.path.iter().last().unwrap().to_os_string();

        self.path.pop();

        self.items = search_directory(self.path.as_path().read_dir().unwrap());

        // 選択肢を現在のフォルダに選択
        if self.path.file_name().is_some() {
            let this_state = self.items[0]
                .iter()
                .position(|f| Into::<OsString>::into(f) == this_folder)
                .unwrap();
            self.state.select(Some(this_state));
        }
    }

    /// カウントディレクトリを変更する
    pub fn _move_dir(&mut self) {
        if self.path.is_file() {
            self.path.pop();
        }

        std::process::Command::new("cd")
            .env("PATH", self.path.to_str().unwrap())
            .spawn()
            .unwrap();
    }

    /// [Events::state]\(選択\)を０にする
    #[inline]
    fn reset_state(&mut self) {
        self.state.select(Some(0));
    }

    /// [Events::substate]を(0,0)に設定する
    #[inline]
    pub fn reset_substate(&mut self) {
        self.substate = (0, 0);
    }

    /// linemode切り替え
    pub fn change_linemode(&mut self) {
        self.line_mode = !self.line_mode;
        let line = self.data.lines.len();

        if self.line_mode {
            // 行数番号を表す"001 |"を挿入する
            // ファイルの行数から桁数を求める
            let digit_count = (line.ilog10() + 1) as usize;

            for i in 0..line {
                let line_text = format!("{:0>digit_count$} |", i + 1);
                let line_span = Span::raw(line_text);
                self.data.lines[i].spans.insert(0, line_span);
            }
        } else {
            // 行列番号のspanを削除する
            for i in 0..line {
                self.data.lines[i].spans.remove(0);
            }
        }
    }

    /// submodeが[true]のときファイルの先頭に移動する
    #[inline]
    pub fn move_first_line(&mut self) {
        if self.submode {
            self.substate = (0, 0);
        }
    }

    /// submodeが[true]のときファイルの行末に移動する
    #[inline]
    pub fn move_last_line(&mut self) {
        if self.submode {
            self.substate = (self.limit_down_size() as u16, 0);
        }
    }

    /// ターミナル上に表示できる最大の下げ幅を取得
    #[inline]
    fn limit_down_size(&self) -> usize {
        self.data
            .lines
            .len()
            .saturating_sub(terminal::size().unwrap().1 as usize - 6)
    }
}

#[inline]
/// 引数[ReadDir]からフォルダ・ファイルを取得し、\[Vec\<OsString\>; 2\]を返す。<br>
/// 順番は\[フォルダ、ファイル\]の順
pub fn search_directory(dir: ReadDir) -> [Vec<OsString>; 2] {
    //! # Examples
    //! ```rust
    //! // カウントディレクトリに[a(file),b(folder),c(file),d(folder)]が入っているとすると
    //! let path: ReadDir = std::env::current_dir()
    //!     .unwrap()
    //!     .read_dir()
    //!     .unwrap();
    //!
    //! let mut list: Vec<String> = search_directory(path);
    //! asset_eq!(list, Vec[b,d,a,c]);
    //! ```
    let mut folder: Vec<OsString> = Vec::with_capacity(30);
    let mut file: Vec<OsString> = Vec::with_capacity(50);

    for data in dir {
        let path = data.unwrap().path();

        let name = path.file_name().unwrap().to_os_string();

        if path.is_dir() {
            folder.push(name);
        } else {
            file.push(name);
        }
    }

    [folder, file]
}

#[test]
fn back_file_test() {
    let mut x = Events::new();
    x.back_file();

    let mut a = env::current_dir().unwrap();
    a.pop();
    assert_eq!(x.path, a);
}
