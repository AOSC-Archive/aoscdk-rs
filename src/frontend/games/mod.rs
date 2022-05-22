use cursive::{
    view::SizeConstraint,
    views::{Dialog, LinearLayout, ResizedView, ScrollView, SelectView},
    Cursive,
};

use self::{minesweeper::start_mines_inner, sudoku::start_sudoku_inner};

mod minesweeper;
mod sudoku;

pub fn start_mines(siv: &mut Cursive) {
    start_mines_inner(siv);
}

pub fn start_sudoku(siv: &mut Cursive) {
    start_sudoku_inner(siv);
}

pub fn start_game(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::around(ResizedView::new(
            SizeConstraint::AtMost(64),
            SizeConstraint::Free,
            ScrollView::new(
                LinearLayout::vertical().child(
                    SelectView::new()
                        .with_all_str(vec!["Sudoku", "Minesweeper"])
                        .on_submit(|s: &mut Cursive, c| match c {
                            "Sudoku" => start_sudoku(s),
                            "Minesweeper" => start_mines(s),
                            _ => unreachable!(),
                        }),
                ),
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
