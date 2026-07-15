<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const a = writable(1);
	const b = writable(2);
	const obj = { p: [10], q: { inner: 20 } };
	function run() {
		({ p: [$a], q: { inner: $b } } = obj);
	}
</script>
<button on:click={run}>{$a}{$b}</button>
