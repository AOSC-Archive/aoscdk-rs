use crate::frontend::games::sudoku::board::SudokuBoard;
use cursive::{
    traits::*,
    views::{Button, Dialog, DummyView, LinearLayout},
    Cursive,
};

pub fn run(siv: &mut Cursive) {
    siv.add_global_callback('r', restart);
    siv.add_global_callback('h', hint);
    siv.add_global_callback('q', quit);

    siv.set_fps(2);

    let board = SudokuBoard::new();

    let buttons1 = LinearLayout::horizontal()
        .child(Button::new("Restart", restart))
        .child(Button::new("Hint", hint))
        .child(Button::new("Undo", undo))
        .child(Button::new("Redo", redo));

    let buttons2 = LinearLayout::horizontal()
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Help", help))
        .child(Button::new("Quit", quit));

    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("board"))
            .child(buttons1)
            .child(buttons2),
    )
    .title("SUDOKU");

    siv.add_layer(view);
}

fn quit(s: &mut Cursive) {
    s.cb_sink()
        .send(Box::new(|s| {
            s.pop_layer();
            s.clear_global_callbacks('q');
        }))
        .unwrap()
}

fn restart(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.restart();
    });
}

fn hint(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.hint();
    });
}

fn undo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.undo();
    });
}

fn redo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut SudokuBoard| {
        board.redo();
    });
}

fn help(s: &mut Cursive) {
    s.add_layer(Dialog::info("Use arrow keys/TAB/Shift+TAB/mouse wheel/mouse click to navigate.\nEnter number 0-9 to fill in.\nClick <Hint> or press <h> to obtain a hint.\nGood luck."))
}
