<script>
	import { writable } from "svelte/store";
	const source = writable(0);
	const held = $derived(source);
	function read() {
		const current = $held;
		return current;
	}
</script>

<p>{read()}</p>
