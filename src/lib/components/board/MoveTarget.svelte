<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import type { Move } from 'chess.js';

	export let move: Move;
	export let isFlipped: boolean = false;

	$: margins = getSquareMargins(move.to, isFlipped);
</script>

<button class="move-target {getSquareColor(move.to)}" style={margins}>
	<div class="move-indicator" />
</button>

<style>
	.move-target {
		width: var(--square-size);
		height: var(--square-size);

		position: absolute;

		display: flex;
		justify-content: center;
		align-items: center;

		border: none;
		background: transparent;
	}

	.move-indicator {
		width: var(--move-indicator-size);
		height: var(--move-indicator-size);
		border-radius: 50%;

		background-color: var(--move-indicator-color);
	}
</style>
