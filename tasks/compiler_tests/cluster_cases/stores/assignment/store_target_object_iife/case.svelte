<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const x = writable(1);
	const y = writable(2);
	function run() {
		({ a: $x, b: $y } = { a: 9, b: 10 });
	}
</script>
<button on:click={run}>{$x}{$y}</button>
