extern crate termion;

use termion::{color, style};

use std::iter;

struct BgColor {
}
impl BgColor {
    pub fn from_ansi(value: u8) -> BgColor {
        print!("{}", color::Bg(color::AnsiValue(value)));
        BgColor {}
    }
}
impl Drop for BgColor {
    fn drop(&mut self) {
        print!("{}", style::Reset);
    }
}

pub struct Cell {
    pub value: char,
    pub ansi_code: u8,
}
impl Cell {
    pub fn new(val: char, ansi: u8) -> Cell {
        Cell {
            value: val,
            ansi_code: ansi,
        }
    }
}
pub fn toggle_color(ansi: &mut u8) -> u8 {
    let black: u8 = 1;
    let white: u8 = 0;
    if *ansi == black {
        *ansi = white
    } else {
        *ansi = black
    }
    *ansi
}

pub fn expand_if_numeric(x: char) -> Vec<char> {
    if x.is_numeric() {
        let space: char = ' ';
        iter::repeat(space).take(x.to_string().parse::<usize>().unwrap()).collect::<Vec<_>>()
    } else {
        vec![x]
    }

}

#[allow(unused_variables)]
pub fn print_line(line: &[Cell]) {
    for cell in line {
        let bg = BgColor::from_ansi(cell.ansi_code);
        print!("{}", cell.value);
    }
    print!("\n");
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
		 _  => x,
	}
}

struct Board {
    n_cols: usize,
    n_rows: usize,
}
impl Board {
    pub fn new() -> Board {
        Board {
            n_cols: 8,
            n_rows: 8,
        }
    }
    pub fn read_xchess(&self, xchess: &'static str) -> Vec<Cell> {
        let mut ansi: u8 = 0;
        xchess.chars()
            .into_iter()
            .map(|x| expand_if_numeric(x))
            .flat_map(|v| v.into_iter())
            .map(|x| Cell::new(unicode_pawn(x), toggle_color(&mut ansi)))
            .filter(|x| x.value != '/')
            .take(self.n_cols * self.n_rows)
            .collect()

    }

    pub fn print(&self, cells: &Vec<Cell>) {
        cells.chunks(self.n_cols)
            .into_iter()
            .map(|slice| {
                print_line(slice);
            })
            .collect::<Vec<_>>();

    }
}

fn main() {
    let c = Board::new();
    let mat = c.read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    c.print(&mat);
}
