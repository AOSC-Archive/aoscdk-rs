use crate::frontend::{
    games::{add_main_callback, sudoku::board::SudokuBoard},
    tui::wrap_in_dialog,
};
use cursive::{
    traits::*,
    views::{Dialog, LinearLayout, TextView},
    Cursive,
};

pub fn run(siv: &mut Cursive) {
    add_game_callbacks(siv);

    siv.set_fps(2);

    let board = SudokuBoard::new();
    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("board"))
            .child(TextView::new("Press 'm' for controls.")),
    )
    .title("SUDOKU");

    siv.add_layer(view);
}

fn add_game_callbacks(siv: &mut Cursive) {
    siv.add_global_callback('r', restart);
    siv.add_global_callback('h', hint);
    siv.add_global_callback('q', quit);
    siv.add_global_callback('m', help);
    siv.add_global_callback('z', redo);
    siv.add_global_callback('u', undo);
}

fn clear_game_callback(siv: &mut Cursive) {
    for i in ['r', 'h', 'q', 'm', 'z', 'u'] {
        siv.clear_global_callbacks(i);
    }
}

fn quit(s: &mut Cursive) {
    s.cb_sink()
        .send(Box::new(|s| {
            s.pop_layer();
            clear_game_callback(s);
            add_main_callback(s);
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
    clear_game_callback(s);
    s.add_layer(wrap_in_dialog(TextView::new("Use arrow keys/Tab/Shift+Tab/mouse wheel/mouse click to navigate.\nEnter number 0-9 to fill in a number.\nClick <Hint> or press <h> to obtain a hint.\nPress <z> to redo\nPress <u> to undo\nPress <r> to restart game\nPress <q> to exit game\n\nGood luck."), "Tips", None).button("OK", |s| {
        s.pop_layer();
        add_game_callbacks(s);
    }));
}
