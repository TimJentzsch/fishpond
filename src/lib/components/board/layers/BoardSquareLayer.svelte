<script lang="ts">
	import {
		getSquareColor,
		getSquarePosMargins,
		getAxisPositions,
		getBoardPositions
	} from '$lib/board';

	export let isFlipped: boolean = false;

	const boardPositions = getBoardPositions();
</script>

<div class="boardBackground">
	{#each boardPositions as squarePos}
		<div
			class="square {getSquareColor(squarePos)}"
			style={getSquarePosMargins(squarePos, isFlipped)}
		/>
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

		transition: top var(--flip-transition-duration);
	}

	.square.w {
		background-color: var(--square-white);
	}

	.square.b {
		background-color: var(--square-black);
	}
</style>
