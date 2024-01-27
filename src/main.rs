mod board;
mod infoline;
mod sudoku;

use std::{
    io::{self, stdout},
    u16, usize,
};

use crossterm::{
    cursor::{MoveTo, RestorePosition, SavePosition},
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Print, ResetColor, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use sudoku::{GameData, SudokuGame};

use crate::{
    board::{draw_board, CELL_WIDTH, START_POINT, SUDOKO_CHAR_LENGTH},
    infoline::{add_info_bellow_table, InfoStatus},
};

fn get_location_from_address(address: u16) -> (u16, u16) {
    let mut x = (address % 9) as u16;
    let mut y = (address / 9) as u16;
    x = x * CELL_WIDTH + START_POINT;
    y = y * CELL_WIDTH + START_POINT;
    return (x, y);
}

fn insert_number_in_sudoku(w: &mut impl io::Write, number: u16, address: u16) -> io::Result<()> {
    let (x, y) = get_location_from_address(address);
    queue!(
        w,
        SavePosition,
        MoveTo(x, y),
        Print(number.to_string().on_red().black()),
        RestorePosition
    )?;
    Ok(())
}

fn fill_sudoku_with_base_data<W>(w: &mut W, base_data: GameData) -> io::Result<()>
where
    W: io::Write,
{
    for i in 0..81 {
        let value = *base_data.get(i).unwrap();
        if value != 0 {
            insert_number_in_sudoku(w, value, i as u16)?;
        }
    }
    w.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let mut w = stdout();
    execute!(w, Clear(ClearType::All), MoveTo(0, 0), DisableMouseCapture)?;
    println!("{}\n", draw_board());

    let mut sudoku = SudokuGame::new();
    let mut solved_game = sudoku.clone_base_game();
    SudokuGame::solve_with_data(&mut solved_game);
    let mut address = 0;

    enable_raw_mode()?;
    fill_sudoku_with_base_data(&mut w, sudoku.base_data)?;

    execute!(w, MoveTo(START_POINT, START_POINT))?;
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        execute!(
            w,
            SavePosition,
            MoveTo(0, SUDOKO_CHAR_LENGTH + 1),
            Clear(ClearType::CurrentLine),
            ResetColor,
            RestorePosition
        )?;
        match code {
            KeyCode::Char('j') | KeyCode::Down => {
                address = (address + 9) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                address = (address + 72) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                address = (address + 1) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }
            KeyCode::Char('h') | KeyCode::Left => {
                address = (address + 80) % 81;
                let (x, y) = get_location_from_address(address);
                execute!(w, MoveTo(x, y))?;
            }

            KeyCode::Char('?') => {
                let possible_values = SudokuGame::possible_values(&sudoku.game_data, address);
                let possible_values_string = possible_values
                    .into_iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(".");
                add_info_bellow_table(
                    format!("you can add {} numbers", possible_values_string),
                    InfoStatus::Help,
                );
            }

            KeyCode::Char('c') => {
                add_info_bellow_table(format!("you can solve this table"), InfoStatus::Warning);
            }

            KeyCode::Char(' ') => {
                let mut cells: Vec<usize> = Vec::new();
                if !sudoku.is_solved() {
                    for i in 0..81 {
                        if solved_game[i] != sudoku.game_data[i] {
                            cells.push(i);
                        }
                    }
                    add_info_bellow_table(
                        format!(
                            "cells {}{}Have Wrong Value",
                            cells[..2]
                                .iter()
                                .map(|&id| {
                                    let (x, y) = SudokuGame::get_column_row_from_game_data_position(
                                        id as u16,
                                    );
                                    format!(" ({},{}) ", y + 1, x + 1)
                                })
                                .collect::<String>(),
                            if cells.len() > 2 {
                                format!(" and {} other cells ", cells.len() - 2)
                            } else {
                                String::from("")
                            },
                        ),
                        InfoStatus::Warning,
                    );
                } else {
                    add_info_bellow_table("Game Compeleted Successfully", InfoStatus::Help);
                }
            }

            KeyCode::Char('w') => {
                if sudoku.base_data[address as usize] == 0 {
                    let value = solved_game[address as usize];
                    sudoku.insert_value(value, address);
                    execute!(w, SavePosition, Print(value), ResetColor, RestorePosition)?;
                }
            }

            KeyCode::Char('q') => {
                execute!(w, Clear(ClearType::All), MoveTo(0, 0))?;
                break;
            }
            KeyCode::Char(c) if c.is_digit(10) => {
                if sudoku.base_data[address as usize] == 0 {
                    let number = c.to_digit(10).unwrap() as u16;
                    sudoku.insert_value(number, address);
                    execute!(
                        w,
                        SavePosition,
                        Print(if number == 0 {
                            ' '.to_string()
                        } else {
                            number.to_string()
                        }),
                        ResetColor,
                        RestorePosition
                    )?;
                } else {
                    add_info_bellow_table("this cell is readonly!", InfoStatus::Warning)
                }
            }
            KeyCode::Char(c) => {
                add_info_bellow_table(format!("Char {} is not acceptable.", c), InfoStatus::Danger)
            }

            _ => add_info_bellow_table("Wrong", InfoStatus::Danger),
        }
    }

    disable_raw_mode()?;
    Ok(())
}
