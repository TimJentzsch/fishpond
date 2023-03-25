<script lang="ts">
	import { getSquareColor, indexToFile, indexToRank, squareIndexes } from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();
	const reversedIndexes = squareIndexes(true);

	$: rankIndexes = flipped ? indexes : reversedIndexes;
</script>

<div class="board-number-container">
	<div class="rank-number-container">
		{#each rankIndexes as rankIndex}
			<div class="square-number rank {getSquareColor(rankIndex, 7)}">
				{indexToRank(rankIndex)}
			</div>
		{/each}
	</div>
	<div class="file-number-container">
		{#each indexes as fileIndex}
			<div class="square-number file {getSquareColor(0, fileIndex)}">
				{indexToFile(fileIndex)}
			</div>
		{/each}
	</div>
</div>

<style>
	.board-number-container {
		width: 100%;
		height: 100%;

		display: grid;
		place-items: center;
		grid-template-areas: 'inner-div';
	}

	.rank-number-container,
	.file-number-container {
		width: 100%;
		height: 100%;
		grid-area: inner-div;

		display: flex;
	}

	.rank-number-container,
	.file-number-container {
		align-items: end;
	}

	.rank-number-container {
		flex-flow: column;
	}

	.file-number-container {
		flex-flow: row;
	}

	.square-number {
		flex-grow: 1;
		padding: 0.2em;
	}

	.square-number.w {
		color: var(--square-black);
	}

	.square-number.b {
		color: var(--square-white);
	}
</style>
