<script>
	import { writable, derived } from "svelte/store";
	const store = writable(0);
	let doubled = $derived($store * 2);
</script>

<p>{doubled} {$store}</p>
