<script>
	import { writable } from 'svelte/store';
	const handler = writable();
</script>

<button on:click={$handler}>x</button>
