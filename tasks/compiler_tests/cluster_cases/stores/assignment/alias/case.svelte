<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ a: 1, b: 2 });
	$: ({ a: x, b: y } = $s);
</script>
<button>{x}{y}</button>
