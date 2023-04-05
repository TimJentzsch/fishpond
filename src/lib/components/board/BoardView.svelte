<script lang="ts">
	import type { Chess, Move } from 'chess.js';
	import BoardBackgroundLayer from './layers/BoardSquareLayer.svelte';
	import BoardNumberLayer from './layers/BoardNumberLayer.svelte';
	import BoardPieceLayer from './layers/BoardPieceLayer.svelte';
	import type { PieceSet } from '$lib/piece';
	import type { SoundSet } from '$lib/sound';
	import BoardIndicatorLayer from './layers/BoardIndicatorLayer.svelte';

	export let chess: Chess;
	export let isFlipped: boolean = false;
	export let pieceSet: PieceSet;
	export let soundSet: SoundSet;

	$: moveSound = new Audio(`/sound/${soundSet}/Move.ogg`);
	$: captureSound = new Audio(`/sound/${soundSet}/Capture.ogg`);

	function playMoveSound(move: Move) {
		if (move.captured !== undefined) {
			captureSound.play();
		} else {
			moveSound.play();
		}
	}

	function onMove(event: CustomEvent<{ move: Move }>) {
		const move = event.detail.move;

		chess.move(move);
		chess = chess;

		playMoveSound(move);
	}
</script>

<div class="board">
	<div class="board-layer">
		<BoardBackgroundLayer {isFlipped} />
	</div>
	<div class="board-layer">
		<BoardIndicatorLayer {chess} {isFlipped} />
	</div>
	<div class="board-layer">
		<BoardNumberLayer {isFlipped} />
	</div>
	<div class="board-layer">
		<BoardPieceLayer {chess} {isFlipped} {pieceSet} on:move={onMove} />
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
