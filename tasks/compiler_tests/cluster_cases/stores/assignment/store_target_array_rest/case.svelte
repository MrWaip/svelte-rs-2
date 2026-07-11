<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const a = writable(1);
	let rest;
	let arr = [1, 2, 3];
	function run() {
		[$a, ...rest] = arr;
	}
</script>
<button on:click={run}>{$a}{rest.length}</button>
