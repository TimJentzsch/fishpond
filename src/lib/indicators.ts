import { KING, type Chess, type Square } from 'chess.js';
import { getBoardPieces } from './board';

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

export type CheckInfo = {
	square: Square;
	isMate: boolean;
};

export function getCheckInfo(chess: Chess): CheckInfo | undefined {
	if (!chess.isCheck() && !chess.isCheckmate()) {
		return undefined;
	}

	const isMate = chess.isCheckmate();

	const boardPieces = [...getBoardPieces(chess)];
	const turn = chess.turn();

	const king = boardPieces.find((pieceInfo) => {
		return pieceInfo !== null && pieceInfo.color === turn && pieceInfo.type === KING;
	});

	if (king === undefined) {
		return undefined;
	}

	return {
		square: king.square,
		isMate
	};
}
