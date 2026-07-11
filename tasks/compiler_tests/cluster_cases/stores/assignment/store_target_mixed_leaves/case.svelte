<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable(1);
	let plain;
	function run() {
		[plain, $s] = [1, 2];
	}
</script>
<button on:click={run}>{plain}{$s}</button>
