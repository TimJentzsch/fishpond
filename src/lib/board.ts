import { BLACK, WHITE, type Color, type PieceSymbol, type Square } from 'chess.js';

export type Rank = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8';
export type File = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h';

export type PieceInfo = {
	square: Square;
	type: PieceSymbol;
	color: Color;
};
export type Board = (PieceInfo | null)[][];

/** The indexes defining a square on the board. */
export type SquarePos = {
	/** The coordinate of the square file, a number between `0` and `7` (mapping to file `a` and `h`). */
	file: number;
	/** The coordinate of the square rank, a number between `0` and `7` (mapping to rank `1` and `8`). */
	rank: number;
};

/**
 * The position along one axis (i.e., file or rank).
 *
 * Returns the numbers from `0` to `7` or `7` to `1` if `reversed` is `true`.
 */
export function getAxisPositions(reversed = false): number[] {
	const indexes = [0, 1, 2, 3, 4, 5, 6, 7];

	if (reversed) {
		indexes.reverse();
	}

	return indexes;
}

/** Get the positions of all squares on the board. */
export function getBoardPositions(): SquarePos[] {
	const boardPositions: SquarePos[] = [];

	for (const rank of getAxisPositions()) {
		for (const file of getAxisPositions()) {
			boardPositions.push({ rank, file });
		}
	}

	return boardPositions;
}

/** Determine the color of a square at the given position. */
export function getSquareColor(squarePos: SquarePos): Color {
	return (squarePos.rank + squarePos.file) % 2 == 0 ? BLACK : WHITE;
}

/** Convert a rank position to its string identifier. */
export function rankPosToStr(rankPos: number): Rank {
	switch (rankPos) {
		case 0:
			return '1';
		case 1:
			return '2';
		case 2:
			return '3';
		case 3:
			return '4';
		case 4:
			return '5';
		case 5:
			return '6';
		case 6:
			return '7';
		case 7:
			return '8';
		default:
			throw Error(`Invalid rank index ${rankPos}`);
	}
}

/** Convert a file position to its string identifier. */
export function filePosToStr(index: number): File {
	switch (index) {
		case 0:
			return 'a';
		case 1:
			return 'b';
		case 2:
			return 'c';
		case 3:
			return 'd';
		case 4:
			return 'e';
		case 5:
			return 'f';
		case 6:
			return 'g';
		case 7:
			return 'h';
		default:
			throw Error(`Invalid file index ${index}`);
	}
}

/**
 * Get a `style` string setting the `top` and `left` attributes to position the square.
 *
 * @param squarePos The position of the square on the board.
 * @param isFlipped `true` if the board is flipped, i.e. rank `1` is at the top of the board.
 * @returns A `style` string with `top` and `left` attributes to position the square.
 */
export function getSquarePosMargins(squarePos: SquarePos, isFlipped: boolean): string {
	const perIndex = 100 / 8;

	const left = squarePos.file * perIndex;
	const top = isFlipped ? squarePos.rank * perIndex : 100 - perIndex - squarePos.rank * perIndex;

	return `top: ${top}%; left: ${left}%`;
}

/**
 * Convert a square in string notation to its position on the board.
 *
 * @param square The square in string notation, e.g. `c4`.
 * @returns The position of the square.
 */
export function squareToSquarePos(square: Square): SquarePos {
	const file = square.charCodeAt(0) - 'a'.charCodeAt(0);
	const rank = square.charCodeAt(1) - '1'.charCodeAt(0);

	return {
		file,
		rank
	};
}

/**
 * Get the `style` string for a square to position it on the board.
 *
 * @param square The square in string notation, e.g. `c4`.
 * @param isFlipped `true` if the board is flipped, i.e. rank `1` is at the top of the board.
 * @returns A `style` string with `top` and `left` set to position the square.
 */
export function getSquareMargins(square: Square, isFlipped = false): string {
	const squarePos = squareToSquarePos(square);
	return getSquarePosMargins(squarePos, isFlipped);
}
