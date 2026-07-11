<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const x = writable(1);
	const k = "a";
	const obj = { a: 42 };
	function run() {
		({ [k]: $x } = obj);
	}
</script>
<button on:click={run}>{$x}</button>
