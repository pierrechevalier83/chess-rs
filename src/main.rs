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

fn main() {
    let format = Format::new(7, 3);
    let board = xchess::read_xchess("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    let data = matrix::Matrix::new(8, board);
    let mut display = MatrixDisplay::new(format, data);
    let mut stdout = MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    clear(&mut stdout);
    display.print(&mut stdout, &style::BordersStyle::None);
    for input in std::io::stdin().events() {
        let evt = input.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    let previous_bg_color;
                    {
                        let mut cell = display.cell_at_cursor_position((x as usize, y as usize));
                        previous_bg_color = cell.color.bg;
                        cell.color.bg = 10;
                    }
                    clear(&mut stdout);
                    display.print(&mut stdout, &style::BordersStyle::None);
                    display
                        .cell_at_cursor_position((x as usize, y as usize))
                        .color
                        .bg = previous_bg_color;
                }
                _ => (),
            },
            _ => {}
        }
    }
}
