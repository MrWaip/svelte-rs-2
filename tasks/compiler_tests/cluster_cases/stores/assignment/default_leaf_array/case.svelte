<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable([1]);
	$: [a = 10, b = 20] = $s;
</script>
<button>{a}{b}</button>
