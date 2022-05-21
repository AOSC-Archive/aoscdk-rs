use cursive::Cursive;

use self::mines::start_mines_inner;

mod mines;

pub fn start_mines(siv: &mut Cursive) {
    start_mines_inner(siv);
}
