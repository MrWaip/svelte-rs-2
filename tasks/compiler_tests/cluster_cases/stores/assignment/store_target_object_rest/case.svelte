<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const a = writable(1);
	let rest;
	const obj = { $a: 1, c: 2, d: 3 };
	function run() {
		({ $a, ...rest } = obj);
	}
</script>
<button on:click={run}>{$a}{rest.c}</button>
