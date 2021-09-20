#![warn(missing_copy_implementations)]

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Event {
    Play(usize),
    Played { player: Player, pos: usize },
    End(Outcome),
    Turn,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }

    pub fn tile(self) -> Tile {
        match self {
            Self::X => Tile::X,
            Self::O => Tile::O,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Outcome {
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

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    X,
    O,
}

#[derive(Clone, Copy)]
pub struct Board {
    pub tiles: [Tile; 9],
}

impl Board {
    pub fn check_outcome(&self) -> Option<Outcome> {
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
