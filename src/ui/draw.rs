use std::{
    io::{self, Write},
    thread,
};

use crossterm::{
    cursor,
    terminal::{self, ClearType},
    QueueableCommand,
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{self, Block, Borders, List, ListItem, Paragraph, Tabs},
    Terminal,
};

use console::Emoji;

use crate::search_dir::Events;

///描写
#[inline]
pub fn draw(events: &mut Events) {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|f| {
            let tab = thread::spawn(|| {
                let titles: Vec<Line> = ["終了q", "開く Enter,→", "戻る ←", "選択 ↑↓,ws"]
                    .into_iter()
                    .map(Line::from)
                    .collect();

                Tabs::new(titles)
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Reset).bg(Color::Reset))
            });

            let chunks = Layout::default()
                .constraints([Constraint::Max(f.area().height - 3), Constraint::Max(3)].as_ref())
                .split(f.area());

            let main_display = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(chunks[0]);

            let items = events
                .items
                .iter()
                .enumerate()
                .map(|(num, i)| {
                    if num == 0 {
                        i.iter()
                            .map(|s| {
                                ListItem::new(Emoji("📂 ", "").to_string() + s.to_str().unwrap())
                            })
                            .collect::<Vec<ListItem>>()
                    } else {
                        i.iter()
                            .map(|s| {
                                ListItem::new(Emoji("📃 ", "").to_string() + s.to_str().unwrap())
                            })
                            .collect::<Vec<ListItem>>()
                    }
                })
                .collect::<Vec<Vec<ListItem>>>()
                .concat();

            let item = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Green))
                .highlight_symbol(">>");

            let text = Paragraph::new({
                if events.property_mode {
                    events.property.as_ref().unwrap().to_text()
                } else {
                    events.data.clone()
                }
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if events.submode {
                        Color::Green
                    } else {
                        Color::White
                    })),
            )
            .style(Style::default().fg(Color::White))
            .scroll(events.substate);

            f.render_widget(widgets::Clear, f.area());
            f.render_widget(tab.join().unwrap(), chunks[1]);
            f.render_stateful_widget(item, main_display[0], &mut events.state);
            f.render_widget(text, main_display[1]);
        })
        .unwrap();
}

/// ターミナルをclearする
#[inline]
pub fn fin_clear() {
    let mut stdout = io::stdout();

    //ターミナルを削除
    stdout
        .queue(terminal::Clear(ClearType::FromCursorUp))
        .unwrap();
    //カーソルを左上にする
    stdout.queue(cursor::MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
}
