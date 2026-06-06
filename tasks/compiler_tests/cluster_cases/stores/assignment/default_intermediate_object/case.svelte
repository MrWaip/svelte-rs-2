<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({});
	$: ({ p: { a } = {} } = $s);
</script>
<button>{a}</button>
