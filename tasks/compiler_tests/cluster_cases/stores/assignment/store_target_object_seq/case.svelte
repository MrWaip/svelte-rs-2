<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const a = writable(1);
	const b = writable(2);
	const obj = { $a: 10, $b: 20 };
	function run() {
		({ $a, $b } = obj);
	}
</script>
<button on:click={run}>{$a}{$b}</button>
