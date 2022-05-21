use cursive::Cursive;

use self::mines::_start_mines_inner;

mod mines;
mod game;

pub fn start_mines_inner(siv: &mut Cursive) {
    _start_mines_inner(siv);
}