// region: info line managment

use core::fmt;
use std::io::stdout;

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    execute,
    style::{self, Print, ResetColor},
};

use crate::board::SUDOKO_CHAR_LENGTH;

pub enum InfoStatus {
    Danger,
    Warning,
    Help,
}

pub fn add_info_bellow_table(content: impl fmt::Display, status: InfoStatus) {
    let mut w = stdout();
    let forground = match status {
        InfoStatus::Danger => style::SetForegroundColor(style::Color::Red),
        InfoStatus::Warning => style::SetForegroundColor(style::Color::DarkYellow),
        InfoStatus::Help => style::SetForegroundColor(style::Color::Green),
    };

    let _ = execute!(
        w,
        SavePosition,
        MoveTo(0, SUDOKO_CHAR_LENGTH + 1),
        forground,
        Print(content),
        ResetColor,
        RestorePosition
    );
}

// endregion: info line managment
