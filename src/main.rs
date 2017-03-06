extern crate matrix_display;
use matrix_display::*;

pub fn n_spaces<T>(n: usize) -> T
    where T: std::iter::FromIterator<char>
{
    std::iter::repeat(' ').take(n).collect::<T>()
}

pub fn expand_if_numeric(x: char) -> Vec<char> {
    if x.is_numeric() {
        n_spaces(x.to_string().parse::<usize>().unwrap())
    } else {
        vec![x]
    }
}

pub fn unicode_pawn(x: char) -> char {
    match x {
        'r' => '♜',
        'R' => '♖',
        'n' => '♞',
        'N' => '♘',
        'b' => '♝',
        'B' => '♗',
        'q' => '♛',
        'Q' => '♕',
        'k' => '♚',
        'K' => '♔',
        'p' => '♟',
        'P' => '♙',
        _ => x,
    }
}

pub fn read_xchess(xchess: &'static str) -> Vec<cell::Cell<char>> {
    xchess.chars()
        .into_iter()
        .map(|x| expand_if_numeric(x))
        .flat_map(|v| v.into_iter())
        .filter(|x| *x != '/')
		.enumerate()
        .map(|(i, x)| {
            let ansi_fg = 33;
            let mut ansi_bg = 0;
            if i % 2 + (i / 8) % 2 == 1 {
                ansi_bg = 7;
            }
		    cell::Cell::new(unicode_pawn(x), ansi_fg, ansi_bg)
		})
        .take(8 * 8)
        .collect()
}

fn main() {
    let format = Format::new(7, 3);
    let board = read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let data = matrix::Matrix::new(8, board);
    let display = MatrixDisplay::new(format, data);
    display.print(&mut std::io::stdout(), &style::BordersStyle::None);
}
