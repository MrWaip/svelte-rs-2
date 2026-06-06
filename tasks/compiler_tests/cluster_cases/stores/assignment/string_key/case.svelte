<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable({ "a-b": 1, "c d": 2 });
	$: ({ "a-b": ab, "c d": cd } = $s);
</script>
<button>{ab}{cd}</button>
