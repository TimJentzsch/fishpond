import type { Chess, Square } from 'chess.js';

/**
 * Get the two squares of the last move in an array.
 *
 * @param chess The chess game to get the last move squares from.
 * @returns The two squares of the last move or an empty array if there was no move yet.
 */
export function getLastMoveSquares(chess: Chess): Square[] {
	const moveHistory = chess.history({ verbose: true });

	if (moveHistory.length === 0) {
		return [];
	}

	const lastMove = moveHistory[moveHistory.length - 1];
	return [lastMove.from, lastMove.to];
}
