<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import type { Move } from 'chess.js';
	import { createEventDispatcher } from 'svelte';

	export let move: Move;
	export let isFlipped: boolean = false;

	const dispatch = createEventDispatcher();

	$: margins = getSquareMargins(move.to, isFlipped);

	function onClick() {
		dispatch('move', { move });
	}
</script>

<button class="move-target {getSquareColor(move.to)}" style={margins} on:click={onClick}>
	<div class="move-indicator" />
</button>

<style>
	.move-target {
		width: var(--square-size);
		height: var(--square-size);

		position: absolute;
		cursor: pointer;

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
