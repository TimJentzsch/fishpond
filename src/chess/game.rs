use bevy::ecs::component::Component;
use shakmaty::{Color, Move, Position};

#[derive(Debug, Clone, Component)]
pub struct Game<P>
where
    P: Position,
{
    start_position: P,
    actions: Vec<Action>,
    current_position: P,
}

#[derive(Debug, Clone)]
enum Action {
    /// A move on the board.
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

    /// Obtain the current position of the game.
    pub fn current_position(&self) -> &P {
        &self.current_position
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
