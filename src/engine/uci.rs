use std::{error::Error, fmt::Display, str::FromStr, time::Duration};

use fishpond_game::Game;
use shakmaty::{uci::Uci, Chess};

#[derive(Debug)]
pub struct UciParseError;

impl Display for UciParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse UCI string")
    }
}

impl Error for UciParseError {}

/// A UCI command sent from the engine to the GUI.
#[derive(Debug, Clone)]
pub enum UciToGuiCmd {
    UciOk,
    Id {
        name: Option<String>,
        author: Option<String>,
    },
    BestMove {
        uci_move: Uci,
    },
}

impl FromStr for UciToGuiCmd {
    type Err = UciParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        if let Some(command) = tokens.next() {
            match command {
                "uciok" => Ok(UciToGuiCmd::UciOk),
                "id" => {
                    if let Some(id_type) = tokens.next() {
                        let rest = tokens.collect::<Vec<_>>().join(" ");

                        if rest.is_empty() {
                            Err(UciParseError)
                        } else {
                            match id_type {
                                "name" => Ok(UciToGuiCmd::Id {
                                    name: Some(rest),
                                    author: None,
                                }),
                                "author" => Ok(UciToGuiCmd::Id {
                                    name: None,
                                    author: Some(rest),
                                }),
                                _ => Err(UciParseError),
                            }
                        }
                    } else {
                        Err(UciParseError)
                    }
                }
                "bestmove" => {
                    if let Some(uci_str) = tokens.next() {
                        if let Ok(uci_move) = uci_str.parse() {
                            Ok(UciToGuiCmd::BestMove { uci_move })
                        } else {
                            Err(UciParseError)
                        }
                    } else {
                        Err(UciParseError)
                    }
                }
                _ => Err(UciParseError),
            }
        } else {
            Err(UciParseError)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UciToEngineCmd {
    Uci,
    Position { game: Box<Game<Chess>> },
    Go { move_time: Duration },
}

impl Display for UciToEngineCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uci => write!(f, "uci"),
            Self::Position { game } => {
                write!(f, "position {}", game.uci_position_with_moves())
            }
            Self::Go { move_time } => writeln!(f, "go movetime {}", move_time.as_millis()),
        }
    }
}
