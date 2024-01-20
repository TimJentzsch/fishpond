use std::{error::Error, fmt::Display};

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
use shakmaty::{
    fen::Fen,
    zobrist::{Zobrist128, ZobristHash},
    Color, Move, Position, Role,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DeclareDrawReason {
    /// The same position was repeated at least 3 times.
    ///
    /// Note that this applies to *positions*, not *moves*.
    /// The positions may have occurred in any order.
    ///
    /// Under FIDE rules, a fivefold repetition is an instant draw, without any players having to declare it.
    Repetition {
        /// The number of times this position has been repeated.
        ///
        /// This will be at least 3.
        ///
        /// Under FIDE rules, the game is automatically declared a draw if the position was repeated 5 times.
        count: usize,
        claimed_by: Color,
    },

    /// The fifty-move rule in chess states that a player can claim a draw
    /// if no capture has been made and no pawn has been moved in the last fifty moves
    /// (for this purpose a "move" consists of a player completing a turn followed by the opponent completing a turn).
    FiftyMoveRule {
        /// The number of *plies* without capture or pawn push.
        ///
        /// Note that a *move* equals two *plies*, so the count will be at least 100.
        ///
        /// Under FIDE rules, the game is automatically declared a draw if for 75 moves, so when the count reaches 150.
        ply_count: usize,
    },
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DecisiveReason {
    /// The winner delivered checkmate.
    Checkmate,
    /// The other player resigned.
    Resigned,
    /// Win by variant rules.
    Variant,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum DrawReason {
    /// The current player cannot make any moves, but is not in check.
    Stalemate,
    /// No player is able to deliver checkmate.
    InsufficientMaterial,
    /// Both players agreed to a draw.
    MutualAgreement,
    /// One of the players declared a draw, for the given reason.
    Declared(DeclareDrawReason),
    /// Draw by variant rules.
    Variant,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Outcome {
    /// A player one the game.
    Decisive {
        /// The player who won the game.
        winner: Color,
        /// The reason why the game was declared a win.
        reason: DecisiveReason,
    },
    /// The game ended in a draw.
    Draw {
        /// The reason why the game was declared a draw.
        reason: DrawReason,
    },
}

impl From<Outcome> for shakmaty::Outcome {
    fn from(value: Outcome) -> Self {
        match value {
            Outcome::Decisive { winner, reason: _ } => shakmaty::Outcome::Decisive { winner },
            Outcome::Draw { reason: _ } => shakmaty::Outcome::Draw,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Component))]
pub struct Game<P>
where
    P: Position,
{
    /// The position from which the game started.
    start_position: P,

    /// All actions that have happened throughout the game.
    actions: Vec<Action>,

    /// The current position of the game.
    ///
    /// This is position resulting in playing all moves from [`actions`](Game::actions) on [`start_position`](Game::start_position).
    current_position: P,

    /// Zobrist hashes of all positions which have occurred during the game.
    ///
    /// Used to determine repetition draws.
    position_hashes: Vec<Zobrist128>,

    /// The number of *plies* without captures or pawn pushes.
    ///
    /// Used to determine fifty-move rule draws.
    ///
    /// Note that this counts *plies* and not *moves*.
    fifty_move_plies: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// A move on the board.
    ///
    /// This must be a valid move in the current position.
    Move(Move),

    /// The given color offers the opponent a draw.
    OfferDraw(Color),

    /// The opponent accepted the previously offered draw.
    AcceptDraw,

    /// A draw is declared by a player.
    DeclareDraw(DeclareDrawReason),

    /// The given color resigns the game.
    Resign(Color),
}

/// The action is invalid in this position
#[derive(Debug)]
pub struct InvalidAction;

impl Display for InvalidAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "this action is not valid in the current game state")
    }
}

impl Error for InvalidAction {}

impl<P> Game<P>
where
    P: Position + Clone,
{
    /// Create a new game starting at the given position.
    pub fn from_start_position(start_position: P) -> Self {
        Self {
            current_position: start_position.clone(),
            position_hashes: vec![start_position.zobrist_hash(shakmaty::EnPassantMode::Legal)],
            start_position,
            actions: Vec::new(),
            fifty_move_plies: 0,
        }
    }

    /// The start position of the game.
    pub fn start_position(&self) -> &P {
        &self.start_position
    }

    /// Obtain the current position of the game.
    pub fn current_position(&self) -> &P {
        &self.current_position
    }

    /// All actions which happened in the game.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// An iterator over all moves played in the game.
    pub fn moves(&self) -> impl Iterator<Item = &Move> {
        self.actions.iter().filter_map(|action| {
            if let Action::Move(r#move) = action {
                Some(r#move)
            } else {
                None
            }
        })
    }

    /// Offer a draw to the opponent.
    /// `color` is the player who offered the draw.
    ///
    /// The draw must accepted before the opponent moves.
    ///
    /// If the game is already over, [`Err`] is returned.
    pub fn offer_draw(&mut self, color: Color) -> Result<(), InvalidAction> {
        if self.outcome().is_some() {
            Err(InvalidAction)
        } else {
            self.actions.push(Action::OfferDraw(color));
            Ok(())
        }
    }

    /// Accept a draw offer from the opponent.
    ///
    /// The opponent must have offered a draw first, otherwise [`Err`] is returned.
    pub fn accept_draw(&mut self) -> Result<(), InvalidAction> {
        if self.outcome().is_some() {
            return Err(InvalidAction);
        }

        let mut iter = self.actions.iter().rev();

        if let Some(last) = iter.next() {
            match last {
                Action::OfferDraw(_) => {
                    self.actions.push(Action::AcceptDraw);
                    Ok(())
                }
                Action::Move(_) => {
                    if Some(&Action::OfferDraw(self.turn().other())) == iter.next() {
                        self.actions.push(Action::AcceptDraw);
                        Ok(())
                    } else {
                        Err(InvalidAction)
                    }
                }
                _ => Err(InvalidAction),
            }
        } else {
            Err(InvalidAction)
        }
    }

    /// `color` resigns the game.
    ///
    /// Returns [`Err`] if the game is already over.
    pub fn resign(&mut self, color: Color) -> Result<(), InvalidAction> {
        if self.outcome().is_some() {
            Err(InvalidAction)
        } else {
            self.actions.push(Action::Resign(color));
            Ok(())
        }
    }

    /// The position with move history in UCI notation.
    pub fn uci_position_with_moves(&self) -> String {
        let uci_start =
            Fen::from_position(self.start_position.clone(), shakmaty::EnPassantMode::Legal);
        let uci_moves: Vec<_> = self
            .moves()
            .map(|r#move| r#move.to_uci(shakmaty::CastlingMode::Standard).to_string())
            .collect();

        if uci_moves.is_empty() {
            format!("fen {uci_start}")
        } else {
            format!("fen {uci_start} moves {}", uci_moves.join(" "))
        }
    }

    /// Determine if a draw can be declared.
    pub fn can_declare_draw(&self) -> Option<DeclareDrawReason> {
        if self.outcome().is_some() {
            return None;
        }

        // FIFTY-MOVE RULE
        if self.fifty_move_plies >= 50 {
            return Some(DeclareDrawReason::FiftyMoveRule {
                ply_count: self.fifty_move_plies,
            });
        }

        // REPETITIONS
        if self.position_hashes.is_empty() {
            return None;
        }

        // Either: The last move resulted in a position which occurred at least 3 times in the game
        let last_hash = self.position_hashes.last().unwrap();
        let repetitions = self
            .position_hashes
            .iter()
            .filter(|hash| *hash == last_hash)
            .count();
        if repetitions >= 3 {
            return Some(DeclareDrawReason::Repetition {
                count: repetitions,
                claimed_by: self.current_position().turn(),
            });
        }

        // Or: The current player can make a move which would result in a position repeated at least 3 times
        for r#move in self.current_position().legal_moves() {
            let mut new_position = self.current_position().clone();
            new_position.play_unchecked(&r#move);
            let last_hash: Zobrist128 = new_position.zobrist_hash(shakmaty::EnPassantMode::Legal);
            let repetitions = self
                .position_hashes
                .iter()
                .filter(|hash| **hash == last_hash)
                .count()
                + 1;
            if repetitions >= 3 {
                return Some(DeclareDrawReason::Repetition {
                    count: repetitions,
                    claimed_by: self.current_position().turn(),
                });
            }
        }

        None
    }

    /// Try to declare the game a draw.
    ///
    /// Returns [`Err`] if [`Game::can_declare_draw`] returns [`None`].
    pub fn declare_draw(&mut self) -> Result<(), InvalidAction> {
        if let Some(reason) = self.can_declare_draw() {
            self.actions.push(Action::DeclareDraw(reason));
            Ok(())
        } else {
            Err(InvalidAction)
        }
    }

    /// Check if the game has ended and get the corresponding reason.
    ///
    /// Returns [`None`] if the game is still ongoing.
    pub fn game_outcome(&self) -> Option<Outcome> {
        if let Some(variant_outcome) = self.variant_outcome() {
            // An outcome determined by the variant
            // Just needs to be converted and the variant reason attached
            Some(match variant_outcome {
                shakmaty::Outcome::Decisive { winner } => Outcome::Decisive {
                    winner,
                    reason: DecisiveReason::Variant,
                },
                shakmaty::Outcome::Draw => Outcome::Draw {
                    reason: DrawReason::Variant,
                },
            })
        } else if self.legal_moves().is_empty() {
            Some(if self.is_check() {
                // Checkmate
                Outcome::Decisive {
                    winner: !self.turn(),
                    reason: DecisiveReason::Checkmate,
                }
            } else {
                // Stalemate
                Outcome::Draw {
                    reason: DrawReason::Stalemate,
                }
            })
        } else if self.is_insufficient_material() {
            // Insufficient material
            Some(Outcome::Draw {
                reason: DrawReason::InsufficientMaterial,
            })
        } else {
            // Check if a player action ended the game
            match self.actions.last() {
                // Resigned
                Some(Action::Resign(color)) => Some(Outcome::Decisive {
                    winner: color.other(),
                    reason: DecisiveReason::Resigned,
                }),
                // Draw by agreement
                Some(Action::AcceptDraw) => Some(Outcome::Draw {
                    reason: DrawReason::MutualAgreement,
                }),
                // Draw declared
                Some(Action::DeclareDraw(reason)) => Some(Outcome::Draw {
                    reason: DrawReason::Declared(*reason),
                }),
                _ => None,
            }
        }
    }
}

impl<P: Position + Clone> Position for Game<P> {
    fn board(&self) -> &shakmaty::Board {
        self.current_position().board()
    }

    fn promoted(&self) -> shakmaty::Bitboard {
        self.current_position().promoted()
    }

    fn pockets(&self) -> Option<&shakmaty::ByColor<shakmaty::ByRole<u8>>> {
        self.current_position().pockets()
    }

    fn turn(&self) -> Color {
        self.current_position().turn()
    }

    fn castles(&self) -> &shakmaty::Castles {
        self.current_position().castles()
    }

    fn maybe_ep_square(&self) -> Option<shakmaty::Square> {
        self.current_position().maybe_ep_square()
    }

    fn remaining_checks(&self) -> Option<&shakmaty::ByColor<shakmaty::RemainingChecks>> {
        self.current_position().remaining_checks()
    }

    fn halfmoves(&self) -> u32 {
        self.current_position().halfmoves()
    }

    fn fullmoves(&self) -> std::num::NonZeroU32 {
        self.current_position().fullmoves()
    }

    fn into_setup(self, mode: shakmaty::EnPassantMode) -> shakmaty::Setup {
        self.current_position.into_setup(mode)
    }

    fn legal_moves(&self) -> shakmaty::MoveList {
        self.current_position().legal_moves()
    }

    fn is_variant_end(&self) -> bool {
        self.current_position().is_variant_end()
    }

    fn has_insufficient_material(&self, color: Color) -> bool {
        self.current_position().has_insufficient_material(color)
    }

    fn variant_outcome(&self) -> Option<shakmaty::Outcome> {
        self.current_position().variant_outcome()
    }

    fn outcome(&self) -> Option<shakmaty::Outcome> {
        // A game has more ways to end than just the position
        self.game_outcome().map(|outcome| outcome.into())
    }

    fn play_unchecked(&mut self, m: &Move) {
        // Track the move in the history
        self.actions.push(Action::Move(m.clone()));
        // Update the current position
        self.current_position.play_unchecked(m);

        // Track hashes for repetitions
        self.position_hashes.push(
            self.current_position()
                .zobrist_hash(shakmaty::EnPassantMode::Legal),
        );
        // Track plies for fifty-move rule
        if m.is_capture() || m.is_castle() || m.role() == Role::Pawn {
            self.fifty_move_plies = 0;
        } else {
            self.fifty_move_plies += 1;
        }
    }
}
