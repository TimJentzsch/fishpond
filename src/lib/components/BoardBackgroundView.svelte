<script lang="ts">
	import SquareView from './SquareView.svelte';
	import type { Color } from 'chess.js';

	const indexes = [...Array(8).keys()];

	function getSquareColor(rank: number, file: number): Color {
		return (rank + file) % 2 == 0 ? 'w' : 'b';
	}
</script>

<div class="boardBackground">
	{#each indexes as rank}
		<div class="rank">
			{#each indexes as file}
				<SquareView color={getSquareColor(rank, file)} />
			{/each}
		</div>
	{/each}
</div>

<style>
	.boardBackground {
		display: flex;
		flex-flow: column;

		width: 100%;
		height: 100%;
	}

	.rank {
		display: flex;
		flex-flow: row;
		flex-grow: 1;
	}
</style>
