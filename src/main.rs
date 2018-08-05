extern crate chess;
extern crate matrix_display;
extern crate termion;

use matrix_display::*;
use termion::input::{MouseTerminal, TermRead};
use termion::event::{Event, Key, MouseEvent};
use termion::raw::IntoRawMode;

enum AnsiColor {
    White = 0,
    Dark = 7,
    Grey = 23,
    Blue = 33,
}

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

fn square_bg(square: chess::Square, selection: &Option<chess::Square>) -> AnsiColor {
    if Some(square) == *selection {
        AnsiColor::Grey
    } else if (square.get_rank().to_index() + square.get_file().to_index()) % 2 == 0 {
        AnsiColor::White
    } else {
        AnsiColor::Dark
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

fn rank_to_display(
    board: chess::Board,
    rank: &chess::Rank,
    selection: &Option<chess::Square>,
) -> Vec<cell::Cell<char>> {
    chess::ALL_FILES
        .iter()
        .map(|file| {
            let square = chess::Square::make_square(*rank, *file);
            cell::Cell::new(
                piece_on_square(board, square, color_of_piece(board, square)),
                AnsiColor::Blue as u8,
                square_bg(square, selection) as u8,
            )
        })
        .collect()
}

fn board_to_display(
    board: chess::Board,
    selection: &Option<chess::Square>,
) -> matrix::Matrix<cell::Cell<char>> {
    let all_squares = chess::ALL_RANKS
        .iter()
        .rev()
        .flat_map(|rank| rank_to_display(board, rank, selection))
        .collect();
    matrix::Matrix::new(chess::NUM_RANKS, all_squares)
}

fn initial_grid() -> chess::Board {
    let starting_position = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    chess::Board::from_fen(starting_position.to_string()).unwrap()
}

fn is_legal_move(board: chess::Board, chess_move: chess::ChessMove) -> bool {
    board.legal(chess_move)
}

fn click_to_square(click: (u16, u16)) -> chess::Square {
    let format = Format::new(7, 3);
    let mut data = matrix::Matrix::new(
        chess::NUM_RANKS,
        [cell::Cell::new(' ', 0, 0)]
            .iter()
            .cycle()
            .take(64)
            .cloned()
            .collect::<Vec<_>>(),
    );
    let display = MatrixDisplay::new(&format, &mut data);
    println!("Clicked {}, {}", click.0, click.1);
    let pos = display.coordinates_at_cursor_position((click.0 as usize, click.1 as usize));
    println!("Pos {}, {}", pos.0, pos.1);
    let sq = chess::Square::make_square(
        chess::Rank::from_index(chess::NUM_RANKS - 1 - pos.1),
        chess::File::from_index(pos.0),
    );
    println!("Square {}", sq);
    sq
}

fn draw(
    mut stdout: &mut MouseTerminal<termion::raw::RawTerminal<std::io::Stdout>>,
    board: chess::Board,
    selection: &Option<chess::Square>,
) {
    let format = Format::new(7, 3);
    let mut data = board_to_display(board, selection);
    let display = MatrixDisplay::new(&format, &mut data);
    clear(&mut stdout);
    display.print(&mut stdout, &style::BordersStyle::None);
}

fn main() {
    let mut board = initial_grid();
    let mut selection: Option<chess::Square> = None;
    let mut stdout = MouseTerminal::from(std::io::stdout().into_raw_mode().unwrap());
    draw(&mut stdout, board, &selection);
    for input in std::io::stdin().events() {
        let evt = input.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    let square = click_to_square((x, y));
                    match selection {
                        Some(previous_selection) => {
                            let attempted_move =
                                chess::ChessMove::new(previous_selection, square, None);
                            if is_legal_move(board, attempted_move) {
                                board = board.make_move(attempted_move);
                            }
                            selection = None;
                        }
                        None => {
                            if board.piece_on(square) != None {
                                selection = Some(square);
                            }
                        }
                    }
                    draw(&mut stdout, board, &selection);
                }
                _ => (),
            },
            _ => {}
        }
    }
}
