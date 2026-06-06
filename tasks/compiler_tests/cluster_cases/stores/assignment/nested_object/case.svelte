<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ p: { a: 1 }, q: { b: 2 } });
	$: ({ p: { a }, q: { b } } = $s);
</script>
<button>{a}{b}</button>
