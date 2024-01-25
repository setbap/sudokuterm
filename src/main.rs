mod board;
use std::io::{self, stdout, Write};

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::board::draw_board;

pub fn read_line() -> io::Result<String> {
    let mut line = String::new();
    while let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Enter => {
                break;
            }
            KeyCode::Char(c) => {
                line.push(c);
            }
            _ => {}
        }
    }

    Ok(line)
}

fn get_location_from_address(address: u8) -> (u16, u16) {
    let mut x = (address % 9) as u16;
    let mut y = (address / 9) as u16;
    x = x * 4 + 2;
    y = y * 4 + 2;
    return (x, y);
}

fn main() -> io::Result<()> {
    let mut w = stdout();
    execute!(w, Clear(ClearType::All), MoveTo(0, 0))?;
    println!("{}\n", draw_board());

    let mut address = 0;

    enable_raw_mode()?;
    execute!(w, MoveTo(2, 2))?;
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        execute!(
            w,
            SavePosition,
            MoveTo(0, 37),
            Clear(ClearType::CurrentLine),
            ResetColor,
            RestorePosition
        )?;
        match code {
            KeyCode::Char('j') => {
                address = (address + 9) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('k') => {
                address = (address + 72) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('l') => {
                address = (address + 1) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('h') => {
                address = (address + 80) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('q') => {
                execute!(w, Clear(ClearType::All), MoveTo(0, 0))?;
                break;
            }
            _ => {
                execute!(
                    w,
                    SavePosition,
                    MoveTo(0, 37),
                    Print("wrong Selected Key".on_red().to_string()),
                    // Reset to default colors
                    ResetColor,
                    RestorePosition
                )?;
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
