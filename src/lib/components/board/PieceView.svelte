<script lang="ts">
	import { WHITE, type Piece } from 'chess.js';

	export let piece: Piece;
	export let isVisible: boolean = true;

	$: symbol = piece.color === WHITE ? piece.type.toUpperCase() : piece.type;
</script>

<div class="piece" hidden={!isVisible}>
	{symbol}
</div>

<style>
	.piece {
		font-size: 2em;
		font-weight: bold;
		user-select: none;
	}
</style>
