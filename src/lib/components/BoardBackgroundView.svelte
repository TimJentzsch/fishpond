<script lang="ts">
	import SquareView from './SquareView.svelte';
	import { getSquareColor, squareIndexes } from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();
	const reversedIndexes = squareIndexes(true);

	$: rankIndexes = flipped ? indexes : reversedIndexes;
</script>

<div class="boardBackground">
	{#each rankIndexes as rank}
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
