<script lang="ts">
	import { getSquareColor, getSquareMargins } from '$lib/board';
	import type { Move } from 'chess.js';
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	export let move: Move;
	export let isFlipped: boolean = false;

	$: margins = getSquareMargins(move.to, isFlipped);

	let isDraggedOver = false;

	function onClick() {
		dispatch('move', { move });
	}

	function onDragOver(event: DragEvent) {
		event.preventDefault();

		const dataTransfer = event.dataTransfer;

		if (dataTransfer !== null) {
			dataTransfer.dropEffect = 'move';
		}
	}

	function onDragEnter() {
		isDraggedOver = true;
	}

	function onDragLeave() {
		isDraggedOver = false;
	}

	function onDrop() {
		dispatch('move', { move });
	}
</script>

<button
	class="move-target {getSquareColor(move.to)}"
	style={margins}
	on:click={onClick}
	on:dragover={onDragOver}
	on:drop={onDrop}
	on:dragenter={onDragEnter}
	on:dragleave={onDragLeave}
>
	<div class="move-indicator" class:isDraggedOver />
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
		padding: 0;
		margin: 0;

		/* The move target must be above the pieces to be clickable reliably. */
		z-index: 2;
	}

	.move-indicator {
		width: var(--move-indicator-size);
		height: var(--move-indicator-size);
		border-radius: 50%;

		background-color: var(--move-indicator-color);

		/* Prevent the move indicator from obstructing the drop target. */
		z-index: -1;
	}

	.move-indicator.isDraggedOver,
	.move-target:hover > .move-indicator {
		width: 100%;
		height: 100%;
		border-radius: 0;
	}
</style>
