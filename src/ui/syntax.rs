use std::borrow::Cow;

use ratatui::text::{Line, Span, Text};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;

/// テキストがシンタックスハイライト可能であれば変更する
pub fn hylight<'a, 'b, T1, T2>(text: T1, extension: T2) -> Text<'a>
where
    T1: Into<Cow<'a, str>> + ToString,
    T2: Into<Cow<'b, str>>,
{
    // 実装は(https://docs.rs/syntect/latest/syntect/easy/struct.HighlightLines.html)参考
    let ps = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();

    let syntax = match ps.find_syntax_by_extension(&extension.into()) {
        Some(f) => f,
        None => return Text::from(text.into()),
    };

    let mut h = HighlightLines::new(syntax, &ts.themes["base16-eighties.dark"]);

    let mut spans: Vec<Line> = Vec::with_capacity(500);
    for line in text.to_string().lines() {
        let span: Vec<Span> = h
            .highlight_line(line, &ps)
            .unwrap()
            .iter()
            .map(|(style, text)| Span::styled(text.to_string(), into_color(style)))
            .collect();

        spans.push(Line::from(span));
    }

    Text::from(spans)
}

/// [syntect::highlighting::Style]を[tui::style::Style]形式に変更する
///
/// ### Note
///     backgroundを変更すると文字の部分のみ反映されてしまうので実装していない。
///     draw側で制御すれば良い感じになるかも？
#[inline]
const fn into_color(style: &Style) -> ratatui::style::Style {
    ratatui::style::Style::new().fg(ratatui::style::Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ))
}
