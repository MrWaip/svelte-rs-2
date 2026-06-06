<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ a: 1, b: 2 });
	$: ({ a, b } = $s);
</script>
<button>{a}{b}</button>
