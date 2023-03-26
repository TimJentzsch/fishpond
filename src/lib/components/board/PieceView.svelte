<script lang="ts">
	import { WHITE, type Piece } from 'chess.js';

	export let piece: Piece;

	$: symbol = piece.color === WHITE ? piece.type.toUpperCase() : piece.type;
</script>

<div class="piece">
	{symbol}
</div>

<style>
	.piece {
		font-size: 2em;
		font-weight: bold;
	}
</style>
