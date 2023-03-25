<script lang="ts">
	import { getSquareColor, squareIndexes } from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();

	function getSquareMargins(rank: number, file: number, flipped: boolean): string {
		const perIndex = 100 / 8;

		const left = file * perIndex;
		const top = flipped ? rank * perIndex : 100 - perIndex - rank * perIndex;

		return `top: ${top}%; left: ${left}%`;
	}
</script>

<div class="boardBackground">
	{#each indexes as rank}
		{#each indexes as file}
			<div
				class="square {getSquareColor(rank, file)}"
				style={getSquareMargins(rank, file, flipped)}
			/>
		{/each}
	{/each}
</div>

<style>
	.boardBackground {
		display: flex;
		flex-flow: column;
		position: relative;

		width: 100%;
		height: 100%;
	}
	.square {
		width: calc(100% / 8);
		height: calc(100% / 8);
		position: absolute;

		transition: top 0.25s;
	}

	.square.w {
		background-color: var(--square-white);
	}

	.square.b {
		background-color: var(--square-black);
	}
</style>
