<script lang="ts">
	import { getSquareMargins, type Board } from '$lib/board';
	import PieceView from '../PieceView.svelte';

	export let board: Board;
	export let flipped: boolean = false;
</script>

<div class="piece-layer">
	{#each board as rank}
		{#each rank as pieceInfo}
			{#if pieceInfo !== null}
				<div class="piece-container" style={getSquareMargins(pieceInfo.square, flipped)}>
					<PieceView piece={{ color: pieceInfo.color, type: pieceInfo.type }} />
				</div>
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

	.piece-container {
		width: calc(100% / 8);
		height: calc(100% / 8);

		position: absolute;

		display: flex;
		justify-content: center;
		align-items: center;

		transition: top var(--flip-transition-duration), left var(--flip-transition-duration);
	}
</style>
