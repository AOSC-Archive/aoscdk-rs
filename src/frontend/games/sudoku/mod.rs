use cursive::Cursive;

use self::game::run;

mod board;
mod game;
mod sudoku;

pub fn start_sudoku_inner(siv: &mut Cursive) {
    run(siv);
}