<script lang="ts">
	import { PieceSet } from '$lib/piece';
	import { Chess } from 'chess.js';
	import BoardView from '../lib/components/board/BoardView.svelte';

	let chess = new Chess();
	let flipped = false;
	let pieceSet: PieceSet = PieceSet.cburnett;
</script>

<h1>Fishpond</h1>

<div class="board-container">
	<BoardView {chess} isFlipped={flipped} {pieceSet} />

	<form>
		<label>
			<input type="checkbox" bind:checked={flipped} />
			Flip Board
		</label>
		<br />
		<label>
			Piece set
			<select name="piece-set" bind:value={pieceSet}>
				{#each Object.values(PieceSet) as set}
					<option value={set}>{set}</option>
				{/each}
			</select>
		</label>
	</form>
</div>

<style>
	.board-container {
		width: 500px;
		height: 500px;
	}
</style>
