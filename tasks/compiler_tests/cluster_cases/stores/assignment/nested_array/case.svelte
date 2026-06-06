<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable([[1, 2], [3, 4]]);
	$: [[a, b], [c, d]] = $s;
</script>
<button>{a}{b}{c}{d}</button>
