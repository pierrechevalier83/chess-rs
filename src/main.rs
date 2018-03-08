extern crate chess;
extern crate matrix_display;
extern crate termion;

use matrix_display::*;
use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, Key, MouseEvent};
use termion::raw::IntoRawMode;

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

fn square_bg_in_ansi(square: chess::Square) -> u8 {
    let ansi_white = 0;
    let ansi_dark = 7;
    if (square.get_rank().to_index() + square.get_file().to_index()) % 2 == 0 {
        ansi_white
    } else {
        ansi_dark
    }
}

fn to_unicode(piece: chess::Piece, color: chess::Color) -> char {
    use chess::Piece::*;
    match color {
        chess::Color::White => match piece {
            Pawn => '♙',
            Knight => '♘',
            Bishop => '♗',
            Rook => '♖',
            Queen => '♕',
            King => '♔',
        },
        chess::Color::Black => match piece {
            Pawn => '♟',
            Knight => '♞',
            Bishop => '♝',
            Rook => '♜',
            Queen => '♛',
            King => '♚',
        },
    }
}

fn piece_on_square(board: chess::Board, square: chess::Square, color: chess::Color) -> char {
    match board.piece_on(square) {
        Some(piece) => to_unicode(piece, color),
        None => ' ',
    }
}

fn color_of_piece(board: chess::Board, square: chess::Square) -> chess::Color {
    let bb = chess::BitBoard::from_square(square);
    if (board.color_combined(chess::Color::White) & bb) == bb {
        chess::Color::White
    } else {
        chess::Color::Black
    }
}

fn rank_to_display(board: chess::Board, rank: &chess::Rank) -> Vec<cell::Cell<char>> {
    let ansi_blue = 33;
    chess::ALL_FILES
        .iter()
        .map(|file| {
            let square = chess::Square::make_square(*rank, *file);
            cell::Cell::new(
                piece_on_square(board, square, color_of_piece(board, square)),
                ansi_blue,
                square_bg_in_ansi(square),
            )
        })
        .collect()
}

fn board_to_display(board: chess::Board) -> matrix::Matrix<cell::Cell<char>> {
    let all_squares = chess::ALL_RANKS
        .iter()
        .rev()
        .flat_map(|rank| rank_to_display(board, rank))
        .collect();
    matrix::Matrix::new(chess::NUM_RANKS, all_squares)
}

fn initial_grid() -> chess::Board {
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    chess::Board::from_fen(starting_position.to_string()).unwrap()
}

fn main() {
    let format = Format::new(7, 3);
    let board = initial_grid();
    let mut data = board_to_display(board);
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
