<script lang="ts">
	import type { Piece } from 'chess.js';

	export let piece: Piece;
</script>

<div class="piece-container">
	<div class="piece">
		{piece.type}
	</div>
</div>

<style>
	.piece-container {
		display: flex;
		flex-grow: 1;
		justify-content: center;
		align-items: center;
	}
</style>
