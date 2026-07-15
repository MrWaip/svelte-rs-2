<script>
	import { writable } from "svelte/store";
	const source = writable({ foo: 0 });
	const held = $derived(source);
	const value = $derived($held.foo);
</script>

<p>{value}</p>
