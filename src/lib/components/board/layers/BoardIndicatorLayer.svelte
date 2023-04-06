<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import { getCheckInfo, getLastMoveSquares } from '$lib/indicators';
	import type { Chess } from 'chess.js';

	export let chess: Chess;
	export let isFlipped: boolean;

	$: lastMoveSquares = getLastMoveSquares(chess);
	$: checkInfo = getCheckInfo(chess);
</script>

<div class="indicator-layer">
	{#each lastMoveSquares as lastMoveSquare (lastMoveSquare)}
		<div
			class="last-move {getSquareColor(lastMoveSquare)}"
			style={getSquareMargins(lastMoveSquare, isFlipped)}
		/>
	{/each}

	{#if checkInfo !== undefined}
		<div
			class="check {getSquareColor(checkInfo.square)}"
			class:mate={checkInfo.isMate}
			style={getSquareMargins(checkInfo.square, isFlipped)}
		/>
	{/if}
</div>

<style>
	.indicator-layer {
		position: relative;

		width: 100%;
		height: 100%;
	}

	.last-move,
	.check {
		position: absolute;
		width: var(--square-size);
		height: var(--square-size);

		box-sizing: border-box;
	}

	.last-move.w {
		background-color: var(--square-white-last-move-color);
	}

	.last-move.b {
		background-color: var(--square-black-last-move-color);
	}

	.check.w {
		background: radial-gradient(
			ellipse at center,
			var(--square-white-check-color) 0%,
			var(--square-white-check-color) 25%,
			transparent 89%,
			transparent 100%
		);
	}

	.check.b {
		background: radial-gradient(
			ellipse at center,
			var(--square-black-check-color) 0%,
			var(--square-black-check-color) 25%,
			transparent 89%,
			transparent 100%
		);
	}

	.check.mate.w {
		background: var(--square-white-check-color);
	}

	.check.mate.b {
		background: var(--square-black-check-color);
	}
</style>
