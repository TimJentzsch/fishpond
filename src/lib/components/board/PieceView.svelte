<script lang="ts">
	import type { PieceSet } from '$lib/piece';
	import { WHITE, type Piece } from 'chess.js';

	export let piece: Piece;
	export let isVisible: boolean = true;
	export let pieceSet: PieceSet;

	$: symbol = piece.color === WHITE ? piece.type.toUpperCase() : piece.type;
	$: src = `/piece/${pieceSet}/${piece.color}${piece.type.toUpperCase()}.svg`;
</script>

<img class="piece" {src} alt={symbol} hidden={!isVisible} draggable="false" />

<style>
	.piece {
		width: 100%;
		height: 100%;

		-webkit-user-drag: none;
		user-select: none;
		-moz-user-select: none;
		-webkit-user-select: none;
		-ms-user-select: none;
	}
</style>
