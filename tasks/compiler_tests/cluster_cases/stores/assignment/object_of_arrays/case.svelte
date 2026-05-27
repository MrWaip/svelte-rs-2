<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ p: [1, 2], q: [3, 4] });
	$: ({ p: [a, b], q: [c, d] } = $s);
</script>
<button>{a}{b}{c}{d}</button>
