extern crate termion;

use termion::{color, style};

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

struct Cell {
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

struct Board {
    cells: Vec<Cell>,
    n_cols: usize,
}
impl Board {
    pub fn new() -> Board {
        Board {
            cells: vec![Cell::new('a', 0),
                        Cell::new('b', 1),
                        Cell::new('♚', 2),
                        Cell::new('c', 3)],
            n_cols: 2,
        }
    }
    #[allow(unused_variables)]
    pub fn print(&self) {
        &self.cells
            .chunks(self.n_cols)
            .into_iter()
            .map(|slice| {
                for cell in slice {
                    let bg = BgColor::from_ansi(cell.ansi_code);
                    print!("{}", cell.value);
                }
                print!("\n");
            })
            .collect::<Vec<_>>();

    }
}

fn print_board(board: &Board) {
    board.print();
}

fn main() {
    let b = Board::new();
    print_board(&b);
}
