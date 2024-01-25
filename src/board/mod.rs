const SUDOKO_CHAR_LENGTH: u8 = 36;

// region: const definition

const TOP_LEFT: char = '╔';
const TOP_RIGHT: char = '╗';
const TOP_DOUBLE_JOINT: char = '╤';
const TOP_AND_MIDDLE_DOUBLE_JOINT: char = '╦';
const BOTTOM_LEFT: char = '╚';
const BOTTOM_RIGHT: char = '╝';
const MIDDLE_DOUBLE_JOINT: char = '╫';
const MIDDLE_SINGLE_JOINT: char = '┼';
const LEFT_MIDDLE_JOINT: char = '╟';
const RIGHT_MIDDLE_JOINT: char = '╢';
const BOTTOM_DOUBLE_JOINT: char = '╧';
const BOTTOM_AND_MIDDLE_DOUBLE_JOINT: char = '╩';
const DOUBLE_H_LINE: char = '═';
const H_LINE: char = '─';
const DOUBLE_V_LINE: char = '║';
const V_LINE: char = '│';
const EMPTY_CELL: char = ' ';

// endregion: const definition

pub fn draw_board() -> String {
    let mut board = String::from("");
    for i in 0..=SUDOKO_CHAR_LENGTH {
        match i {
            0 => board.push_str(&draw_top_bottom_line(true)),
            SUDOKO_CHAR_LENGTH => board.push_str(&draw_top_bottom_line(false)),
            n if n % 4 == 0 && n % 3 == 0 => board.push_str(&draw_middle_with_double_joint()),
            n if n % 4 == 0 => board.push_str(&draw_middle_with_single_joint()),
            _ => board.push_str(&draw_middle_with_empty_cell()),
        };
        board.push('\n');
    }
    board
}

// region: draw sudoko lines

fn draw_top_bottom_line(is_top: bool) -> String {
    let mut line: Vec<char> = vec![];
    for i in 0..=SUDOKO_CHAR_LENGTH {
        match i {
            0 => line.push(if is_top { TOP_LEFT } else { BOTTOM_LEFT }),
            SUDOKO_CHAR_LENGTH => line.push(if is_top { TOP_RIGHT } else { BOTTOM_RIGHT }),
            n if n % 4 == 0 && n % 3 == 0 => line.push(if is_top {
                TOP_AND_MIDDLE_DOUBLE_JOINT
            } else {
                BOTTOM_AND_MIDDLE_DOUBLE_JOINT
            }),
            n if n % 4 == 0 => line.push(if is_top {
                TOP_DOUBLE_JOINT
            } else {
                BOTTOM_DOUBLE_JOINT
            }),
            _ => line.push(DOUBLE_H_LINE),
        }
    }

    line.into_iter().collect::<String>()
}

fn draw_middle_with_single_joint() -> String {
    let mut line: Vec<char> = vec![];
    for i in 0..=SUDOKO_CHAR_LENGTH {
        match i {
            0 => line.push(LEFT_MIDDLE_JOINT),
            SUDOKO_CHAR_LENGTH => line.push(RIGHT_MIDDLE_JOINT),
            n if n % 4 == 0 && n % 3 == 0 => line.push(MIDDLE_DOUBLE_JOINT),
            n if n % 4 == 0 => line.push(MIDDLE_SINGLE_JOINT),
            _ => line.push(H_LINE),
        }
    }

    line.into_iter().collect::<String>()
}

fn draw_middle_with_double_joint() -> String {
    let mut line: Vec<char> = vec![];
    for i in 0..=SUDOKO_CHAR_LENGTH {
        match i {
            0 => line.push(LEFT_MIDDLE_JOINT),
            SUDOKO_CHAR_LENGTH => line.push(RIGHT_MIDDLE_JOINT),
            n if n % 4 == 0 && n % 3 == 0 => line.push(MIDDLE_DOUBLE_JOINT),
            n if n % 4 == 0 => line.push(MIDDLE_SINGLE_JOINT),
            _ => line.push(H_LINE),
        }
    }

    line.into_iter().collect::<String>()
}

fn draw_middle_with_empty_cell() -> String {
    let mut line: Vec<char> = vec![];
    for i in 0..=SUDOKO_CHAR_LENGTH {
        match i {
            0 => line.push(DOUBLE_V_LINE),
            SUDOKO_CHAR_LENGTH => line.push(DOUBLE_V_LINE),
            n if n % 4 == 0 && n % 3 == 0 => line.push(DOUBLE_V_LINE),
            n if n % 4 == 0 => line.push(V_LINE),
            _ => line.push(EMPTY_CELL),
        }
    }

    line.into_iter().collect::<String>()
}

// endregion: draw sudoko lines
