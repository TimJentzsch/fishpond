<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import type { Chess, Square } from 'chess.js';

	export let chess: Chess;
	export let isFlipped: boolean;

	$: lastMoveSquares = getLastMoveSquares(chess);

	function getLastMoveSquares(myChess: Chess): Square[] {
		console.debug('Getting last move squares');
		const moveHistory = myChess.history({ verbose: true });

		if (moveHistory.length === 0) {
			return [];
		}

		const lastMove = moveHistory[moveHistory.length - 1];
		return [lastMove.from, lastMove.to];
	}
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
