<script lang="ts">
	import type { Chess, Move } from 'chess.js';
	import BoardBackgroundLayer from './layers/BoardSquareLayer.svelte';
	import BoardNumberLayer from './layers/BoardNumberLayer.svelte';
	import BoardPieceLayer from './layers/BoardPieceLayer.svelte';

	export let chess: Chess;
	export let isFlipped: boolean = false;

	function onMove(event: CustomEvent<{ move: Move }>) {
		chess.move(event.detail.move);
		chess = chess;
	}
</script>

<div class="board">
	<div class="board-layer">
		<BoardBackgroundLayer {isFlipped} />
	</div>
	<div class="board-layer">
		<BoardNumberLayer {isFlipped} />
	</div>
	<div class="board-layer">
		<BoardPieceLayer {chess} {isFlipped} on:move={onMove} />
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
