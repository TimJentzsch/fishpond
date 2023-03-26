<script>
	import { Chess } from 'chess.js';
	import BoardView from '../lib/components/board/BoardView.svelte';

	let chess = new Chess();
	let flipped = true;
</script>

<h1>Fishpond</h1>

<div class="board-container">
	<BoardView {chess} {flipped} />
	<label>
		<input type="checkbox" bind:checked={flipped} />
		Flip Board
	</label>
</div>

<style>
	.board-container {
		width: 500px;
		height: 500px;
	}
</style>
