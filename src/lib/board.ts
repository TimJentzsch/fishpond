import type { Color, PieceSymbol, Square } from 'chess.js';
export type Rank = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8';
export type File = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h';

export type PieceInfo = {
	square: Square;
	type: PieceSymbol;
	color: Color;
};
export type Board = (PieceInfo | null)[][];

export function squareIndexes(reversed = false): number[] {
	const indexes = [0, 1, 2, 3, 4, 5, 6, 7];

	if (reversed) {
		indexes.reverse();
	}

	return indexes;
}

export function getSquareColor(rank: number, file: number): Color {
	return (rank + file) % 2 == 0 ? 'b' : 'w';
}

export function indexToRank(index: number): Rank {
	switch (index) {
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
			throw Error(`Invalid rank index ${index}`);
	}
}

export function indexToFile(index: number): File {
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

export function getSquareIndexMargins(rank: number, file: number, flipped: boolean): string {
	const perIndex = 100 / 8;

	const left = file * perIndex;
	const top = flipped ? rank * perIndex : 100 - perIndex - rank * perIndex;

	return `top: ${top}%; left: ${left}%`;
}

export function squareToIndexes(square: Square): { file: number; rank: number } {
	const file = square.charCodeAt(0) - 'a'.charCodeAt(0);
	const rank = square.charCodeAt(1) - '1'.charCodeAt(0);

	return {
		file,
		rank
	};
}

export function getSquareMargins(square: Square, flipped = false): string {
	const indexes = squareToIndexes(square);
	return getSquareIndexMargins(indexes.rank, indexes.file, flipped);
}
