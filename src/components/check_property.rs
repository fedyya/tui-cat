use ratatui::{text::{Text, Span, Line}, style::Style};
use chrono::{DateTime, Utc, NaiveDateTime, Datelike, Timelike, TimeDelta};
use std::{path::{Path, PathBuf}, fs::Metadata, io::{Error, ErrorKind}};

/// 時差
const TIME_DIFFERENSE: i64 = 9;

#[derive(Debug)]
pub struct Property {
    /// ファイル名
    file_name: String,
    /// ファイルのパス
    place: PathBuf,
    /// ファイルの作成日時
    create_time: Result<NaiveDateTime, Error>,
    /// ファイルの最終更新日時
    last_update: Result<NaiveDateTime, Error>,
    /// ファイルの最終アクセス日時
    last_access: Result<NaiveDateTime, Error>,
    /// ファイルの大きさ（バイト）
    size: u64,
    /// [true]アクセス可能 [false]アクセス不可
    user_access: bool,
}

enum Get {
    /// 作成日時
    CreateTime,
    /// 最終更新日時
    UpdateTime,
    /// 最終アクセス日時
    LastAccess,
}

impl Property {
    pub fn new(path: &Path) -> Option<Property> {
        let data = match path.metadata() {
            Ok(x) => x,
            Err(_) => return None,
        };

        Some(Property {
            file_name: path.iter().last()?.to_str()?.to_string(),
            place: path.to_path_buf(),
            create_time: get_time(&data, Get::CreateTime),
            last_update: get_time(&data, Get::UpdateTime),
            last_access: get_time(&data, Get::LastAccess),
            size: data.len(),
            user_access: data.accessed().is_ok(),
        })
    }

    pub fn to_text(&self) -> Text{
        let access_check = |f: bool| -> &str {
            if let true = f {
                "可能"
            }else {
                "不可"
            }
        };

        let spans = vec![
            Line::from(
                Span::styled(
                    "プロパティモード",
                    Style::default().fg(ratatui::style::Color::DarkGray)
                )
            ),
            Line::default(),
            Line::from("ファイル名：".to_string() + &self.file_name),
            Line::from("場所：".to_string() + self.place.to_str().unwrap()),
            Line::from("サイズ：".to_string() + &self.size.to_string()),
            Line::from("アクセス：".to_string() + access_check(self.user_access)),
            Line::from("ファイル作成日：".to_string() + &time_check(&self.create_time)),
            Line::from("最終更新日　　：".to_string() + &time_check(&self.last_update)),
            Line::from("最終アクセス　：".to_string() + &time_check(&self.last_access)),
        ];
        
        Text::from(spans)
    } 
}

/// 時刻を取得し出来たら値を返す。<br>
/// 第二引数が以下のものを返す <br>
/// [Get::CreateTime] -> ファイル作成した日時<br>
/// [Get::UpdateTime] -> ファイル更新日時<br>
/// [Get::LastAccess] -> ファイルに最後にアクセスした日時 
#[inline]
fn get_time(metadata: &Metadata, x: Get) -> Result<NaiveDateTime, Error> {
    if let Some(x) = DateTime::<Utc>::from(
                match x {
                    Get::CreateTime => metadata.created()?,
                    Get::UpdateTime => metadata.modified()?,
                    Get::LastAccess => metadata.accessed()?,
                }
            )
            .naive_local()
            .checked_add_signed(TimeDelta::try_hours(TIME_DIFFERENSE).unwrap()) {
        Ok(x)
    } else {
        Err(Error::new(ErrorKind::Other, "err"))
    }
}

#[inline]
fn time_check(time: &Result<NaiveDateTime, Error>) -> String {
    match time {
        Ok(date_time) => {
            [&date_time.date().year().to_string() , "年" ,
                &date_time.date().month().to_string() , "月" ,
                &date_time.date().day().to_string() , "日" ,
                &date_time.hour().to_string() , "時" , 
                &date_time.minute().to_string() , "分"].concat()
        },

        Err(_) => {
            "取得できませんでした".to_string()
        }
    }
}
