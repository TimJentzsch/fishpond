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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            Self::Go { move_time } => write!(f, "go movetime {}", move_time.as_millis()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("uciok", UciToGuiCmd::UciOk)]
    #[case("id name Stockfish 16", UciToGuiCmd::Id { name: Some("Stockfish 16".to_string()), author: None })]
    #[case("id author the Stockfish developers (see AUTHORS file)", UciToGuiCmd::Id { name: None, author: Some("the Stockfish developers (see AUTHORS file)".to_string()) })]
    #[case("bestmove e2e4 ponder e7e5", UciToGuiCmd::BestMove { uci_move: Uci::from_str("e2e4").unwrap() })]
    fn test_uci_to_gui_cmd_valid(#[case] input: &str, #[case] expected: UciToGuiCmd) {
        assert_eq!(input.parse::<UciToGuiCmd>().unwrap(), expected);
    }

    #[rstest]
    #[case(UciToEngineCmd::Uci, "uci")]
    #[case(UciToEngineCmd::Position { game: Game::from_start_position(Chess::new()).into() }, "position startpos")]
    #[case(UciToEngineCmd::Go { move_time: Duration::from_millis(1234) }, "go movetime 1234")]
    fn test_uci_to_engine_cmd_display(#[case] input: UciToEngineCmd, #[case] expected: &str) {
        assert_eq!(format!("{input}"), expected.to_string());
    }
}
