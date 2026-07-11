<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable(1);
	function run() {
		$s = 5;
	}
</script>
<button on:click={run}>{$s}</button>
