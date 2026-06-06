<svelte:options runes={false} />
<script>
	import { writable } from "svelte/store";
	const k = "z";
	const s = writable({ z: 1 });
	$: ({ [k]: v } = $s);
</script>
<button>{v}</button>
