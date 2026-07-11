<script>
	import { writable } from "svelte/store";
	const source = writable(0);
	const doubled = $derived($source);
</script>

<p>{doubled}</p>
