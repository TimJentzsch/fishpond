<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import { getLastMoveSquares } from '$lib/indicators';
	import type { Chess } from 'chess.js';

	export let chess: Chess;
	export let isFlipped: boolean;

	$: lastMoveSquares = getLastMoveSquares(chess);
</script>

<div class="indicator-layer">
	{#each lastMoveSquares as lastMoveSquare (lastMoveSquare)}
		<div
			class="last-move {getSquareColor(lastMoveSquare)}"
			style={getSquareMargins(lastMoveSquare, isFlipped)}
		/>
	{/each}
</div>

<style>
	.indicator-layer {
		position: relative;

		width: 100%;
		height: 100%;
	}

	.last-move {
		position: absolute;
		width: var(--square-size);
		height: var(--square-size);
	}

	.last-move.w {
		background-color: var(--square-white-last-move-color);
	}

	.last-move.b {
		background-color: var(--square-black-last-move-color);
	}
</style>
