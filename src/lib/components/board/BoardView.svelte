<script lang="ts">
	import type { Chess } from 'chess.js';
	import BoardBackgroundLayer from './layers/BoardSquareLayer.svelte';
	import BoardNumberLayer from './layers/BoardNumberLayer.svelte';
	import BoardPieceLayer from './layers/BoardPieceLayer.svelte';

	export let chess: Chess;
	export let flipped: boolean = false;
</script>

<div class="board">
	<div class="board-layer">
		<BoardBackgroundLayer {flipped} />
	</div>
	<div class="board-layer">
		<BoardNumberLayer {flipped} />
	</div>
	<div class="board-layer">
		<BoardPieceLayer board={chess.board()} {flipped} />
	</div>
</div>

<style>
	.board {
		height: 100%;
		width: 100%;

		display: grid;
		place-items: center;
		grid-template-areas: 'inner-div';
	}

	.board-layer {
		grid-area: inner-div;
		width: 100%;
		height: 100%;
	}
</style>
