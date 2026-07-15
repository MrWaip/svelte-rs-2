<script>
	import { writable } from "svelte/store";
	const source = writable({ bar: 0 });
	const held = $derived(source);
	function bump() {
		$held.bar = 6;
	}
</script>

<p>{$held.bar}</p>
<button onclick={bump}>bump</button>
