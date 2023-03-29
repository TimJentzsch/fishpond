<script lang="ts">
	import {
		getSquarePosColor,
		getSquareMargins,
		getTargetSquare,
		type PieceInfo,
		getSquareColor
	} from '$lib/board';
	import type { Move, Square } from 'chess.js';
	import { createEventDispatcher } from 'svelte';
	import PieceView from './PieceView.svelte';

	const dispatch = createEventDispatcher();

	export let pieceInfo: PieceInfo;
	export let moves: Move[] = [];
	export let isFlipped: boolean = false;
	export let isSelected: boolean = false;

	$: margins = getSquareMargins(pieceInfo.square, isFlipped);
	$: piece = { color: pieceInfo.color, type: pieceInfo.type };

	function onClick() {
		dispatch('click', { pieceInfo });
	}
</script>

<button
	class="piece-square {getSquareColor(pieceInfo.square)}"
	class:selected={isSelected}
	style={margins}
	disabled={moves.length === 0}
	on:click={onClick}
>
	<PieceView {piece} />
</button>

<style>
	.piece-square {
		width: calc(100% / 8);
		height: calc(100% / 8);

		position: absolute;

		display: flex;
		justify-content: center;
		align-items: center;

		border: none;
		background: transparent;
		cursor: pointer;

		transition: var(--square-pos-transition);
	}

	.piece-square:disabled {
		color: inherit;
		cursor: inherit;
	}

	.piece-square.w.selected {
		background-color: var(--square-white-selected);
	}

	.piece-square.b.selected {
		background-color: var(--square-black-selected);
	}
</style>
