use cursive::{
    view::SizeConstraint,
    views::{Dialog, LinearLayout, ResizedView, ScrollView, SelectView},
    Cursive,
};

use self::{minesweeper::start_mines_inner, sudoku::start_sudoku_inner};

mod minesweeper;
mod sudoku;

fn start_mines(siv: &mut Cursive) {
    clear_callback(siv);
    start_mines_inner(siv);
}

fn start_sudoku(siv: &mut Cursive) {
    clear_callback(siv);
    start_sudoku_inner(siv);
}

fn start_game(siv: &mut Cursive) {
    let mut game = vec!["Sudoku"];
    if std::env::var("DISPLAY").is_ok() {
        game.push("Minesweeper");
    }
    siv.add_layer(
        Dialog::around(ResizedView::new(
            SizeConstraint::AtMost(64),
            SizeConstraint::Free,
            ScrollView::new(
                LinearLayout::vertical().child(SelectView::new().with_all_str(game).on_submit(
                    |s: &mut Cursive, c| match c {
                        "Sudoku" => start_sudoku(s),
                        "Minesweeper" => start_mines(s),
                        _ => unreachable!(),
                    },
                )),
            ),
        ))
        .padding_lrtb(2, 2, 1, 1)
        .title("Select a game")
        .button("Quit", |s| {
            s.cb_sink()
                .send(Box::new(|s| {
                    s.pop_layer();
                }))
                .unwrap()
        }),
    );
}

fn clear_callback(siv: &mut Cursive) {
    siv.clear_global_callbacks('m');
    siv.clear_global_callbacks('s');
    siv.clear_global_callbacks('g');
}

pub fn add_main_callback(siv: &mut Cursive) {
    if std::env::var("DISPLAY").is_ok() {
        siv.add_global_callback('m', |s| {
            start_mines(s);
        });
    }
    siv.add_global_callback('s', |s| {
        start_sudoku(s);
    });
    siv.add_global_callback('g', |s| {
        start_game(s);
    });
}
