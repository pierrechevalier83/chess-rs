extern crate matrix_display;
extern crate termion;

use matrix_display::*;
use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, Key, MouseEvent};
use termion::raw::IntoRawMode;
mod xchess;

fn clear<W>(out: &mut W)
where
    W: std::io::Write,
{
    write!(
        out,
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Hide,
        termion::cursor::Goto(1, 1)
    ).unwrap();
}

#[derive(Clone)]
struct Selection {
    pos: (usize, usize),
    cell: cell::Cell<char>,
}

fn get_cell(display: &mut MatrixDisplay<char>, pos: (usize, usize)) -> cell::Cell<char> {
    display.cell_at_cursor_position(pos).clone()
}

fn set_bg(display: &mut MatrixDisplay<char>, pos: (usize, usize), bg: u8) {
    display.cell_at_cursor_position(pos).color.bg = bg;
}

fn swap_cells(
    display: &mut MatrixDisplay<char>,
    left_pos: (usize, usize),
    right_pos: (usize, usize),
) {
    let left_value = get_cell(display, left_pos).value;
    display
        .cell_at_cursor_position((left_pos.0, left_pos.1))
        .value = display
        .cell_at_cursor_position((right_pos.0, right_pos.1))
        .value;
    display
        .cell_at_cursor_position((right_pos.0, right_pos.1))
        .value = left_value;
}

fn redraw(
    mut stdout: &mut MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
    mut display: &mut MatrixDisplay<char>,
    selection: &Option<Selection>,
) {
    match selection {
        &Some(ref sel) => set_bg(&mut display, sel.pos, 23),
        &None => (),
    }
    clear(&mut stdout);
    display.print(&mut stdout, &style::BordersStyle::None);
    match selection {
        &Some(ref sel) => set_bg(&mut display, sel.pos, sel.cell.color.bg),
        &None => (),
    }
}

fn initial_grid() -> (Format, matrix::Matrix<cell::Cell<char>>) {
    let format = Format::new(7, 3);
    let board = xchess::read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    (format, matrix::Matrix::new(8, board))
}

fn main() {
    let (format, mut data) = initial_grid();
    let mut display = MatrixDisplay::new(&format, &mut data);
    let mut stdout = MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    let mut selection: Option<Selection> = None;
    redraw(&mut stdout, &mut display, &selection);
    for input in std::io::stdin().events() {
        let evt = input.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    let pos = (x as usize, y as usize);
                    let cell = get_cell(&mut display, pos);
                    match selection {
                        Some(sel) => if cell.value == ' ' && sel.cell.value != ' ' {
                            swap_cells(&mut display, sel.pos, pos)
                        },
                        None => (),
                    }
                    selection = Some(Selection {
                        pos: pos,
                        cell: get_cell(&mut display, pos),
                    });
                    redraw(&mut stdout, &mut display, &selection);
                }
                _ => (),
            },
            _ => {}
        }
    }
}
