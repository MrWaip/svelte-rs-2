<script>
	import { writable } from "svelte/store";
	const source = writable(0);
	const held = $derived(source);
	function replace() {
		$held = writable(1);
	}
</script>

<p>{$held}</p>
<button onclick={replace}>replace</button>
