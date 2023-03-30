<script lang="ts">
	import { getSquareMargins, type PieceInfo, getSquareColor } from '$lib/board';
	import type { PieceSet } from '$lib/piece';
	import type { Move } from 'chess.js';
	import { createEventDispatcher } from 'svelte';
	import MoveTarget from './MoveTarget.svelte';
	import PieceView from './PieceView.svelte';

	const dispatch = createEventDispatcher();

	export let pieceInfo: PieceInfo;
	export let moves: Move[] = [];
	export let isFlipped: boolean = false;
	export let isSelected: boolean = false;
	export let pieceSet: PieceSet;

	let isDragging = false;

	$: margins = getSquareMargins(pieceInfo.square, isFlipped);
	$: piece = { color: pieceInfo.color, type: pieceInfo.type };
	$: disabled = moves.length === 0;

	function onClick() {
		dispatch('pieceClick', { pieceInfo });
	}

	function onDragStart() {
		dispatch('pieceDragStart', { pieceInfo });
	}

	function onDrag() {
		isDragging = true;
	}

	function onDragEnd() {
		isDragging = false;
	}
</script>

{#if isSelected}
	<div class="piece-selected {getSquareColor(pieceInfo.square)}" style={margins} />
{/if}

<button
	class="piece-square"
	class:selected={isSelected}
	style={margins}
	{disabled}
	draggable={!disabled}
	on:click={onClick}
	on:dragstart={onDragStart}
	on:dragend={onDragEnd}
	on:drag={onDrag}
>
	<div class="piece-square-inner">
		<PieceView {piece} {pieceSet} isVisible={!isDragging} />
	</div>
</button>

{#if isSelected}
	{#each moves as move}
		<MoveTarget {move} {isFlipped} on:move />
	{/each}
{/if}

<style>
	.piece-square {
		width: var(--square-size);
		height: var(--square-size);

		position: absolute;

		border: none;
		background: transparent;
		padding: 0;
		margin: 0;

		cursor: pointer;

		transition: var(--square-pos-transition);
	}

	.piece-square:disabled {
		color: inherit;
		cursor: inherit;
	}

	.piece-square-inner {
		height: 100%;
		width: 100%;

		display: flex;
		justify-content: center;
		align-items: center;
	}

	.piece-selected {
		width: var(--square-size);
		height: var(--square-size);

		position: absolute;
	}

	.piece-selected.w {
		background-color: var(--square-white-selected-color);
	}

	.piece-selected.b {
		background-color: var(--square-black-selected-color);
	}
</style>
