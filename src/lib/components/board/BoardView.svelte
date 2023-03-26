<script lang="ts">
	import BoardBackgroundView from './layers/BoardSquareLayer.svelte';
	import BoardNumberView from './layers/BoardNumberLayer.svelte';

	export let flipped: boolean = false;
</script>

<div class="board">
	<div class="board-layer">
		<BoardBackgroundView {flipped} />
	</div>
	<div class="board-layer">
		<BoardNumberView {flipped} />
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
