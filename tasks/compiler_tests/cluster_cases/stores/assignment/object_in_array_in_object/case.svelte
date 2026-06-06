<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ outer: [{ inner: 1 }] });
	$: ({ outer: [{ inner }] } = $s);
</script>
<button>{inner}</button>
