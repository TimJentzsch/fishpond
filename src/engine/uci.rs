use std::{error::Error, fmt::Display, str::FromStr};

use shakmaty::uci::Uci;

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
    BestMove { uci_move: Uci },
}

impl FromStr for UciToGuiCmd {
    type Err = UciParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();

        if let Some(command) = tokens.next() {
            match command {
                "uciok" => Ok(UciToGuiCmd::UciOk),
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
