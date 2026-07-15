<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const u = writable(1);
	const v = writable(2);
	const w = writable(3);
	function run() {
		[$u, $v, $w] = [7, 8, 9];
	}
</script>
<button on:click={run}>{$u}{$v}{$w}</button>
