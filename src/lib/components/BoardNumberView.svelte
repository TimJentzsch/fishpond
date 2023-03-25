<script lang="ts">
	import { getSquareColor, indexToFile, indexToRank, squareIndexes } from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();
</script>

<div class="board-number-container">
	<div class="rank-number-container" class:flipped>
		{#each indexes as rankIndex (rankIndex)}
			<div class="square-number rank {getSquareColor(rankIndex, 7)}" class:flipped>
				{indexToRank(rankIndex)}
			</div>
		{/each}
	</div>
	<div class="file-number-container" class:flipped>
		{#each indexes as fileIndex (fileIndex)}
			<div class="square-number file {getSquareColor(0, fileIndex)}" class:flipped>
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
		flex-flow: column-reverse;
	}

	.flipped.rank-number-container {
		flex-flow: column;
	}

	.file-number-container {
		flex-flow: row;
	}

	.square-number {
		flex-grow: 1;
		padding: 0.2em;
	}

	.square-number.w,
	.flipped.square-number.b.file {
		color: var(--square-black);
	}

	.square-number.b,
	.flipped.square-number.w.file {
		color: var(--square-white);
	}
</style>
