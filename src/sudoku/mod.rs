use std::{u16, usize};

use rand::{seq::SliceRandom, Rng};

pub type GameData = [u16; 81];

const game_data: GameData = [
    0, 0, 7, 4, 1, 9, 0, 0, 0, 6, 0, 9, 2, 5, 0, 1, 0, 0, 1, 4, 5, 3, 7, 0, 8, 0, 0, 0, 6, 3, 8, 4,
    0, 7, 5, 0, 7, 0, 0, 0, 0, 0, 4, 0, 8, 4, 1, 8, 0, 0, 0, 6, 0, 3, 5, 9, 6, 0, 0, 0, 0, 0, 0, 3,
    8, 4, 0, 9, 0, 5, 0, 6, 2, 7, 1, 6, 8, 0, 9, 3, 0,
];

#[derive(Debug)]
pub struct SudokuGame {
    pub base_data: GameData,
    pub game_data: GameData,
}

impl SudokuGame {
    pub fn new() -> SudokuGame {
        SudokuGame {
            base_data: game_data,
            game_data: game_data.clone(),
        }
    }

    pub fn clone_base_game(&mut self) -> GameData {
        let cloned_base = self.base_data.clone();
        return cloned_base;
    }

    pub fn insert_value(&mut self, value: u16, location: u16) {
        self.game_data[location as usize] = value;
    }

    pub fn get_column_row_from_game_data_position(position: u16) -> (u16, u16) {
        let x = (position % 9) as u16;
        let y = (position / 9) as u16;
        return (x, y);
    }
    fn get_position_from_column_row(x: u16, y: u16) -> u16 {
        return y * 9 + x;
    }
    fn get_box_first_cell_row_column(x: u16, y: u16) -> (u16, u16) {
        (x - (x % 3), y - (y % 3))
    }

    fn is_valid_in_box(data: &GameData, x: u16, y: u16, number: u16) -> bool {
        let (start_x, start_y) = Self::get_box_first_cell_row_column(x, y);
        for i in 0..3 {
            for j in 0..3 {
                let location = Self::get_position_from_column_row(start_x + j, start_y + i);
                let current_location = Self::get_position_from_column_row(x, y);
                let cell_value = data.get(location as usize).unwrap();
                if cell_value == &number && current_location != location {
                    return false;
                }
            }
        }
        return true;
    }
    fn is_valid_in_row(data: &GameData, _x: u16, y: u16, number: u16) -> bool {
        for column in 0..9 {
            let location = Self::get_position_from_column_row(column, y);
            let cell_value = data.get(location as usize).unwrap();
            if cell_value == &number && _x != column {
                return false;
            }
        }
        return true;
    }
    fn is_valid_in_column(data: &GameData, x: u16, _y: u16, number: u16) -> bool {
        for row in 0..9 {
            let location = Self::get_position_from_column_row(x, row);
            let cell_value = data.get(location as usize).unwrap();
            if cell_value == &number && _y != row {
                return false;
            }
        }
        return true;
    }
    fn _is_valid(data: &GameData, position: u16, number: u16) -> bool {
        let (x, y) = Self::get_column_row_from_game_data_position(position);
        Self::is_valid_in_column(data, x, y, number)
            && Self::is_valid_in_row(data, x, y, number)
            && Self::is_valid_in_box(data, x, y, number)
    }
    pub fn is_valid(&self, position: u16, number: u16) -> bool {
        Self::_is_valid(&self.game_data, position, number)
    }

    pub fn is_solved(&self) -> bool {
        Self::_is_solved(&self.game_data)
    }

    fn _is_solved(data: &GameData) -> bool {
        if data.contains(&0) {
            return false;
        }
        for i in 0..81 {
            if !Self::_is_valid(data, i, data[i as usize]) {
                return false;
            }
        }
        return true;
    }

    pub fn possible_values(data: &GameData, position: u16) -> Vec<u16> {
        let mut values: Vec<u16> = Vec::new();
        for i in 0..9 {
            if Self::_is_valid(data, position, i) {
                values.push(i);
            }
        }

        values
    }

    pub fn invalid_cells(&self, data: &GameData) -> Vec<usize> {
        let mut wrong_cells: Vec<usize> = Vec::new();
        for i in 0..81 {
            let val = data[i];
            if val != 0 && !Self::_is_valid(data, i as u16, data[i]) {
                wrong_cells.push(i);
            }
        }
        return wrong_cells;
    }

    pub fn solve_according_base(&self) -> GameData {
        let mut data = self.base_data.clone();
        Self::solve_with_data(&mut data);

        data
    }

    pub fn solve_current_game(&self) -> GameData {
        let mut data = self.game_data.clone();
        Self::solve_with_data(&mut data);
        data
    }

    pub fn solve_with_data(data: &mut GameData) -> bool {
        let first_zero_index = data.iter().position(|&x| x == 0);

        if let Some(i) = first_zero_index {
            for guess_value in 1..=9 {
                if Self::_is_valid(&data, i as u16, guess_value) {
                    data[i] = guess_value;

                    if Self::solve_with_data(data) {
                        return true;
                    }
                    data[i] = 0
                }
            }
            return false;
        }

        return true;
    }
}
