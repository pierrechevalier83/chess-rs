extern crate termion;

use termion::{color, style};

use std::iter;

struct BgColor {
}
impl BgColor {
    pub fn from_ansi(value: u8) -> BgColor {
        print!("{}{}",color::Fg(color::Blue), color::Bg(color::AnsiValue(value)));
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
    let black: u8 = 0;
    let white: u8 = 7;
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

pub fn n_spaces(n: usize) -> String {
    iter::repeat(' ').take(n).collect()
}

#[allow(unused_variables)]
pub fn print_line(line: &[Cell], cell_width: usize) {
    let pad_first = n_spaces( (cell_width - 1) / 2);
	let pad_next = n_spaces(cell_width - 1 - pad_first.len());
    for cell in line {
        let bg = BgColor::from_ansi(cell.ansi_code);
		
        print!("{}", pad_first);
        print!("{}", cell.value);
        print!("{}", pad_next);
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

struct BoardFormat {
    pub cell_width: usize,
}
impl BoardFormat {
    pub fn new() -> BoardFormat {
	    BoardFormat {
		    cell_width: 3,
		}
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

    pub fn print(&self, cells: &Vec<Cell>, fmt: &BoardFormat) {
        cells.chunks(self.n_cols)
            .into_iter()
            .map(|slice| {
                print_line(slice, fmt.cell_width);
            })
            .collect::<Vec<_>>();
    }
}

fn main() {
    let b = Board::new();
    let mat = b.read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    b.print(&mat, &BoardFormat::new());
}
