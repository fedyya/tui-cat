mod components;
mod ui;

use crate::ui::{
    draw::draw,
    events::{self, Key},
    search_dir,
};
use crossterm::terminal;
use std::env;

fn main() {
    // CLI 引数の簡易パース: --space N
    let args: Vec<String> = env::args().collect();
    let mut space: usize = 4;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--space" | "-s" => {
                if i + 1 < args.len() {
                    if let Ok(v) = args[i + 1].parse::<usize>() {
                        space = v;
                    }
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    //初期設定
    let mut list = search_dir::Events::new(space);

    list.next();
    //rawモードon
    terminal::enable_raw_mode().unwrap();

    loop {
        draw(&mut list);

        //入力
        list.key = events::input();

        // サブモード時の処理
        if list.submode {
            match list.key {
                Key::Exit | Key::ExitMove => break,
                Key::Up => {
                    if !list.property_mode {
                        list.back()
                    }
                }
                Key::Down => {
                    if !list.property_mode {
                        list.next()
                    }
                }
                Key::Change => list.change(),
                Key::Enter => {}
                Key::Next => {
                    if !list.property_mode {
                        list.subnext()
                    }
                }
                Key::Back => list.subback(),
                Key::PropertyMode => {
                    list.reset_substate();
                    list.property_mode = !list.property_mode;
                }
                Key::LineMode => list.change_linemode(),
                Key::MoveFirstLine => list.move_first_line(),
                Key::MoveLastLine => list.move_last_line(),
                Key::None => {}
            }

            continue;
        }

        // 通常選択時の処理
        match list.key {
            Key::Exit | Key::ExitMove => break,
            Key::Up => list.back(),
            Key::Down => list.next(),
            Key::Change => list.change(),
            Key::Enter => list.open_file(),
            Key::Next => list.open_file(),
            Key::Back => list.back_file(),
            Key::PropertyMode => {
                list.reset_substate();
                list.property_mode = !list.property_mode;
            }
            Key::LineMode => list.change_linemode(),
            Key::MoveFirstLine => list.move_first_line(),
            Key::MoveLastLine => list.move_last_line(),
            Key::None => {}
        }
    }
    //画面をクリア
    ui::draw::fin_clear();
    //rawモードoff
    crossterm::terminal::disable_raw_mode().unwrap();
}
