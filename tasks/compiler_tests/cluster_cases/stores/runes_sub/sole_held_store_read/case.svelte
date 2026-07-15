<script>
	import { writable } from "svelte/store";
	const source = writable(0);
	const held = $derived(source);
	const doubled = $derived($held);
</script>

<p>{doubled}</p>
