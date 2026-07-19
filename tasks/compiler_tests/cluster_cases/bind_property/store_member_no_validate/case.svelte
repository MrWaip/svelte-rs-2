<script>
	import { writable } from "svelte/store";

	const obj = writable({ a: 1 });
</script>

<input type="number" bind:value={$obj.a}>
