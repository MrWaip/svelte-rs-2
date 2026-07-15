<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const a = writable(1);
	const obj = {};
	function run() {
		({ $a = 5 } = obj);
	}
</script>
<button on:click={run}>{$a}</button>
