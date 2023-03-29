<script lang="ts">
	import {
		getSquarePosColor,
		getSquarePosMargins,
		filePosToStr,
		rankPosToStr,
		getAxisPositions
	} from '$lib/board';

	export let isFlipped: boolean = false;

	const axisPositions = getAxisPositions();
</script>

<div class="board-number-container">
	{#each axisPositions as rank (rank)}
		<div
			class="square rank {getSquarePosColor({ rank, file: 7 })}"
			class:isFlipped
			style={getSquarePosMargins({ rank, file: 7 }, isFlipped)}
		>
			<div class="square-number">{rankPosToStr(rank)}</div>
		</div>
	{/each}
	{#each axisPositions as file (file)}
		<div
			class="square file {getSquarePosColor({ rank: 0, file })}"
			class:isFlipped
			style={getSquarePosMargins({ rank: 0, file }, false)}
		>
			<div class="square-number">{filePosToStr(file)}</div>
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
		user-select: none;
		width: var(--square-size);
		height: var(--square-size);

		transition: var(--square-pos-transition);
		transition: color var(--flip-transition-duration);
	}

	.square.rank {
		justify-content: end;
	}

	.square.file {
		align-items: end;
	}

	.square.w,
	.isFlipped.square.b.file {
		color: var(--square-black-color);
	}

	.square.b,
	.isFlipped.square.w.file {
		color: var(--square-white-color);
	}

	.square-number {
		padding: 0.2em;
	}
</style>
