<script>
	import { writable } from "svelte/store";
	const count = writable(0);
</script>

<button onclick={() => $count++}>{$count}</button>
