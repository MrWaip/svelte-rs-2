<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const s = writable([1, 2]);
	$: [globalThis.foo, a] = $s;
</script>
<button>{a}</button>
