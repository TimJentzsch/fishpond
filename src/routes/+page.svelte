<script>
	import BoardView from '../lib/components/board/BoardView.svelte';

	let flipped = true;
</script>

<h1>Fishpond</h1>

<div class="board-container">
	<BoardView {flipped} />
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
