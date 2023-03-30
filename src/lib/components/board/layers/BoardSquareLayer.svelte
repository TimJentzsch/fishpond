<script lang="ts">
	import { getSquarePosColor, getSquarePosMargins, getBoardPositions } from '$lib/board';

	export let isFlipped: boolean = false;

	const boardPositions = getBoardPositions();
</script>

<div class="boardBackground">
	{#each boardPositions as squarePos}
		<div
			class="square {getSquarePosColor(squarePos)}"
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
		width: var(--square-size);
		height: var(--square-size);
		position: absolute;

		transition: var(--square-pos-transition);
	}

	.square.w {
		background-color: var(--square-white-color);
	}

	.square.b {
		background-color: var(--square-black-color);
	}
</style>
