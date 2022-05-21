use cursive::Cursive;

use self::minesweapers::start_mines_inner;

mod minesweapers;

pub fn start_mines(siv: &mut Cursive) {
    start_mines_inner(siv);
}
