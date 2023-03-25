<script lang="ts">
	import type { Color } from 'chess.js';

	export let color: Color;
</script>

<div class="square {color}" />

<style>
	.square {
		flex-grow: 1;
	}

	.square.w {
		background-color: var(--square-white);
	}

	.square.b {
		background-color: var(--square-black);
	}
</style>
