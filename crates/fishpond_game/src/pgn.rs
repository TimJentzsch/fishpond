use std::fmt::Display;

use shakmaty::{san::San, Position};

use crate::Game;

/// Portable game notation (PGN) to record an entire chess game.
pub struct Pgn<P: Position> {
    game: Game<P>,
}

impl<P: Position> Pgn<P> {
    /// Create a portable game notation (PGN) for the given game.
    pub fn from_game(game: Game<P>) -> Self {
        Pgn { game }
    }
}

impl<P: Position + Clone> Display for Pgn<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Mandatory tags
        // TODO: Provide means to fill these out properly
        writeln!(f, "[Event \"?\"]")?;
        writeln!(f, "[Site \"fishpond\"]")?;
        writeln!(f, "[Date \"????.??.??\"]")?;
        writeln!(f, "[Round \"?\"]")?;
        writeln!(f, "[White \"?\"]")?;
        writeln!(f, "[Black \"?\"]")?;

        let result = match self.game.outcome() {
            Some(shakmaty::Outcome::Draw) => "1/2-1/2",
            Some(shakmaty::Outcome::Decisive { winner }) => match winner {
                shakmaty::Color::White => "1-0",
                shakmaty::Color::Black => "0-1",
            },
            None => "*",
        };

        writeln!(f, "[Result \"{result}\"]")?;
        writeln!(f)?;

        let mut current_position = self.game.start_position().clone();

        for (index, r#move) in self.game.moves().enumerate() {
            if index % 2 == 0 {
                // Move number
                if index > 0 {
                    write!(f, " {}.", index + 1)?;
                } else {
                    write!(f, "{}.", index + 1)?;
                }
            }

            // Move in SAN notation
            write!(f, " {}", San::from_move(&current_position, r#move))?;

            // Update position
            // All moves in the game are expected to be validated already
            current_position.play_unchecked(r#move);
        }

        if result != "*" {
            write!(f, " {result}")?;
        }

        Ok(())
    }
}
