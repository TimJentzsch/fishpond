<script lang="ts">
	import { WHITE, type Piece } from 'chess.js';

	export let piece: Piece;
	export let isVisible: boolean = true;
	export let theme: string = 'cburnett';

	$: symbol = piece.color === WHITE ? piece.type.toUpperCase() : piece.type;
	$: src = `/piece/${theme}/${piece.color}${piece.type.toUpperCase()}.svg`;
</script>

<img class="piece" {src} alt={symbol} hidden={!isVisible} />

<style>
	.piece {
		width: 100%;
		height: 100%;
		user-select: none;
	}
</style>
