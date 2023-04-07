<script lang="ts">
	import { PieceSet } from '$lib/piece';
	import { Chess } from 'chess.js';
	import BoardView from '../lib/components/board/BoardView.svelte';
	import { SoundSet } from '$lib/sound';
	import { invoke } from '@tauri-apps/api/tauri';

	let chess = new Chess();
	let flipped = false;
	let pieceSet: PieceSet = PieceSet.cburnett;
	let soundSet: SoundSet = SoundSet.fishpond;

	function runEngine() {
		invoke('run_engine');
	}
</script>

<h1>Fishpond</h1>

<div class="board-container">
	<BoardView {chess} isFlipped={flipped} {pieceSet} {soundSet} />

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
		<button on:click={runEngine}>Run engine</button>
	</form>
</div>

<style>
	.board-container {
		width: 500px;
		height: 500px;
	}
</style>
