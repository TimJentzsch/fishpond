<script lang="ts">
	export let color: 'white' | 'black';
</script>

<div class="square {color}" />

<style>
	.square {
		flex-grow: 1;
	}

	.square.white {
		background-color: var(--square-white);
	}

	.square.black {
		background-color: var(--square-black);
	}
</style>
