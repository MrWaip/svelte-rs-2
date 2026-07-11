<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const u = writable(1);
	const v = writable(2);
	let foo;
	let arr = [1, 2];
	function run() {
		foo = ([$u, $v] = arr);
	}
</script>
<button on:click={run}>{foo}{$u}{$v}</button>
