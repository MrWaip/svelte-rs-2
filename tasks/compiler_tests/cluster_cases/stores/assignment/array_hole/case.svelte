<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable([1, 2, 3]);
	$: [a, , c] = $s;
</script>
<button>{a}{c}</button>
