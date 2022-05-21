use cursive::Cursive;

use self::mines::_start_mines_inner;

mod game;
mod mines;

pub fn start_mines_inner(siv: &mut Cursive) {
    _start_mines_inner(siv);
}
