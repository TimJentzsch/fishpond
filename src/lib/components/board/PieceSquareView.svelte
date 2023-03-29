<script lang="ts">
	import { getSquareMargins, type PieceInfo } from '$lib/board';
	import { createEventDispatcher } from 'svelte';
	import PieceView from './PieceView.svelte';

	const dispatch = createEventDispatcher();

	export let pieceInfo: PieceInfo;
	export let isFlipped: boolean = false;
	export let isSelected: boolean = false;

	$: margins = getSquareMargins(pieceInfo.square, isFlipped);
	$: piece = { color: pieceInfo.color, type: pieceInfo.type };

	function onClick() {
		dispatch('click', { pieceInfo });
	}
</script>

<button class="piece-square" class:selected={isSelected} style={margins} on:click={onClick}>
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

		transition: top var(--flip-transition-duration), left var(--flip-transition-duration);
	}

	.piece-square:disabled {
		color: inherit;
		cursor: inherit;
	}

	.piece-square.selected {
		background-color: green;
	}
</style>
