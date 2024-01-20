use std::{error::Error, fmt::Display};

#[cfg(feature = "bevy")]
use bevy_ecs::component::Component;
use shakmaty::{fen::Fen, Color, Move, Position};

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
    DeclareDraw,

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
            start_position,
            actions: Vec::new(),
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

    fn play_unchecked(&mut self, m: &Move) {
        self.actions.push(Action::Move(m.clone()));
        self.current_position.play_unchecked(m);
    }
}