<script lang="ts">
	import {
		getSquareColor,
		getSquareMargins,
		indexToFile,
		indexToRank,
		squareIndexes
	} from '$lib/board';

	export let flipped: boolean = false;

	const indexes = squareIndexes();
</script>

<div class="board-number-container">
	{#each indexes as rank (rank)}
		<div
			class="square rank {getSquareColor(rank, 7)}"
			class:flipped
			style={getSquareMargins(rank, 7, flipped)}
		>
			<div class="square-number">{indexToRank(rank)}</div>
		</div>
	{/each}
	{#each indexes as file (file)}
		<div
			class="square file {getSquareColor(0, file)}"
			class:flipped
			style={getSquareMargins(0, file, false)}
		>
			<div class="square-number">{indexToFile(file)}</div>
		</div>
	{/each}
</div>

<style>
	.board-number-container {
		width: 100%;
		height: 100%;

		position: relative;
	}

	.square {
		position: absolute;
		display: flex;
		width: calc(100% / 8);
		height: calc(100% / 8);

		transition: top var(--flip-transition-duration), color var(--flip-transition-duration);
	}

	.square.rank {
		justify-content: end;
	}

	.square.file {
		align-items: end;
	}

	.square.w,
	.flipped.square.b.file {
		color: var(--square-black);
	}

	.square.b,
	.flipped.square.w.file {
		color: var(--square-white);
	}

	.square-number {
		padding: 0.2em;
	}
</style>
