use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{event, queue};
use std::io::Write;
use std::{fmt, io};

fn main() -> io::Result<()> {
    let mut std_out = io::stdout();
    enable_raw_mode()?;
    queue!(
        std_out,
        Hide,
        EnableMouseCapture,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    std_out.flush()?;
    let mut board = Board {
        tiles: [Tile::Empty; 9],
    };
    println!("{}\r", &board);

    let mut player = Player::X;

    loop {
        match event::read()? {
            event::Event::Mouse(event::MouseEvent {
                kind: event::MouseEventKind::Up(event::MouseButton::Left),
                column,
                row,
                ..
            }) => {
                let game_column = match column {
                    0..=2 => 0,
                    4..=6 => 1,
                    8..=10 => 2,
                    _ => continue,
                };
                let game_row = match row {
                    0 => 0,
                    2 => 1,
                    4 => 2,
                    _ => continue,
                };

                let i = game_row * 3 + game_column;

                if board.tiles[i] == Tile::Empty {
                    board.tiles[i] = match player {
                        Player::X => Tile::X,
                        Player::O => Tile::O,
                    };
                    player = player.next();
                }

                if let Some(outcome) = board.check_outcome() {
                    queue!(std_out, Clear(ClearType::All), MoveTo(0, 0))?;
                    println!("{}\r", &board);
                    println!("{}\r", outcome);
                    break;
                }
            }
            event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
            })
            | event::Event::Key(event::KeyEvent {
                code: event::KeyCode::Char('q'),
                ..
            }) => break,
            _ => continue,
        }
        queue!(std_out, Clear(ClearType::All), MoveTo(0, 0))?;
        println!("{}\r", &board);
    }

    queue!(std_out, DisableMouseCapture, Show)?;
    disable_raw_mode()?;
    std_out.flush()?;

    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    X,
    O,
}

#[derive(Debug)]
enum Outcome {
    Tie,
    X,
    O,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Tie => write!(f, "It's a draw..."),
            Outcome::X => write!(f, "X wins!"),
            Outcome::O => write!(f, "O wins!"),
        }
    }
}

enum Player {
    X,
    O,
}

impl Player {
    fn next(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

struct Board {
    tiles: [Tile; 9],
}

impl Board {
    fn check_outcome(&self) -> Option<Outcome> {
        if let Some(value) = self.check_horiz(0) {
            return Some(value);
        }
        if let Some(value) = self.check_horiz(3) {
            return Some(value);
        }
        if let Some(value) = self.check_horiz(6) {
            return Some(value);
        }

        if let Some(value) = self.check_vert(0) {
            return Some(value);
        }
        if let Some(value) = self.check_vert(1) {
            return Some(value);
        }
        if let Some(value) = self.check_vert(2) {
            return Some(value);
        }

        if self.tiles[0] == self.tiles[4] && self.tiles[0] == self.tiles[8] {
            match self.tiles[0] {
                Tile::Empty => {}
                Tile::X => return Some(Outcome::X),
                Tile::O => return Some(Outcome::O),
            }
        }

        if self.tiles[2] == self.tiles[4] && self.tiles[2] == self.tiles[6] {
            match self.tiles[2] {
                Tile::Empty => {}
                Tile::X => return Some(Outcome::X),
                Tile::O => return Some(Outcome::O),
            }
        }

        for tile in self.tiles {
            if tile == Tile::Empty {
                return None;
            }
        }

        Some(Outcome::Tie)
    }

    fn check_horiz(&self, i: usize) -> Option<Outcome> {
        if self.tiles[i] == self.tiles[i + 1] && self.tiles[i] == self.tiles[i + 2] {
            match self.tiles[i] {
                Tile::Empty => {}
                Tile::X => return Some(Outcome::X),
                Tile::O => return Some(Outcome::O),
            }
        }
        None
    }

    fn check_vert(&self, i: usize) -> Option<Outcome> {
        if self.tiles[i] == self.tiles[i + 3] && self.tiles[i] == self.tiles[i + 6] {
            match self.tiles[i] {
                Tile::Empty => {}
                Tile::X => return Some(Outcome::X),
                Tile::O => return Some(Outcome::O),
            }
        }
        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % 3 == 0 {
                if i != 0 {
                    write!(f, "\r\n───┼───┼───\r\n")?;
                }
                write!(f, " ")?;
            }
            match tile {
                Tile::Empty => write!(f, " ")?,
                Tile::X => write!(f, "X")?,
                Tile::O => write!(f, "O")?,
            }
            if (i + 1) % 3 != 0 {
                write!(f, " │ ")?;
            }
        }

        Ok(())
    }
}
