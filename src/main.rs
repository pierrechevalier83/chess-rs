extern crate termion;

use termion::{color, style};
use termion::input::TermRead;
use termion::event::{Key, Event, MouseEvent};
use std::iter;
use std::io::Write;

struct BgColor {
}
impl BgColor {
    pub fn from_ansi(value: u8) -> BgColor {
        print!("{}{}",
               color::Fg(color::Blue),
               color::Bg(color::AnsiValue(value)));
        BgColor {}
    }
}
impl Drop for BgColor {
    fn drop(&mut self) {
        print!("{}", style::Reset);
    }
}

#[derive(Clone)]
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

pub fn n_spaces<T>(n: usize) -> T
    where T: std::iter::FromIterator<char>
{
    iter::repeat(' ').take(n).collect::<T>()
}

pub fn expand_if_numeric(x: char) -> Vec<char> {
    if x.is_numeric() {
        n_spaces(x.to_string().parse::<usize>().unwrap())
    } else {
        vec![x]
    }

}

struct Pad {
    pub before: usize,
    pub after: usize,
}

impl Pad {
    pub fn new(width: usize, content: usize) -> Pad {
        Pad {
            before: (width - content) / 2 + 1 - width % 2,
            after: (width - content) / 2,
        }
    }
}

pub fn print_row(line: &Vec<Cell>, fmt: &BoardFormat) {
    let pad = Pad::new(fmt.cell_height, 1);
    for _ in 0..pad.before {
        padding_line(line, fmt.cell_width);
    }
    print_line(line, fmt.cell_width);
    for _ in 0..pad.after {
        padding_line(line, fmt.cell_width);
    }
}

pub fn padding_line(line: &Vec<Cell>, cell_width: usize) {
    print_line(&line.iter()
                   .cloned()
                   .map(|x| Cell::new(' ', x.ansi_code))
                   .collect::<Vec<_>>(),
               cell_width);
}

#[allow(unused_variables)]
pub fn print_line(line: &Vec<Cell>, cell_width: usize) {
    let pad = Pad::new(cell_width, 1);
    for cell in line {
        let bg = BgColor::from_ansi(cell.ansi_code);
        print!("{}", n_spaces::<String>(pad.before));
        print!("{}", cell.value);
        print!("{}", n_spaces::<String>(pad.after));
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
        _ => x,
    }
}

pub struct BoardFormat {
    pub cell_width: usize,
    pub cell_height: usize,
}
impl BoardFormat {
    pub fn new() -> BoardFormat {
        BoardFormat {
            cell_width: 7,
            cell_height: 3,
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
        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        cells.chunks(self.n_cols)
            .into_iter()
            .map(|row| {
                print_row(&row.iter().cloned().collect::<Vec<_>>(), fmt);
            })
            .collect::<Vec<_>>();
    }
}

fn main() {
    let b = Board::new();
    let fmt = BoardFormat::new();
    let mat = b.read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    b.print(&mat, &fmt);
    for input in std::io::stdin().events() {
        b.print(&mat, &fmt);
        let evt = input.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            _ => {}
        }
        std::io::stdout().flush().ok().expect("Could not flush stdout");
    }
}
