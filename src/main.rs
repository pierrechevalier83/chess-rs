extern crate termion;

use termion::{color, style};
use termion::input::{TermRead, MouseTerminal};
use termion::event::{Key, Event, MouseEvent};
use termion::raw::IntoRawMode;
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

pub fn print_row<W: Write>(stdout: &mut W, line: &Vec<Cell>, fmt: &BoardFormat) {
    let pad = Pad::new(fmt.cell_height, 1);
    for _ in 0..pad.before {
        padding_line(stdout, line, fmt.cell_width);
    }
    print_line(stdout, line, fmt.cell_width);
    for _ in 0..pad.after {
        padding_line(stdout, line, fmt.cell_width);
    }
}

pub fn padding_line<W: Write>(stdout: &mut W, line: &Vec<Cell>, cell_width: usize) {
    print_line(stdout,
               &line.iter()
                   .cloned()
                   .map(|x| Cell::new(' ', x.ansi_code))
                   .collect::<Vec<_>>(),
               cell_width);
}

macro_rules! wr {
    ($out:expr$(, $x:expr)* ) => (write!($out$(, $x)*).expect("Error while trying to write to out!"));
}

#[allow(unused_variables)]
pub fn print_line<W: Write>(stdout: &mut W, line: &Vec<Cell>, cell_width: usize) {
    let pad = Pad::new(cell_width, 1);
    for cell in line {
        let bg = BgColor::from_ansi(cell.ansi_code);
        wr!(stdout, "{}", n_spaces::<String>(pad.before));
        wr!(stdout, "{}", cell.value);
        wr!(stdout, "{}", n_spaces::<String>(pad.after));
    }
    wr!(stdout, "\n");
    wr!(stdout,
        "{}",
        termion::cursor::Left(cell_width as u16 * line.len() as u16));
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

struct Board<W: Write> {
    n_cols: usize,
    n_rows: usize,
    stdout: W,
}
impl<W: Write> Board<W> {
    pub fn new(w: W) -> Board<W> {
        Board {
            n_cols: 8,
            n_rows: 8,
            stdout: w,
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

    pub fn print(&mut self, cells: &Vec<Cell>, fmt: &BoardFormat) {
        wr!(self.stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1));
        cells.chunks(self.n_cols)
            .into_iter()
            .map(|row| {
                print_row(&mut self.stdout,
                          &row.iter().cloned().collect::<Vec<_>>(),
                          fmt);
            })
            .collect::<Vec<_>>();
    }
    pub fn handle_mouse(&mut self, x: u16, y: u16) {
        wr!(self.stdout, "{},{}\n", x, y);
        wr!(self.stdout, "{}", termion::cursor::Goto(1, 25));
    }
}

fn main() {
    let stdout = MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let mut b = Board::new(stdout);
    let fmt = BoardFormat::new();
    let mat = b.read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");


    b.print(&mat, &fmt);

    for input in std::io::stdin().events() {
        b.print(&mat, &fmt);
        let evt = input.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        b.handle_mouse(x, y);
                    }
                    _ => (),
                }
            }
            _ => {}
        }
        std::io::stdout().flush().ok().expect("Could not flush stdout");
    }
}
