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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScoreValue {
    /// The score from the engine's point of view in centipawns.
    Centipawns(i16),

    /// Number of moves (not plies) to mate.
    ///
    /// If the engine is getting mated, use negative values.
    Mate(i16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ScoreBound {
    Exact,

    /// The score is just a lower bound.
    Lower,

    /// The score is just an upper bound.
    Upper,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refutation {
    /// The move the refutation is sent for.
    r#move: Uci,

    /// The line that refutes the move.
    refutation: Vec<Uci>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Score {
    value: ScoreValue,
    bound: ScoreBound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentLine {
    cpu_number: Option<usize>,
    line: Vec<Uci>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UciInfo {
    /// Search depth in plies.
    depth: Option<usize>,

    /// Selective search depth in plies.
    ///
    /// If the engine sends `seldepth` there must also be a `depth` present in the same string.
    selective_depth: Option<usize>,

    /// The time searched, this should be sent together with the `pv`.
    time: Option<Duration>,

    /// Number of nodes searched, the engine should send this info regularly.
    nodes: Option<usize>,

    /// The best line found.
    principal_variation: Option<Vec<Uci>>,

    /// This is for the multi pv mode.
    ///
    /// For the best move/pv, add `multipv 1` in the string where you send the pv.
    ///
    /// In k-best mode always send all k variants in k strings together.
    multi_pv: Option<usize>,

    /// The score of this line.
    score: Option<Score>,

    /// Currently searching this move.
    current_move: Option<Uci>,

    /// Currently searching the move of this number, for the first move should be 1 not 0.
    current_move_number: Option<usize>,

    /// The percentage that the hash table is fill, as a fraction in [0, 1].
    ///
    /// The engine should send this info regularly.
    hash_full: Option<f32>,

    /// Number of nodes searched per second.
    ///
    /// The engine should send this info regularly.
    nodes_per_second: Option<usize>,

    /// Number of positions that were found in the endgame table bases.
    table_hits: Option<usize>,

    /// Number of positions found in the shredder endgame databases.
    shredder_hits: Option<usize>,

    /// The CPU usage of the engine, as a fraction in [0, 1].
    cpu_load: Option<f32>,

    /// Any string which will be displayed by the engine.
    ///
    /// If there is a `string` command the rest of the line will be interpreted as string.
    string: Option<String>,

    /// The first move is refuted by this line.
    ///
    /// If there is no refutation for the move found, the line can be empty.
    ///
    /// The engine should only send this if the option `UCI_ShowRefutations` is set to true.
    refutation: Refutation,

    /// This is the current line the engine is calculating.
    ///
    /// The engine should only send this if the option `UCI_ShowCurrLine` is set to true.
    current_line: CurrentLine,
}

/// A UCI command sent from the engine to the GUI.
#[derive(Debug, Clone, PartialEq)]
pub enum UciToGuiCmd {
    UciOk,
    Id {
        name: Option<String>,
        author: Option<String>,
    },
    Info(Box<UciInfo>),
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
