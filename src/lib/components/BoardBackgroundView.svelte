<script lang="ts">
	import SquareView from './SquareView.svelte';
	import { getSquareColor, squareIndexes } from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();
</script>

<div class="boardBackground">
	{#each indexes as rank}
		<div class="rank" class:flipped>
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
		flex-flow: row-reverse;
		flex-grow: 1;
	}

	.rank.flipped {
		flex-flow: row;
	}
</style>
