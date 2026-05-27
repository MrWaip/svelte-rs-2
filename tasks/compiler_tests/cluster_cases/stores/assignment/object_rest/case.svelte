<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ a: 1, b: 2, c: 3 });
	$: ({ a, ...rest } = $s);
</script>
<button>{a}{rest.b}</button>
