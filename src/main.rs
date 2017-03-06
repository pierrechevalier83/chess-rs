extern crate matrix_display;
use matrix_display::*;
mod xchess;

fn main() {
    let format = Format::new(7, 3);
    let board = xchess::read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let data = matrix::Matrix::new(8, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
