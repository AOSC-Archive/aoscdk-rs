use cursive::Cursive;

use self::minesweeper::start_mines_inner;

mod minesweeper;

pub fn start_mines(siv: &mut Cursive) {
    start_mines_inner(siv);
}
