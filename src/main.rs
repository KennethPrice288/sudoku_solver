mod app;
mod sudoku;

use std::io;

fn main() -> io::Result<()> {
    let mut board = [[0; 9]; 9];
    let mut app = app::App::new(&mut board);
    app.run()
}
