<script lang="ts">
	import type { PieceInfo } from '$lib/board';
	import type { Chess } from 'chess.js';
	import PieceSquareView from '../PieceSquareView.svelte';

	export let chess: Chess;
	export let isFlipped: boolean = false;

	$: board = chess.board();

	let selectedPieceInfo: PieceInfo | null = null;

	function onPieceSquareClick(event: CustomEvent) {
		const pieceInfo: PieceInfo = event.detail.pieceInfo;

		if (
			pieceInfo.square === selectedPieceInfo?.square &&
			pieceInfo.type === selectedPieceInfo?.type &&
			pieceInfo.color === selectedPieceInfo?.color
		) {
			selectedPieceInfo = null;
		} else {
			selectedPieceInfo = pieceInfo;
		}
	}

	function onPieceDragStart(event: CustomEvent) {
		const pieceInfo: PieceInfo = event.detail.pieceInfo;
		selectedPieceInfo = pieceInfo;
	}
</script>

<div class="piece-layer">
	{#each board as rank}
		{#each rank as pieceInfo}
			{#if pieceInfo !== null}
				<PieceSquareView
					{pieceInfo}
					{isFlipped}
					isSelected={selectedPieceInfo?.square === pieceInfo.square}
					moves={chess.moves({ verbose: true, ...pieceInfo })}
					on:pieceClick={onPieceSquareClick}
					on:pieceDragStart={onPieceDragStart}
					on:move
				/>
			{/if}
		{/each}
	{/each}
</div>

<style>
	.piece-layer {
		width: 100%;
		height: 100%;

		position: relative;
	}
</style>
