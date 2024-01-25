mod board;
use std::io;

use crate::board::draw_board;

fn main() -> io::Result<()> {
    println!("{}", draw_board());

    Ok(())
}
